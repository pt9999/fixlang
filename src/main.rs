extern crate pest;
#[macro_use]
extern crate pest_derive;
#[allow(unused_imports)]
#[macro_use]
extern crate serial_test;
// extern crate rustc_llvm_proxy;

mod ast;
mod builtin;
mod generator;
mod misc;
mod object;
mod parser;
mod runner;
mod runtime;
mod stdlib;
#[cfg(test)]
mod tests;
mod typecheck;
mod uncurry_optimization;

use ast::expr::*;
use ast::module::*;
use ast::traits::*;
use ast::typedecl::*;
use ast::types::*;
use builtin::*;
use clap::{App, AppSettings, Arg};
use generator::*;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::support::load_library_permanently;
use inkwell::types::{BasicTypeEnum, FunctionType, IntType, PointerType, StructType};
use inkwell::values::{
    BasicValue, BasicValueEnum, CallableValue, FunctionValue, IntValue, PointerValue,
};
use inkwell::{AddressSpace, IntPredicate, OptimizationLevel};
use misc::*;
use object::*;
use parser::*;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use runner::*;
use runtime::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use std::vec::Vec;
use stdlib::*;
use typecheck::*;
use uncurry_optimization::*;

const SANITIZE_MEMORY: bool = false;

const NO_RETAIN_RELEASE: bool = false; // In this mode, not only memory leak occurrs, reference transparency breaks.
const TUPLE_SIZE_MAX: u32 = 4; // This affects on compilation time heavily. We should make tuple generation on-demand.

const UNCURRY_OPTIMIZATION: bool = false;
const TUPLE_UNBOX: bool = true;
const NOT_RETAIN_GLOBAL: bool = true;

fn main() {
    let source_file = Arg::new("source-file").required(true);
    let run_subc = App::new("run").arg(source_file.clone());
    let build_subc = App::new("build").arg(source_file.clone());
    let app = App::new("Fix-lang")
        .bin_name("fix")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(run_subc)
        .subcommand(build_subc);

    match app.get_matches().subcommand() {
        Some(("run", m)) => {
            let path = m.value_of("source-file").unwrap();
            run_file(Path::new(path), OptimizationLevel::Aggressive);
        }
        Some(("build", m)) => {
            let path = m.value_of("source-file").unwrap();
            build_file(Path::new(path), OptimizationLevel::Aggressive);
        }
        _ => eprintln!("Unknown command!"),
    }
}
