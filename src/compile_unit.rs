/*
Cache system for object (*.o) files.
*/

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::path::PathBuf;

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::TargetMachine;
use rand::Rng;

use crate::ast::name::FullName;
use crate::ast::name::Name;
use crate::configuration::Configuration;
use crate::constants::COMPILATION_UNITS_PATH;
use crate::split_by_max_size;
use crate::GenerationContext;
use crate::InstantiatedSymbol;

pub struct CompileUnit<'c> {
    // Name of symbols in the module
    symbols: Vec<FullName>,
    // Modules on which symbols in this compilation unit depends.
    dependent_modules: Vec<Name>,
    // Name of this compilation unit. Generated by hashing the names of symbols and the hashes of dependent modules.
    unit_hash: String,
    // LLVM module
    module: Option<Module<'c>>,
}

impl<'c> fmt::Display for CompileUnit<'c> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CompileUnit(hash = {}, size = {}, symbols = [{}, ...], dependency = [{}], is_cached = {})",
            self.unit_hash,
            self.symbols.len(),
            if self.symbols.len() > 0 {
                self.symbols[0].to_string()
            } else {
                "N/A".to_string()
            },
            self.dependent_modules.join(", "),
            self.is_cached()
        )
    }
}

impl<'c> CompileUnit<'c> {
    pub fn new(symbols: Vec<FullName>, dependent_modules: Vec<Name>) -> Self {
        CompileUnit {
            symbols,
            dependent_modules,
            unit_hash: "".to_string(),
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
        self.object_file_path().exists()
    }

    pub fn object_file_path(&self) -> PathBuf {
        if self.unit_hash.len() == 0 {
            panic!("unit_hash is not set.");
        }
        let mut path = PathBuf::from(COMPILATION_UNITS_PATH);
        let file_name = self.unit_hash.to_string() + ".o";
        path.push(file_name);
        path
    }

    // Calculate the hash of this compilation unit and set it to `self`.
    fn update_unit_hash(
        &mut self,
        module_dependency_hash: &HashMap<Name, String>,
        config: &Configuration,
    ) {
        if self.unit_hash.len() > 0 {
            return;
        }
        assert!(self.symbols.len() > 0);
        assert!(self.dependent_modules.len() > 0);

        self.symbols.sort();
        self.dependent_modules.sort();

        // Add dependency to the configuration.
        let mut data = vec![];
        data.push("<configuration>".to_string());
        data.push(config.object_generation_hash());

        // Add dependency to the symbols.
        data.push("<symbols>".to_string());
        for name in &self.symbols {
            data.push(name.to_string());
        }

        // Add dependency to source codes of the dependent modules.
        data.push("<dependent modules>".to_string());
        for name in &self.dependent_modules {
            data.push(module_dependency_hash[name].clone());
        }

        self.unit_hash = format!("{:x}", md5::compute(data.join(", ")));
    }

    // Set the hash of this compilation unit to a random value.
    // This makes the compilation unit to be recompiled always.
    pub fn set_random_unit_hash(&mut self) {
        assert!(self.unit_hash.len() == 0);
        self.unit_hash = format!(
            "{:x}",
            md5::compute(rand::thread_rng().gen::<u64>().to_string())
        );
    }

    pub fn split_by_max_size(self, max_size: usize) -> Vec<CompileUnit<'c>> {
        // `unit_hash` is lost after this method is called.
        assert_eq!(self.unit_hash, "");
        // `module` is lost after this method is called.
        assert!(self.module.is_none());

        let symbols = self.symbols;
        let dependent_modules = self.dependent_modules;

        let split_symbols = split_by_max_size(symbols, max_size);
        let mut units = vec![];
        for symbols in split_symbols {
            units.push(CompileUnit::new(symbols, dependent_modules.clone()));
        }

        units
    }

    // Given a sequence of symbols, split it into compilation units.
    pub fn split_symbols(
        symbols: &[&InstantiatedSymbol],
        module_dependency_hash: &HashMap<Name, String>,
        module_dependency_map: &HashMap<Name, HashSet<Name>>,
        config: &Configuration,
    ) -> Vec<CompileUnit<'c>> {
        let mut units: HashMap<
            String, /* concatenated string of dependent modules sorted by their names */
            CompileUnit<'c>,
        > = HashMap::new();
        // Classify symbols into compilation units depending on their dependent modules.
        for symbol in symbols {
            let name = symbol.instantiated_name.clone();
            let mut depmods = HashSet::new();
            for module in symbol.dependent_modules() {
                depmods.extend(module_dependency_map[&module].clone());
            }
            let mut depmods = depmods.iter().cloned().collect::<Vec<_>>();
            depmods.sort();
            let concat_depmods = depmods.join(", ");
            let unit = if let Some(unit) = units.get_mut(&concat_depmods) {
                unit
            } else {
                units.insert(concat_depmods.clone(), CompileUnit::new(vec![], depmods));
                units.get_mut(&concat_depmods).unwrap()
            };
            unit.symbols.push(name);
        }
        let units = units.into_iter().map(|(_, unit)| unit).collect::<Vec<_>>();

        // Split compilation units into smaller ones if they are too large.
        let mut units = units
            .into_iter()
            .flat_map(|unit| unit.split_by_max_size(config.max_cu_size))
            .collect::<Vec<_>>();

        // Set unit hash.
        for unit in &mut units {
            unit.update_unit_hash(module_dependency_hash, config);
        }

        units
    }
}
