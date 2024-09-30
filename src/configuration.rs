use crate::constants::{CHECK_C_TYPES_EXEC_PATH, CHECK_C_TYPES_PATH, C_TYPES_JSON_PATH};
use crate::cpu_features::CpuFeatures;
use crate::error::{exit_if_err, Errors};
use crate::typecheckcache::{self, TypeCheckCache};
use crate::{error::error_exit, DEFAULT_COMPILATION_UNIT_MAX_SIZE};
use crate::{
    to_absolute_path, C_CHAR_NAME, C_DOUBLE_NAME, C_FLOAT_NAME, C_INT_NAME, C_LONG_LONG_NAME,
    C_LONG_NAME, C_SHORT_NAME, C_SIZE_T_NAME, C_UNSIGNED_CHAR_NAME, C_UNSIGNED_INT_NAME,
    C_UNSIGNED_LONG_LONG_NAME, C_UNSIGNED_LONG_NAME, C_UNSIGNED_SHORT_NAME,
    OPTIMIZATION_LEVEL_DEFAULT, OPTIMIZATION_LEVEL_MINIMUM, OPTIMIZATION_LEVEL_NONE,
    OPTIMIZATION_LEVEL_SEPARATED,
};
use build_time::build_time_utc;
use inkwell::module::Linkage;
use inkwell::OptimizationLevel;
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::sync::Arc;
use std::{env, path::PathBuf};

#[derive(Clone, Copy)]
pub enum LinkType {
    Static,
    Dynamic,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ValgrindTool {
    None,
    MemCheck,
    // Currently, we cannot use DRD or helgrind because valgrind does not understand atomic operations.
    // In C/C++ program, we can use `ANNOTATE_HAPPENS_BEFORE` and `ANNOTATE_HAPPENS_AFTER` to tell helgrind happens-before relations,
    // but how can we do similar things in Fix?
    // DataRaceDetection,
}

// Subcommands of the `fix` command.
#[derive(Clone)]
pub enum SubCommand {
    Build,
    Run,
    Test,
    Diagnostics(DiagnosticsConfig),
    Docs,
}

impl SubCommand {
    // Should we run preliminary commands before building the program?
    pub fn run_preliminary_commands(&self) -> bool {
        match self {
            SubCommand::Build => true,
            SubCommand::Run => true,
            SubCommand::Test => true,
            SubCommand::Diagnostics(_) => false,
            SubCommand::Docs => false,
        }
    }

    // Should we build program binary?
    pub fn build_binary(&self) -> bool {
        match self {
            SubCommand::Build => true,
            SubCommand::Run => true,
            SubCommand::Test => true,
            SubCommand::Diagnostics(_) => false,
            SubCommand::Docs => false,
        }
    }

    // Should we use test files?
    pub fn use_test_files(&self) -> bool {
        match self {
            SubCommand::Build => false,
            SubCommand::Run => false,
            SubCommand::Test => true,
            SubCommand::Diagnostics(_) => true,
            SubCommand::Docs => true,
        }
    }

    // Should we typecheck the program?
    pub fn typecheck(&self) -> bool {
        match self {
            SubCommand::Build => true,
            SubCommand::Run => true,
            SubCommand::Test => true,
            SubCommand::Diagnostics(_) => true,
            SubCommand::Docs => false,
        }
    }
}

// Configuration for diagnostics subcommand.
#[derive(Clone, Default)]
pub struct DiagnosticsConfig {
    // Target source files.
    pub files: Vec<PathBuf>,
}

#[derive(Clone)]
pub struct Configuration {
    // Source files.
    pub source_files: Vec<PathBuf>,
    // Object files to be linked.
    pub object_files: Vec<PathBuf>,
    // Runs memory sanitizer to detect memory leak and invalid memory reference at early time.
    // Requires shared library ,/sanitizer/libfixsanitizer.so.
    pub sanitize_memory: bool,
    // Fix's optimization level.
    pub fix_opt_level: FixOptimizationLevel,
    // Linked libraries
    pub linked_libraries: Vec<(String, LinkType)>,
    // Library search paths.
    pub library_search_paths: Vec<PathBuf>,
    // Create debug info.
    pub debug_info: bool,
    // Is emit llvm?
    pub emit_llvm: bool,
    // Output file name.
    pub out_file_path: Option<PathBuf>,
    // Use threads.
    // To turn on this true and link pthread library, use `set_threaded` function.
    pub threaded: bool,
    // Macros defined in runtime.c.
    pub runtime_c_macro: Vec<String>,
    // Show times for each build steps.
    pub show_build_times: bool,
    // Verbose mode.
    pub verbose: bool,
    // Maximum size of compilation unit.
    pub max_cu_size: usize,
    // Run program with valgrind. Effective only in `run` mode.
    pub valgrind_tool: ValgrindTool,
    // Sizes of C types.
    pub c_type_sizes: CTypeSizes,
    // Subcommand of the `fix` command.
    pub subcommand: SubCommand,
    // Extra build commands.
    pub extra_commands: Vec<ExtraCommand>,
    // Type chech cache.
    pub type_check_cache: Arc<dyn TypeCheckCache + Send + Sync>,
}

#[derive(Clone)]
pub struct ExtraCommand {
    pub work_dir: PathBuf,
    pub command: Vec<String>,
}

impl ExtraCommand {
    pub fn run(&self) -> Result<(), Errors> {
        let mut com = Command::new(&self.command[0]);
        for arg in &self.command[1..] {
            com.arg(arg);
        }
        let work_dir = to_absolute_path(&self.work_dir);
        com.current_dir(&work_dir);
        let status = com.status().map_err(|e| {
            Errors::from_msg(format!(
                "Failed to run command \"{}\": {:?}",
                self.command.join(" "),
                e
            ))
        })?;
        if !status.success() {
            return Err(Errors::from_msg(format!(
                "Command \"{}\" failed with exit code {}.",
                self.command.join(" "),
                status.code().unwrap_or(-1)
            )));
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum FixOptimizationLevel {
    None,      // For debugging; skip even tail call optimization.
    Minimum,   // For fast compilation.
    Separated, // Perform almost all of the optimizations except for LLVM-level LTO.
    Default,   // For fast execution.
}

impl std::fmt::Display for FixOptimizationLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FixOptimizationLevel::None => write!(f, "{}", OPTIMIZATION_LEVEL_NONE),
            FixOptimizationLevel::Minimum => write!(f, "{}", OPTIMIZATION_LEVEL_MINIMUM),
            FixOptimizationLevel::Separated => write!(f, "{}", OPTIMIZATION_LEVEL_SEPARATED),
            FixOptimizationLevel::Default => write!(f, "{}", OPTIMIZATION_LEVEL_DEFAULT),
        }
    }
}

impl FixOptimizationLevel {
    pub fn from_str(opt_level: &str) -> Option<Self> {
        match opt_level {
            OPTIMIZATION_LEVEL_NONE => Some(FixOptimizationLevel::None),
            OPTIMIZATION_LEVEL_MINIMUM => Some(FixOptimizationLevel::Minimum),
            OPTIMIZATION_LEVEL_SEPARATED => Some(FixOptimizationLevel::Separated),
            OPTIMIZATION_LEVEL_DEFAULT => Some(FixOptimizationLevel::Default),
            _ => None,
        }
    }
}

impl Configuration {
    pub fn new(subcommand: SubCommand) -> Result<Self, Errors> {
        Ok(Configuration {
            subcommand,
            source_files: vec![],
            object_files: vec![],
            sanitize_memory: false,
            fix_opt_level: FixOptimizationLevel::Default, // Fix's optimization level.
            linked_libraries: vec![],
            debug_info: false,
            emit_llvm: false,
            out_file_path: None,
            threaded: false,
            runtime_c_macro: vec![],
            show_build_times: false,
            verbose: false,
            max_cu_size: DEFAULT_COMPILATION_UNIT_MAX_SIZE,
            valgrind_tool: ValgrindTool::None,
            library_search_paths: vec![],
            c_type_sizes: CTypeSizes::load_or_check()?,
            extra_commands: vec![],
            type_check_cache: Arc::new(typecheckcache::FileCache::new()),
        })
    }
}

impl Configuration {
    // Configuration for release build.
    pub fn release_mode(subcommand: SubCommand) -> Configuration {
        exit_if_err(Self::new(subcommand))
    }

    // Usual configuration for compiler development
    #[allow(dead_code)]
    pub fn develop_compiler_mode() -> Configuration {
        #[allow(unused_mut)]
        let mut config = exit_if_err(Self::new(SubCommand::Run));
        config.set_valgrind(ValgrindTool::MemCheck);
        // config.fix_opt_level = FixOptimizationLevel::Separated;
        // config.set_sanitize_memory();
        // config.emit_llvm = true;
        // config.debug_info = true;
        config
    }

    pub fn set_valgrind(&mut self, tool: ValgrindTool) -> &mut Configuration {
        self.valgrind_tool = tool;
        self
    }

    // Add dynamically linked library.
    // To link libabc.so, provide library name "abc".
    pub fn add_dyanmic_library(&mut self, name: &str) {
        self.linked_libraries
            .push((name.to_string(), LinkType::Dynamic));
    }

    pub fn get_output_llvm_ir_path(&self, optimized: bool, unit_name: &str) -> PathBuf {
        match &self.out_file_path {
            None => {
                if optimized {
                    return PathBuf::from(format!("{}_optimized.ll", unit_name));
                } else {
                    return PathBuf::from(format!("{}.ll", unit_name));
                }
            }
            Some(out_file_path) => {
                let file_name = out_file_path.file_name();
                if file_name.is_none() {
                    error_exit(&format!(
                        "Invalid output file path: `{}`",
                        out_file_path.to_str().unwrap()
                    ))
                } else {
                    let file_name = file_name.unwrap().to_str().unwrap();
                    let file_name = file_name.to_string()
                        + "_"
                        + unit_name
                        + if optimized { "_optimized.ll" } else { ".ll" };
                    let mut out_file_path = out_file_path.clone();
                    out_file_path.set_file_name(file_name);
                    out_file_path
                }
            }
        }
    }

    pub fn get_output_executable_file_path(&self) -> PathBuf {
        match &self.out_file_path {
            None => PathBuf::from(if env::consts::OS != "windows" {
                "a.out"
            } else {
                "a.exe"
            }),
            Some(out_file_path) => out_file_path.clone(),
        }
    }

    // Set threaded = true, and add ptherad library to linked_libraries.
    pub fn set_threaded(&mut self) {
        self.threaded = true;
        self.add_dyanmic_library("pthread");
    }

    #[allow(dead_code)]
    pub fn set_sanitize_memory(&mut self) {
        self.sanitize_memory = true;
    }

    pub fn set_debug_info(&mut self) {
        self.debug_info = true;
        self.set_fix_opt_level(FixOptimizationLevel::None);
    }

    pub fn set_fix_opt_level(&mut self, level: FixOptimizationLevel) {
        self.fix_opt_level = level;
    }

    pub fn get_llvm_opt_level(&self) -> OptimizationLevel {
        match self.fix_opt_level {
            FixOptimizationLevel::None => OptimizationLevel::None,
            FixOptimizationLevel::Minimum => OptimizationLevel::Less,
            FixOptimizationLevel::Separated => OptimizationLevel::Default,
            FixOptimizationLevel::Default => OptimizationLevel::Default,
        }
    }

    pub fn perform_uncurry_optimization(&self) -> bool {
        match self.fix_opt_level {
            FixOptimizationLevel::None => false,
            FixOptimizationLevel::Minimum => false,
            FixOptimizationLevel::Separated => true,
            FixOptimizationLevel::Default => true,
        }
    }

    pub fn perform_borrowing_optimization(&self) -> bool {
        match self.fix_opt_level {
            FixOptimizationLevel::None => false,
            FixOptimizationLevel::Minimum => false,
            FixOptimizationLevel::Separated => true,
            FixOptimizationLevel::Default => true,
        }
    }

    // Get hash value of the configurations that affect the object file generation.
    pub fn object_generation_hash(&self) -> String {
        let mut data = String::new();
        data.push_str(&self.sanitize_memory.to_string());
        data.push_str(&self.fix_opt_level.to_string());
        data.push_str(&self.debug_info.to_string());
        data.push_str(&self.threaded.to_string());
        data.push_str(&self.c_type_sizes.to_string());
        data.push_str(build_time_utc!()); // Also add build time of the compiler.
        format!("{:x}", md5::compute(data))
    }

    pub fn separate_compilation(&self) -> bool {
        self.fix_opt_level != FixOptimizationLevel::Default
    }

    pub fn edit_features(&self, features: &mut CpuFeatures) {
        if self.valgrind_tool != ValgrindTool::None {
            features.disable_avx512(); // Valgrind-3.22.0 does not support AVX-512 (#41).
        }
    }

    pub fn valgrind_command(&self) -> Command {
        let mut com = Command::new("valgrind");
        com.arg("--error-exitcode=1"); // This option makes valgrind return 1 if an error is detected.
        com.arg("--suppressions=valgrind.supp");
        match self.valgrind_tool {
            ValgrindTool::None => {
                error_exit("Valgrind tool is not specified.");
            }
            ValgrindTool::MemCheck => {
                // Check memory leaks.
                com.arg("--tool=memcheck");
                com.arg("--leak-check=yes"); // This option turns memory leak into error.
            }
        }
        com
    }

    pub fn external_if_separated(&self) -> Linkage {
        if self.separate_compilation() {
            Linkage::External
        } else {
            Linkage::Internal
        }
    }

    pub fn run_extra_commands(&self) -> Result<(), Errors> {
        for com in &self.extra_commands {
            com.run()?;
        }
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CTypeSizes {
    pub char: usize,
    pub short: usize,
    pub int: usize,
    pub long: usize,
    pub long_long: usize,
    pub size_t: usize,
    pub float: usize,
    pub double: usize,
}

impl CTypeSizes {
    pub fn get_c_types(&self) -> Vec<(&str, &str, usize)> {
        vec![
            (C_CHAR_NAME, "I", self.char),
            (C_UNSIGNED_CHAR_NAME, "U", self.char),
            (C_SHORT_NAME, "I", self.short),
            (C_UNSIGNED_SHORT_NAME, "U", self.short),
            (C_INT_NAME, "I", self.int),
            (C_UNSIGNED_INT_NAME, "U", self.int),
            (C_LONG_NAME, "I", self.long),
            (C_UNSIGNED_LONG_NAME, "U", self.long),
            (C_LONG_LONG_NAME, "I", self.long_long),
            (C_UNSIGNED_LONG_LONG_NAME, "U", self.long_long),
            (C_SIZE_T_NAME, "U", self.size_t),
            (C_FLOAT_NAME, "F", self.float),
            (C_DOUBLE_NAME, "F", self.double),
        ]
    }

    fn to_string(&self) -> String {
        vec![
            format!("char: {}", self.char),
            format!("short: {}", self.short),
            format!("int: {}", self.int),
            format!("long: {}", self.long),
            format!("long long: {}", self.long_long),
            format!("size_t: {}", self.size_t),
            format!("float: {}", self.float),
            format!("double: {}", self.double),
        ]
        .join(", ")
    }

    // Get the size of each C types by compiling and running a C program.
    fn from_gcc() -> Result<Self, Errors> {
        // First, create a C source file to check the size of each C types.
        let c_source = r#"
#include <stdio.h>
#include <stddef.h>
#include <limits.h>
int main() {
    printf("%lu\n", sizeof(char) * CHAR_BIT);
    printf("%lu\n", sizeof(short) * CHAR_BIT);
    printf("%lu\n", sizeof(int) * CHAR_BIT);
    printf("%lu\n", sizeof(long) * CHAR_BIT);
    printf("%lu\n", sizeof(long long) * CHAR_BIT);
    printf("%lu\n", sizeof(size_t) * CHAR_BIT);
    printf("%lu\n", sizeof(float) * CHAR_BIT);
    printf("%lu\n", sizeof(double) * CHAR_BIT);
    return 0;
}
        "#;
        // Then save it to a temporary file ".fixlang/check_c_types.c".
        let check_c_types_path = PathBuf::from(CHECK_C_TYPES_PATH);

        // Create parent folders.
        let parent = check_c_types_path.parent().unwrap();
        if let Err(e) = std::fs::create_dir_all(parent) {
            return Err(Errors::from_msg(format!(
                "Failed to create directory \"{}\": {}",
                parent.to_string_lossy().to_string(),
                e
            )));
        }
        if let Err(e) = std::fs::write(&check_c_types_path, c_source) {
            return Err(Errors::from_msg(format!(
                "Failed to write file \"{}\": {}",
                check_c_types_path.to_string_lossy().to_string(),
                e
            )));
        }

        // Run it by gcc.
        let output = Command::new("gcc")
            .arg(CHECK_C_TYPES_PATH)
            .arg("-o")
            .arg(CHECK_C_TYPES_EXEC_PATH)
            .output();
        if let Err(e) = output {
            return Err(Errors::from_msg(format!(
                "Failed to compile \"{}\": {}.",
                CHECK_C_TYPES_PATH, e
            )));
        }
        let output = output.unwrap();

        // Run the program and parse the result to create CTypeSizes.
        if !output.status.success() {
            return Err(Errors::from_msg(format!(
                "Failed to compile \"{}\": \"{}\".",
                CHECK_C_TYPES_PATH,
                String::from_utf8_lossy(&output.stderr)
            )));
        }
        let output = Command::new(CHECK_C_TYPES_EXEC_PATH).output();
        if let Err(e) = output {
            return Err(Errors::from_msg(format!(
                "Failed to run \"{}\": {}.",
                CHECK_C_TYPES_EXEC_PATH, e
            )));
        }
        let output = output.unwrap();
        if !output.status.success() {
            return Err(Errors::from_msg(format!(
                "Failed to run \"{}\": \"{}\".",
                CHECK_C_TYPES_EXEC_PATH,
                String::from_utf8_lossy(&output.stderr)
            )));
        }
        let output = String::from_utf8_lossy(&output.stdout);
        let mut lines = output.lines();
        let char = lines.next().unwrap().parse().unwrap();
        let short = lines.next().unwrap().parse().unwrap();
        let int = lines.next().unwrap().parse().unwrap();
        let long = lines.next().unwrap().parse().unwrap();
        let long_long = lines.next().unwrap().parse().unwrap();
        let size_t = lines.next().unwrap().parse().unwrap();
        let float = lines.next().unwrap().parse().unwrap();
        let double = lines.next().unwrap().parse().unwrap();
        let res = CTypeSizes {
            char,
            short,
            int,
            long,
            long_long,
            size_t,
            float,
            double,
        };
        Ok(res)
    }

    fn save_to_file(&self) -> Result<(), Errors> {
        // Open json file.
        let path = C_TYPES_JSON_PATH;
        let file = std::fs::File::create(path);
        if let Err(e) = file {
            return Err(Errors::from_msg(format!(
                "Failed to create \"{}\": {}",
                path, e
            )));
        }
        let file = file.unwrap();

        // Serialize and write to the file.
        if let Err(e) = serde_json::to_writer_pretty(file, self) {
            return Err(Errors::from_msg(format!(
                "Failed to write \"{}\": {}",
                path, e
            )));
        }
        Ok(())
    }

    fn load_file() -> Option<Self> {
        let path = PathBuf::from(C_TYPES_JSON_PATH);
        if !path.exists() {
            return None;
        }
        let file = std::fs::File::open(path);
        if file.is_err() {
            eprintln!("Failed to open \"{}\".", C_TYPES_JSON_PATH);
            return None;
        }
        let file = file.unwrap();
        let sizes = serde_json::from_reader(file);
        if sizes.is_err() {
            eprintln!("Failed to parse the content of \"{}\".", C_TYPES_JSON_PATH);
            return None;
        }
        Some(sizes.unwrap())
    }

    fn load_or_check() -> Result<Self, Errors> {
        match Self::load_file() {
            Some(sizes) => Ok(sizes),
            None => {
                let sizes = Self::from_gcc()?;
                sizes.save_to_file()?;
                Ok(sizes)
            }
        }
    }
}
