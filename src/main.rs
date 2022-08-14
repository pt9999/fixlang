extern crate pest;
#[macro_use]
extern crate pest_derive;
#[allow(unused_imports)]
#[macro_use]
extern crate serial_test;

mod ast;
mod generator;
mod parser;
#[cfg(test)]
mod tests;

use ast::*;
use clap::{App, AppSettings, Arg};
use either::Either;
use generator::*;
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::support::load_library_permanently;
use inkwell::types::{BasicTypeEnum, FunctionType, IntType, PointerType, StructType};
use inkwell::values::{
    BasicValue, BasicValueEnum, CallableValue, FunctionValue, IntValue, PointerValue,
};
use inkwell::{AddressSpace, IntPredicate, OptimizationLevel};
use once_cell::sync::Lazy;
use parser::*;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use std::alloc::System;
use std::collections::{HashMap, HashSet};
use std::ffi::CString;
use std::fmt::Pointer;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::ptr::null;
use std::string;
use std::sync::Arc;
use std::thread::panicking;
use std::vec::Vec;
use Either::Right;

#[derive(Eq, Hash, PartialEq, Clone)]
enum ObjectFieldType {
    ControlBlock,
    LambdaFunction,
    SubObject,
    Int,
    Bool,
}

impl ObjectFieldType {
    fn to_basic_type<'ctx>(&self, context: &'ctx Context) -> BasicTypeEnum<'ctx> {
        match self {
            ObjectFieldType::ControlBlock => control_block_type(context).into(),
            ObjectFieldType::LambdaFunction => ptr_to_lambda_function_type(context).into(),
            ObjectFieldType::SubObject => ptr_to_object_type(context).into(),
            ObjectFieldType::Int => context.i64_type().into(),
            ObjectFieldType::Bool => context.i8_type().into(),
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct ObjectType {
    field_types: Vec<ObjectFieldType>,
}

impl ObjectType {
    // fn from_type(ty: Arc<Type>) -> Self {
    //     if ty == *INT_TYPE {
    //         return Self::int_obj_type();
    //     }
    //     match &*ty {
    //         Type::TyVar(var) => ObjectType::from_type(var.ty().clone()),
    //         Type::LitTy(_) => unreachable!("Should have treated above."),
    //         Type::AppTy(_, _) => todo!(),
    //         Type::TyConApp(_, _) => todo!(),
    //         Type::FunTy(_, _) => {
    //             let mut field_types: Vec<ObjectFieldType> = Default::default();
    //             field_types.push(ObjectFieldType::ControlBlock);
    //             field_types.push(ObjectFieldType::LambdaFunction);
    //             // Following fields may exist, but their types are unknown.
    //             ObjectType { field_types }
    //         }
    //         Type::ForAllTy(_, _) => todo!(),
    //     }
    //     // let mut field_types: Vec<ObjectFieldType> = Default::default();
    //     // field_types.push(ObjectFieldType::ControlBlock);
    //     // ObjectType { field_types }
    // }
    fn to_struct_type<'ctx>(&self, context: &'ctx Context) -> StructType<'ctx> {
        let mut fields: Vec<BasicTypeEnum<'ctx>> = vec![];
        for field_type in &self.field_types {
            fields.push(field_type.to_basic_type(context));
        }
        context.struct_type(&fields, false)
    }

    fn shared_obj_type(mut field_types: Vec<ObjectFieldType>) -> Self {
        let mut fields = vec![ObjectFieldType::ControlBlock];
        fields.append(&mut field_types);
        Self {
            field_types: fields,
        }
    }

    fn int_obj_type() -> Self {
        Self::shared_obj_type(vec![ObjectFieldType::Int])
    }

    fn bool_obj_type() -> Self {
        Self::shared_obj_type(vec![ObjectFieldType::Bool])
    }

    fn lam_obj_type() -> Self {
        let mut field_types: Vec<ObjectFieldType> = Default::default();
        field_types.push(ObjectFieldType::ControlBlock);
        field_types.push(ObjectFieldType::LambdaFunction);
        // Following fields may exist, but their types are unknown.
        ObjectType { field_types }
    }

    fn generate_func_dtor<'c, 'm, 'b>(
        &self,
        gc: &mut GenerationContext<'c, 'm, 'b>,
    ) -> FunctionValue<'c> {
        if gc
            .system_functions
            .contains_key(&SystemFunctions::Dtor(self.clone()))
        {
            return *gc
                .system_functions
                .get(&SystemFunctions::Dtor(self.clone()))
                .unwrap();
        }
        let struct_type = self.to_struct_type(gc.context);
        let func_type = dtor_type(gc.context);
        let func = gc.module.add_function("dtor", func_type, None);
        let bb = gc.context.append_basic_block(func, "entry");
        let builder = gc.context.create_builder();
        {
            let context = gc.context;
            let module = gc.module;
            // Create new gc
            let gc = GenerationContext {
                context,
                module,
                builder: &builder,
                scope: Default::default(), // This gc use used only for build_release, and it doesn't use scope.
                system_functions: gc.system_functions.clone(),
            };
            builder.position_at_end(bb);
            let ptr_to_obj = func.get_first_param().unwrap().into_pointer_value();
            let ptr_to_obj = gc.builder.build_pointer_cast(
                ptr_to_obj,
                struct_type.ptr_type(AddressSpace::Generic),
                "ptr_to_obj",
            );
            for (i, ft) in self.field_types.iter().enumerate() {
                match ft {
                    ObjectFieldType::SubObject => {
                        let ptr_to_subobj =
                            build_get_field(ptr_to_obj, i as u32, &gc).into_pointer_value();
                        build_release(ptr_to_subobj, &gc);
                    }
                    ObjectFieldType::ControlBlock => {}
                    ObjectFieldType::Int => {}
                    ObjectFieldType::LambdaFunction => {}
                    ObjectFieldType::Bool => {}
                }
            }
            builder.build_return(None);
        }
        gc.system_functions
            .insert(SystemFunctions::Dtor(self.clone()), func);
        func
    }

    fn build_allocate_shared_obj<'c, 'm, 'b>(
        &self,
        gc: &mut GenerationContext<'c, 'm, 'b>,
        name: Option<&str>,
    ) -> PointerValue<'c> {
        let context = gc.context;
        let builder = gc.builder;
        let struct_type = self.to_struct_type(context);
        // NOTE: Only once allocation is needed since we don't implement weak_ptr
        let ptr_to_obj = builder.build_malloc(struct_type, "ptr_to_obj").unwrap();

        let mut object_id = obj_id_type(gc.context).const_int(0, false);

        if SANITIZE_MEMORY {
            let string_ptr = name.unwrap_or("N/A");
            let string_ptr = builder.build_global_string_ptr(string_ptr, "name_of_obj");
            let string_ptr = string_ptr.as_pointer_value();
            let string_ptr = gc.builder.build_pointer_cast(
                string_ptr,
                gc.context.i8_type().ptr_type(AddressSpace::Generic),
                "name_of_obj_i8ptr",
            );
            let ptr = builder.build_pointer_cast(
                ptr_to_obj,
                ptr_to_object_type(gc.context),
                "cast_to_i8ptr",
            );
            let obj_id = builder.build_call(
                *gc.system_functions
                    .get(&SystemFunctions::ReportMalloc)
                    .unwrap(),
                &[ptr.into(), string_ptr.into()],
                "call_report_malloc",
            );
            object_id = obj_id.try_as_basic_value().unwrap_left().into_int_value();
        }

        for (i, ft) in self.field_types.iter().enumerate() {
            match ft {
                ObjectFieldType::ControlBlock => {
                    let ptr_to_control_block = builder
                        .build_struct_gep(ptr_to_obj, i as u32, "ptr_to_control_block")
                        .unwrap();
                    let ptr_to_refcnt = builder
                        .build_struct_gep(ptr_to_control_block, 0, "ptr_to_refcnt")
                        .unwrap();
                    // The initial value of refcnt should be one (as std::make_shared of C++ does).
                    builder.build_store(ptr_to_refcnt, refcnt_type(context).const_int(1, false));
                    let ptr_to_dtor_field = builder
                        .build_struct_gep(ptr_to_control_block, 1, "ptr_to_dtor_field")
                        .unwrap();
                    let dtor = self.generate_func_dtor(gc);
                    builder
                        .build_store(ptr_to_dtor_field, dtor.as_global_value().as_pointer_value());

                    if SANITIZE_MEMORY {
                        let ptr_to_objid = builder
                            .build_struct_gep(ptr_to_control_block, 2, "ptr_to_objid")
                            .unwrap();
                        builder.build_store(ptr_to_objid, object_id);
                    }
                }
                ObjectFieldType::Int => {}
                ObjectFieldType::SubObject => {}
                ObjectFieldType::LambdaFunction => {}
                ObjectFieldType::Bool => {}
            }
        }
        ptr_to_obj
    }
}

fn refcnt_type<'ctx>(context: &'ctx Context) -> IntType<'ctx> {
    context.i64_type()
}

fn ptr_to_refcnt_type<'ctx>(context: &'ctx Context) -> PointerType<'ctx> {
    refcnt_type(context).ptr_type(AddressSpace::Generic)
}

fn obj_id_type<'ctx>(context: &'ctx Context) -> IntType<'ctx> {
    context.i64_type()
}

fn ptr_to_object_type<'ctx>(context: &'ctx Context) -> PointerType<'ctx> {
    context.i8_type().ptr_type(AddressSpace::Generic)
}

fn dtor_type<'ctx>(context: &'ctx Context) -> FunctionType<'ctx> {
    context
        .void_type()
        .fn_type(&[ptr_to_object_type(context).into()], false)
}

fn ptr_to_dtor_type<'ctx>(context: &'ctx Context) -> PointerType<'ctx> {
    dtor_type(context).ptr_type(AddressSpace::Generic)
}

fn control_block_type<'ctx>(context: &'ctx Context) -> StructType<'ctx> {
    let mut fields = vec![
        refcnt_type(context).into(),
        ptr_to_dtor_type(context).into(),
    ];
    if SANITIZE_MEMORY {
        fields.push(obj_id_type(context).into())
    }
    context.struct_type(&fields, false)
}

fn ptr_to_control_block_type<'ctx>(context: &'ctx Context) -> PointerType<'ctx> {
    control_block_type(context).ptr_type(AddressSpace::Generic)
}

fn lambda_function_type<'ctx>(context: &'ctx Context) -> FunctionType<'ctx> {
    // A function that takes argument and context (=lambda object itself).
    ptr_to_object_type(context).fn_type(
        &[
            ptr_to_object_type(context).into(),
            ptr_to_object_type(context).into(),
        ],
        false,
    )
}

fn ptr_to_lambda_function_type<'ctx>(context: &'ctx Context) -> PointerType<'ctx> {
    lambda_function_type(context).ptr_type(AddressSpace::Generic)
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum SystemFunctions {
    Printf,
    ReportMalloc,
    ReportRetain,
    ReportRelease,
    CheckLeak,
    PrintIntObj,
    RetainObj,
    ReleaseObj,
    EmptyDestructor,
    Dtor(ObjectType),
}

fn generate_func_printf<'c, 'm, 'b>(gc: &GenerationContext<'c, 'm, 'b>) -> FunctionValue<'c> {
    let context = gc.context;
    let module = gc.module;

    let i32_type = context.i32_type();
    let i8_type = context.i8_type();
    let i8_ptr_type = i8_type.ptr_type(inkwell::AddressSpace::Generic);

    let fn_type = i32_type.fn_type(&[i8_ptr_type.into()], true);
    let func = module.add_function("printf", fn_type, None);

    func
}

fn generate_func_report_malloc<'c, 'm, 'b>(
    gc: &GenerationContext<'c, 'm, 'b>,
) -> FunctionValue<'c> {
    let fn_ty = gc.context.i64_type().fn_type(
        &[
            ptr_to_object_type(gc.context).into(),
            gc.context.i8_type().ptr_type(AddressSpace::Generic).into(),
        ],
        false,
    );
    gc.module.add_function("report_malloc", fn_ty, None)
}

fn generate_func_report_retain<'c, 'm, 'b>(
    gc: &GenerationContext<'c, 'm, 'b>,
) -> FunctionValue<'c> {
    let fn_ty = gc.context.void_type().fn_type(
        &[
            ptr_to_object_type(gc.context).into(),
            obj_id_type(gc.context).into(),
            refcnt_type(gc.context).into(),
        ],
        false,
    );
    gc.module.add_function("report_retain", fn_ty, None)
}

fn generate_func_report_release<'c, 'm, 'b>(
    gc: &GenerationContext<'c, 'm, 'b>,
) -> FunctionValue<'c> {
    let fn_ty = gc.context.void_type().fn_type(
        &[
            ptr_to_object_type(gc.context).into(),
            obj_id_type(gc.context).into(),
            refcnt_type(gc.context).into(),
        ],
        false,
    );
    gc.module.add_function("report_release", fn_ty, None)
}

fn generate_check_leak<'c, 'm, 'b>(gc: &GenerationContext<'c, 'm, 'b>) -> FunctionValue<'c> {
    let fn_ty = gc.context.void_type().fn_type(&[], false);
    gc.module.add_function("check_leak", fn_ty, None)
}

fn generate_func_print_int_obj<'c, 'm, 'b>(
    gc: &GenerationContext<'c, 'm, 'b>,
) -> FunctionValue<'c> {
    let context = gc.context;
    let module = gc.module;
    let system_functions = &gc.system_functions;
    let void_type = context.void_type();
    let int_obj_type = ObjectType::int_obj_type().to_struct_type(context);
    let int_obj_ptr_type = int_obj_type.ptr_type(AddressSpace::Generic);
    let fn_type = void_type.fn_type(&[int_obj_ptr_type.into()], false);
    let func = module.add_function("print_int_obj", fn_type, None);

    let entry_bb = context.append_basic_block(func, "entry");
    let builder = context.create_builder();
    builder.position_at_end(entry_bb);
    let int_obj_ptr = func.get_first_param().unwrap().into_pointer_value();
    let int_field_ptr = builder
        .build_struct_gep(int_obj_ptr, 1, "int_field_ptr")
        .unwrap();
    let int_val = builder
        .build_load(int_field_ptr, "int_val")
        .into_int_value();
    let string_ptr = builder.build_global_string_ptr("%lld\n", "int_placefolder");
    let printf_func = *system_functions.get(&SystemFunctions::Printf).unwrap();
    builder.build_call(
        printf_func,
        &[string_ptr.as_pointer_value().into(), int_val.into()],
        "call_print_int",
    );
    builder.build_return(None);

    func
}

fn generate_func_retain_obj<'c, 'm, 'b>(
    gc: &mut GenerationContext<'c, 'm, 'b>,
) -> FunctionValue<'c> {
    let context = gc.context;
    let module = gc.module;
    let void_type = context.void_type();
    let func_type = void_type.fn_type(&[ptr_to_object_type(context).into()], false);
    let retain_func = module.add_function("retain_obj", func_type, None);
    let bb = context.append_basic_block(retain_func, "entry");

    let builder = context.create_builder();
    let (mut new_gc, pop_gc) = gc.push_builder(&builder);
    {
        let gc = &mut new_gc;
        builder.position_at_end(bb);
        let ptr_to_obj = retain_func.get_first_param().unwrap().into_pointer_value();
        let ptr_to_control_block = builder.build_pointer_cast(
            ptr_to_obj,
            ptr_to_control_block_type(gc.context),
            "ptr_to_control_block",
        );
        let ptr_to_refcnt = builder
            .build_struct_gep(ptr_to_control_block, 0, "ptr_to_refcnt")
            .unwrap();
        let refcnt = builder.build_load(ptr_to_refcnt, "refcnt").into_int_value();

        if SANITIZE_MEMORY {
            let objid = build_get_objid(ptr_to_obj, gc);
            builder.build_call(
                *gc.system_functions
                    .get(&SystemFunctions::ReportRetain)
                    .unwrap(),
                &[ptr_to_obj.into(), objid.into(), refcnt.into()],
                "call_report_retain",
            );
        }

        let one = context.i64_type().const_int(1, false);
        let refcnt = builder.build_int_add(refcnt, one, "refcnt");
        builder.build_store(ptr_to_refcnt, refcnt);
        builder.build_return(None);
    }
    pop_gc(new_gc);
    retain_func
    // TODO: Add fence instruction for incrementing refcnt
}

fn generate_func_release_obj<'c, 'm, 'b>(
    gc: &mut GenerationContext<'c, 'm, 'b>,
) -> FunctionValue<'c> {
    let void_type = gc.context.void_type();
    let func_type = void_type.fn_type(&[ptr_to_object_type(gc.context).into()], false);
    let release_func = gc.module.add_function("release_obj", func_type, None);
    let mut bb = gc.context.append_basic_block(release_func, "entry");

    let builder = gc.context.create_builder();
    let (mut new_gc, pop_gc) = gc.push_builder(&builder);
    {
        let gc = &mut new_gc;
        builder.position_at_end(bb);
        let ptr_to_obj = release_func.get_first_param().unwrap().into_pointer_value();
        let ptr_to_control_block = builder.build_pointer_cast(
            ptr_to_obj,
            ptr_to_control_block_type(gc.context),
            "ptr_to_control_block",
        );
        let ptr_to_refcnt = builder
            .build_struct_gep(ptr_to_control_block, 0, "ptr_to_refcnt")
            .unwrap();
        let refcnt = builder.build_load(ptr_to_refcnt, "refcnt").into_int_value();

        if SANITIZE_MEMORY {
            let objid = build_get_objid(ptr_to_obj, gc);
            gc.builder.build_call(
                *gc.system_functions
                    .get(&SystemFunctions::ReportRelease)
                    .unwrap(),
                &[ptr_to_obj.into(), objid.into(), refcnt.into()],
                "report_release_call",
            );
        }

        // Decrement refcnt.
        let one = gc.context.i64_type().const_int(1, false);
        let refcnt = builder.build_int_sub(refcnt, one, "refcnt");
        builder.build_store(ptr_to_refcnt, refcnt);

        let zero = gc.context.i64_type().const_zero();
        let is_refcnt_zero =
            builder.build_int_compare(inkwell::IntPredicate::EQ, refcnt, zero, "is_refcnt_zero");
        let then_bb = gc
            .context
            .append_basic_block(release_func, "refcnt_zero_after_release");
        let cont_bb = gc.context.append_basic_block(release_func, "end");
        builder.build_conditional_branch(is_refcnt_zero, then_bb, cont_bb);

        builder.position_at_end(then_bb);
        let ptr_to_dtor_ptr = builder
            .build_struct_gep(ptr_to_control_block, 1, "ptr_to_dtor_ptr")
            .unwrap();
        let ptr_to_dtor = builder
            .build_load(ptr_to_dtor_ptr, "ptr_to_dtor")
            .into_pointer_value();

        let dtor_func = CallableValue::try_from(ptr_to_dtor).unwrap();
        builder.build_call(dtor_func, &[ptr_to_obj.into()], "call_dtor");
        builder.build_free(ptr_to_refcnt);
        builder.build_unconditional_branch(cont_bb);

        builder.position_at_end(cont_bb);
        builder.build_return(None);
    }
    pop_gc(new_gc);
    release_func
    // TODO: Add fence instruction for incrementing refcnt
    // TODO: Add code for leak detector
}

fn generate_func_empty_destructor<'c, 'm, 'b>(
    gc: &GenerationContext<'c, 'm, 'b>,
) -> FunctionValue<'c> {
    let context = gc.context;
    let module = gc.module;
    let void_type = context.void_type();
    let ptr_to_obj_type = context.i64_type().ptr_type(AddressSpace::Generic);
    let func_type = void_type.fn_type(&[ptr_to_obj_type.into()], false);
    let func = module.add_function("empty_destructor", func_type, None);
    let bb = context.append_basic_block(func, "entry");
    let builder = context.create_builder();
    builder.position_at_end(bb);
    builder.build_return(None);

    func
}

fn generate_func_dtor<'c, 'm, 'b>(
    obj_type: StructType<'c>,
    subobj_indices: &[i32],
    gc: &GenerationContext<'c, 'm, 'b>,
) -> FunctionValue<'c> {
    let context = gc.context;
    let module = gc.module;
    let void_type = context.void_type();
    let ptr_to_obj_type = obj_type.ptr_type(AddressSpace::Generic);
    let func_type = void_type.fn_type(&[ptr_to_obj_type.into()], false);
    let func = module.add_function("destructor", func_type, None); // TODO: give appropriate name
    let bb = context.append_basic_block(func, "entry");
    let builder = context.create_builder();
    builder.position_at_end(bb);
    builder.build_return(None);
    func
}

fn generate_system_functions<'c, 'm, 'b>(gc: &mut GenerationContext<'c, 'm, 'b>) {
    gc.system_functions.insert(
        SystemFunctions::EmptyDestructor,
        generate_func_empty_destructor(gc),
    );
    gc.system_functions
        .insert(SystemFunctions::Printf, generate_func_printf(gc));
    if SANITIZE_MEMORY {
        gc.system_functions.insert(
            SystemFunctions::ReportMalloc,
            generate_func_report_malloc(gc),
        );
        gc.system_functions.insert(
            SystemFunctions::ReportRetain,
            generate_func_report_retain(gc),
        );
        gc.system_functions.insert(
            SystemFunctions::ReportRelease,
            generate_func_report_release(gc),
        );
        gc.system_functions
            .insert(SystemFunctions::CheckLeak, generate_check_leak(gc));
    }
    gc.system_functions.insert(
        SystemFunctions::PrintIntObj,
        generate_func_print_int_obj(gc),
    );
    let retain_func = generate_func_retain_obj(gc);
    gc.system_functions
        .insert(SystemFunctions::RetainObj, retain_func);
    let release_func = generate_func_release_obj(gc);
    gc.system_functions
        .insert(SystemFunctions::ReleaseObj, release_func);
}

fn execute_main_module<'c>(
    context: &'c Context,
    module: &Module<'c>,
    opt_level: OptimizationLevel,
) -> i64 {
    if SANITIZE_MEMORY {
        assert_eq!(
            load_library_permanently("sanitizer/libfixsanitizer.so"),
            false
        );
    }
    let execution_engine = module.create_jit_execution_engine(opt_level).unwrap();
    unsafe {
        let func = execution_engine
            .get_function::<unsafe extern "C" fn() -> i64>("main")
            .unwrap();
        func.call()
    }
}

fn run_ast(program: Arc<ExprInfo>, opt_level: OptimizationLevel) -> i64 {
    // Add library functions to program.
    let program = let_in(var_var("add"), add(), program);
    let program = let_in(var_var("eq"), eq(), program);
    let program = let_in(var_var("fix"), fix(), program);

    let program = calculate_aux_info(program);

    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();
    let mut gc = GenerationContext {
        context: &context,
        module: &module,
        builder: &builder,
        scope: Default::default(),
        system_functions: Default::default(),
    };
    generate_system_functions(&mut gc);

    let main_fn_type = context.i64_type().fn_type(&[], false);
    let main_function = module.add_function("main", main_fn_type, None);

    let entry_bb = context.append_basic_block(main_function, "entry");
    builder.position_at_end(entry_bb);

    let program_result = generate_expr(program, &mut gc);

    let int_obj_ptr = builder.build_pointer_cast(
        program_result.ptr,
        ObjectType::int_obj_type()
            .to_struct_type(&context)
            .ptr_type(AddressSpace::Generic),
        "int_obj_ptr",
    );
    let value = build_get_field(int_obj_ptr, 1, &gc);
    build_release(program_result.ptr, &gc);

    if SANITIZE_MEMORY {
        // Perform leak check
        let check_leak = *gc
            .system_functions
            .get(&SystemFunctions::CheckLeak)
            .unwrap();
        gc.builder.build_call(check_leak, &[], "check_leak");
    }

    if let BasicValueEnum::IntValue(value) = value {
        builder.build_return(Some(&value));
    } else {
        panic!("Given program doesn't return int value!");
    }

    module.print_to_file("ir").unwrap();
    let verify = module.verify();
    if verify.is_err() {
        print!("{}", verify.unwrap_err().to_str().unwrap());
        panic!("LLVM verify failed!");
    }
    execute_main_module(&context, &module, opt_level)
}

fn run_source(source: &str, opt_level: OptimizationLevel) -> i64 {
    let ast = parse_source(source);
    run_ast(ast, opt_level)
}

fn run_file(path: &Path, opt_level: OptimizationLevel) -> i64 {
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    // ファイルの中身を文字列に読み込む。`io::Result<useize>`を返す。
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read {}: {}", display, why),
        Ok(_) => (),
    }

    run_source(s.as_str(), opt_level)
}

fn test_run_source(source: &str, answer: i64, opt_level: OptimizationLevel) {
    assert_eq!(run_source(source, opt_level), answer)
}

const SANITIZE_MEMORY: bool = true;

fn main() {
    let source_file = Arg::new("source-file").required(true);
    let run_subcom = App::new("run").arg(source_file);
    let app = App::new("Fix-lang")
        .bin_name("fix")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(run_subcom);

    match app.get_matches().subcommand() {
        Some(("run", m)) => {
            let path = m.value_of("source-file").unwrap();
            let res = run_file(Path::new(path), OptimizationLevel::Default);
            println!("{}", res);
        }
        _ => eprintln!("Unknown command!"),
    }
}
