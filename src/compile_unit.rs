/*
Cache system for object (*.o) files.
*/

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::path::PathBuf;

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::TargetMachine;

use crate::ast::name::FullName;
use crate::ast::name::Name;
use crate::configuration::Configuration;
use crate::constants::COMPILATION_UNITS_PATH;
use crate::error_exit;
use crate::GenerationContext;

// Determine the name of compilation unit which consists from the given symbols.
// - names: Sequence of symbols. This should be sorted.
// - mod_to_hash: A map from module name to the hash of dependency files.
fn unit_hash(
    symbol_names: &[FullName],
    mod_to_hash: &HashMap<Name, String>,
    config: &Configuration,
) -> String {
    let mut data = config.object_generation_hash();
    for name in symbol_names {
        data.push_str(&name.to_string());
        data.push_str(&mod_to_hash[&name.module()]);
    }
    format!("{:x}", md5::compute(data))
}

fn cache_file_hash_list() -> HashSet<String> {
    let dir_path = PathBuf::from(COMPILATION_UNITS_PATH);
    if !dir_path.exists() {
        return HashSet::new();
    }
    let paths = fs::read_dir(dir_path);
    if paths.is_err() {
        error_exit(&format!(
            "Failed to read directory {}: {}",
            COMPILATION_UNITS_PATH,
            paths.err().unwrap()
        ));
    }
    let dir = paths.unwrap();
    let mut paths = HashSet::new();
    for entry in dir {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            continue;
        }
        if path.extension().is_none() {
            continue;
        }
        if path.extension().unwrap() != "o" {
            continue;
        }
        paths.insert(path.file_stem().unwrap().to_str().unwrap().to_string());
    }
    paths
}

fn is_cached(
    symbol_names: &[FullName],
    mod_to_hash: &HashMap<Name, String>,
    config: &Configuration,
    cache_file_hash_list: &HashSet<String>,
) -> bool {
    cache_file_hash_list.contains(&unit_hash(symbol_names, mod_to_hash, config))
}

fn unit_file_path(unit_hash: &str) -> PathBuf {
    let mut path = PathBuf::from(COMPILATION_UNITS_PATH);
    let file_name = unit_hash.to_string() + ".o";
    path.push(file_name);
    path
}

pub struct CompileUnit<'c> {
    // Name of symbols in the module
    symbols: Vec<FullName>,
    // Name of this compilation unit
    unit_hash: String,
    // Is this unit cached?
    is_cached: bool,
    // LLVM module
    module: Option<Module<'c>>,
}

impl<'c> fmt::Display for CompileUnit<'c> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CompileUnit(hash = {}, symbols[0] = {}, symbols[-1] = {}, is_cached = {})",
            self.unit_hash,
            if self.symbols.len() > 0 {
                self.symbols[0].to_string()
            } else {
                "N/A".to_string()
            },
            if self.symbols.len() > 0 {
                self.symbols[self.symbols.len() - 1].to_string()
            } else {
                "N/A".to_string()
            },
            self.is_cached
        )
    }
}

impl<'c> CompileUnit<'c> {
    pub fn new(
        symbols: &[FullName],
        mod_to_hash: &HashMap<Name, String>,
        config: &Configuration,
        is_cached: bool,
    ) -> Self {
        CompileUnit {
            unit_hash: unit_hash(&symbols, mod_to_hash, config),
            symbols: symbols.to_vec(),
            is_cached,
            module: None,
        }
    }

    pub fn create_module_if_none(&mut self, ctx: &'c Context, target_machine: &TargetMachine) {
        if self.module.is_some() {
            return;
        }
        let module = GenerationContext::create_module(
            &format!("Module_{}", self.unit_hash),
            ctx,
            target_machine,
        );
        self.module = Some(module);
    }

    pub fn module(&self) -> &Module<'c> {
        self.module.as_ref().unwrap()
    }

    pub fn symbols(&self) -> &Vec<FullName> {
        &self.symbols
    }

    pub fn is_cached(&self) -> bool {
        self.is_cached
    }

    pub fn set_unit_hash(&mut self, hash: String) {
        self.unit_hash = hash;
    }

    pub fn obj_path(&self) -> PathBuf {
        unit_file_path(&self.unit_hash)
    }

    // Given a sequence of symbols, split it into compilation units, each of which is either cached or not.
    pub fn split_symbols(
        symbols: &[FullName],
        mod_to_hash: &HashMap<Name, String>,
        config: &Configuration,
    ) -> Vec<CompileUnit<'c>> {
        let cache_file_list = cache_file_hash_list();
        let is_cached =
            |names: &[FullName]| is_cached(names, mod_to_hash, config, &cache_file_list);
        let units = crate::segcache::split_into_units(symbols, is_cached);
        let mut result = vec![];
        for unit in &units {
            let unit = CompileUnit::new(&unit.items(), mod_to_hash, config, unit.is_cached());
            result.push(unit);
        }
        result
    }
}