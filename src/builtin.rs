// Implement built-in functions, types, etc.
use super::*;

pub fn bulitin_tycons() -> HashMap<TyCon, TyConInfo> {
    let mut ret = HashMap::new();
    ret.insert(
        TyCon::new(FullName::from_strs(&[STD_NAME], PTR_NAME)),
        TyConInfo {
            kind: kind_star(),
            variant: TyConVariant::Primitive,
            is_unbox: true,
            tyvars: vec![],
            fields: vec![],
        },
    );
    ret.insert(
        TyCon::new(FullName::from_strs(&[STD_NAME], U8_NAME)),
        TyConInfo {
            kind: kind_star(),
            variant: TyConVariant::Primitive,
            is_unbox: true,
            tyvars: vec![],
            fields: vec![],
        },
    );
    ret.insert(
        TyCon::new(FullName::from_strs(&[STD_NAME], I32_NAME)),
        TyConInfo {
            kind: kind_star(),
            variant: TyConVariant::Primitive,
            is_unbox: true,
            tyvars: vec![],
            fields: vec![],
        },
    );
    ret.insert(
        TyCon::new(FullName::from_strs(&[STD_NAME], U32_NAME)),
        TyConInfo {
            kind: kind_star(),
            variant: TyConVariant::Primitive,
            is_unbox: true,
            tyvars: vec![],
            fields: vec![],
        },
    );
    ret.insert(
        TyCon::new(FullName::from_strs(&[STD_NAME], I64_NAME)),
        TyConInfo {
            kind: kind_star(),
            variant: TyConVariant::Primitive,
            is_unbox: true,
            tyvars: vec![],
            fields: vec![],
        },
    );
    ret.insert(
        TyCon::new(FullName::from_strs(&[STD_NAME], U64_NAME)),
        TyConInfo {
            kind: kind_star(),
            variant: TyConVariant::Primitive,
            is_unbox: true,
            tyvars: vec![],
            fields: vec![],
        },
    );
    ret.insert(
        TyCon::new(FullName::from_strs(&[STD_NAME], BOOL_NAME)),
        TyConInfo {
            kind: kind_star(),
            variant: TyConVariant::Primitive,
            is_unbox: true,
            tyvars: vec![],
            fields: vec![],
        },
    );
    ret.insert(
        TyCon::new(FullName::from_strs(&[STD_NAME], F32_NAME)),
        TyConInfo {
            kind: kind_star(),
            variant: TyConVariant::Primitive,
            is_unbox: true,
            tyvars: vec![],
            fields: vec![],
        },
    );
    ret.insert(
        TyCon::new(FullName::from_strs(&[STD_NAME], F64_NAME)),
        TyConInfo {
            kind: kind_star(),
            variant: TyConVariant::Primitive,
            is_unbox: true,
            tyvars: vec![],
            fields: vec![],
        },
    );
    // IO is defined in the source code of Std.
    ret.insert(
        make_array_tycon(),
        TyConInfo {
            kind: kind_arrow(kind_star(), kind_star()),
            variant: TyConVariant::Array,
            is_unbox: false,
            tyvars: vec!["a".to_string()],
            fields: vec![Field {
                name: "array_elem".to_string(), // Unused
                ty: type_tyvar_star("a"),
            }],
        },
    );
    // String is defined in the source code of Std.

    // Function Pointers
    for arity in 1..=FUNPTR_ARGS_MAX {
        ret.insert(
            make_funptr_tycon(arity),
            TyConInfo {
                kind: make_kind_fun(arity),
                variant: TyConVariant::Primitive,
                is_unbox: true,
                tyvars: (0..arity).map(|i| format!("a{}", i)).collect(),
                fields: vec![],
            },
        );
    }
    // Opaque object
    ret.insert(
        TyCon::new(make_dynamic_object_name()),
        TyConInfo {
            kind: kind_star(),
            variant: TyConVariant::DynamicObject,
            is_unbox: false,
            tyvars: vec![],
            fields: vec![],
        },
    );

    ret
}

pub fn make_dynamic_object_name() -> FullName {
    FullName::from_strs(&[STD_NAME], DYNAMIC_OBJECT_NAME)
}

pub fn make_destructor_object_name() -> FullName {
    FullName::from_strs(&[STD_NAME], DESTRUCTOR_OBJECT_NAME)
}

pub fn make_funptr_name(arity: u32) -> Name {
    format!("{}{}", FUNPTR_NAME, arity)
}

pub fn make_funptr_tycon(arity: u32) -> TyCon {
    TyCon::new(FullName::from_strs(&[STD_NAME], &make_funptr_name(arity)))
}

pub fn make_array_tycon() -> TyCon {
    TyCon::new(FullName::from_strs(&[STD_NAME], ARRAY_NAME))
}

// If given tycon is function pointer, returns it's arity
pub fn is_funptr_tycon(tc: &TyCon) -> Option<u32> {
    if tc.name.namespace != NameSpace::new(vec![STD_NAME.to_string()]) {
        return None;
    }
    if !tc.name.name.starts_with(FUNPTR_NAME) {
        return None;
    }
    let mut chars = tc.name.name.chars();
    for _ in 0..FUNPTR_NAME.len() {
        chars.next();
    }
    let number = chars.as_str().to_string();
    Some(number.parse::<u32>().unwrap())
}

// Returns whether given tycon is dyanmic object
pub fn is_dynamic_object_tycon(tc: &TyCon) -> bool {
    tc.name == make_dynamic_object_name()
}

// Returns whether given tycon is Std::Destructor
pub fn is_destructor_object_tycon(tc: &TyCon) -> bool {
    tc.name == make_destructor_object_name()
}

// Returns whether given tycon is array
pub fn is_array_tycon(tc: &TyCon) -> bool {
    *tc == make_array_tycon()
}

pub fn make_kind_fun(arity: u32) -> Rc<Kind> {
    let mut res = kind_star();
    for _ in 0..arity {
        res = kind_arrow(kind_star(), res);
    }
    res
}

// Following types are coustructed using primitive types.
pub const LOOP_RESULT_NAME: &str = "LoopResult";
pub const TUPLE_NAME: &str = "Tuple";

// Get Ptr type.
pub fn make_ptr_ty() -> Rc<TypeNode> {
    type_tycon(&tycon(FullName::from_strs(&[STD_NAME], PTR_NAME)))
}

// Get U8 type.
pub fn make_u8_ty() -> Rc<TypeNode> {
    type_tycon(&tycon(FullName::from_strs(&[STD_NAME], U8_NAME)))
}

// Get I32 type.
pub fn make_i32_ty() -> Rc<TypeNode> {
    type_tycon(&tycon(FullName::from_strs(&[STD_NAME], I32_NAME)))
}

// Get U32 type.
pub fn make_u32_ty() -> Rc<TypeNode> {
    type_tycon(&tycon(FullName::from_strs(&[STD_NAME], U32_NAME)))
}

// Get I64 type.
pub fn make_i64_ty() -> Rc<TypeNode> {
    type_tycon(&tycon(FullName::from_strs(&[STD_NAME], I64_NAME)))
}

// Get U32 type.
pub fn make_u64_ty() -> Rc<TypeNode> {
    type_tycon(&tycon(FullName::from_strs(&[STD_NAME], U64_NAME)))
}

// Get F32 type.
pub fn make_f32_ty() -> Rc<TypeNode> {
    type_tycon(&tycon(FullName::from_strs(&[STD_NAME], F32_NAME)))
}

// Get F64 type.
pub fn make_f64_ty() -> Rc<TypeNode> {
    type_tycon(&tycon(FullName::from_strs(&[STD_NAME], F64_NAME)))
}

// Get Bool type.
pub fn make_bool_ty() -> Rc<TypeNode> {
    type_tycon(&tycon(FullName::from_strs(&[STD_NAME], BOOL_NAME)))
}

// Get Array type.
pub fn make_array_ty() -> Rc<TypeNode> {
    type_tycon(&tycon(FullName::from_strs(&[STD_NAME], ARRAY_NAME)))
}

// Get String type.
pub fn make_string_ty() -> Rc<TypeNode> {
    type_tycon(&tycon(FullName::from_strs(&[STD_NAME], STRING_NAME)))
}

// Get LoopResult type.
pub fn make_loop_result_ty() -> Rc<TypeNode> {
    type_tycon(&tycon(FullName::from_strs(&[STD_NAME], LOOP_RESULT_NAME)))
}

// Get dynamic object type.
pub fn make_dynamic_object_ty() -> Rc<TypeNode> {
    type_tycon(&tycon(FullName::from_strs(
        &[STD_NAME],
        DYNAMIC_OBJECT_NAME,
    )))
}

// Get tuple type.
pub fn make_tuple_ty(tys: Vec<Rc<TypeNode>>) -> Rc<TypeNode> {
    assert!(tys.len() <= FUNPTR_ARGS_MAX as usize);
    let mut ty = type_tycon(&tycon(make_tuple_name(tys.len() as u32)));
    for field_ty in tys {
        ty = type_tyapp(ty, field_ty);
    }
    ty
}

// Make tuple name.
pub fn make_tuple_name(size: u32) -> FullName {
    let name = format!("{}{}", TUPLE_NAME, size);
    FullName::from_strs(&[STD_NAME], &name)
}

// Get Unit type.
pub fn make_unit_ty() -> Rc<TypeNode> {
    make_tuple_ty(vec![])
}

// Make type `IO ()`
pub fn make_io_unit_ty() -> Rc<TypeNode> {
    type_tyapp(
        type_tycon(&tycon(FullName::from_strs(&[STD_NAME], IO_NAME))),
        make_unit_ty(),
    )
}

// Check if given name has form `TupleN` and returns N.
pub fn get_tuple_n(name: &FullName) -> Option<u32> {
    if name.namespace != NameSpace::new_str(&[STD_NAME]) {
        return None;
    }
    if name.name.len() < TUPLE_NAME.len() {
        return None;
    }
    let prefix = &name.name[..TUPLE_NAME.len()];
    if prefix != TUPLE_NAME {
        return None;
    }
    let number_str = &name.name[TUPLE_NAME.len()..];
    number_str.parse::<u32>().ok()
}

pub fn expr_int_lit(val: u64, ty: Rc<TypeNode>, source: Option<Span>) -> Rc<ExprNode> {
    let generator: Rc<InlineLLVM> = Rc::new(move |gc, ty, rvo| {
        let obj = if rvo.is_none() {
            allocate_obj(
                ty.clone(),
                &vec![],
                None,
                gc,
                Some(&format!("int_lit_{}", val)),
            )
        } else {
            rvo.unwrap()
        };
        let int_ty = ty
            .get_struct_type(gc, &vec![])
            .get_field_type_at_index(0)
            .unwrap()
            .into_int_type();
        let value = int_ty.const_int(val as u64, false);
        obj.store_field_nocap(gc, 0, value);
        obj
    });
    expr_lit(generator, vec![], val.to_string(), ty, source)
}

pub fn expr_float_lit(val: f64, ty: Rc<TypeNode>, source: Option<Span>) -> Rc<ExprNode> {
    let generator: Rc<InlineLLVM> = Rc::new(move |gc, ty, rvo| {
        let obj = if rvo.is_none() {
            allocate_obj(
                ty.clone(),
                &vec![],
                None,
                gc,
                Some(&format!("float_lit_{}", val)),
            )
        } else {
            rvo.unwrap()
        };
        let float_ty = ty
            .get_struct_type(gc, &vec![])
            .get_field_type_at_index(0)
            .unwrap()
            .into_float_type();
        let value = float_ty.const_float(val);
        obj.store_field_nocap(gc, 0, value);
        obj
    });
    expr_lit(generator, vec![], val.to_string(), ty, source)
}

pub fn expr_nullptr_lit(source: Option<Span>) -> Rc<ExprNode> {
    let generator: Rc<InlineLLVM> = Rc::new(move |gc, ty, rvo| {
        let obj = if rvo.is_none() {
            allocate_obj(ty.clone(), &vec![], None, gc, Some("nullptr"))
        } else {
            rvo.unwrap()
        };
        let ptr_ty = gc.context.i8_type().ptr_type(AddressSpace::from(0));
        let value = ptr_ty.const_null();
        obj.store_field_nocap(gc, 0, value);
        obj
    });
    expr_lit(
        generator,
        vec![],
        "nullptr_literal".to_string(),
        make_ptr_ty(),
        source,
    )
}

pub fn expr_bool_lit(val: bool, source: Option<Span>) -> Rc<ExprNode> {
    let generator: Rc<InlineLLVM> = Rc::new(move |gc, ty, rvo| {
        let obj = if rvo.is_none() {
            allocate_obj(
                ty.clone(),
                &vec![],
                None,
                gc,
                Some(&format!("bool_lit_{}", val)),
            )
        } else {
            rvo.unwrap()
        };
        let value = gc.context.i8_type().const_int(val as u64, false);
        obj.store_field_nocap(gc, 0, value);
        obj
    });
    expr_lit(generator, vec![], val.to_string(), make_bool_ty(), source)
}

pub fn make_string_from_ptr<'c, 'm>(
    gc: &mut GenerationContext<'c, 'm>,
    buf_with_null_terminator: PointerValue<'c>,
    len_with_null_terminator: IntValue<'c>,
    rvo: Option<Object<'c>>,
) -> Object<'c> {
    // Create `Array U8` which contains null-terminated string.
    let array_ty = type_tyapp(make_array_ty(), make_u8_ty());
    let array = allocate_obj(
        array_ty,
        &vec![],
        Some(len_with_null_terminator),
        gc,
        Some("array@make_string_from_ptr"),
    );
    array.store_field_nocap(gc, ARRAY_LEN_IDX, len_with_null_terminator);
    let dst = array.ptr_to_field_nocap(gc, ARRAY_BUF_IDX);
    let len_ptr = gc.builder().build_int_cast(
        len_with_null_terminator,
        gc.context.ptr_sized_int_type(gc.target_data(), None),
        "len_ptr@make_string_from_ptr",
    );
    gc.builder()
        .build_memcpy(dst, 1, buf_with_null_terminator, 1, len_ptr)
        .ok()
        .unwrap();

    // Allocate String and store the array into it.
    let string = if rvo.is_none() {
        allocate_obj(
            make_string_ty(),
            &vec![],
            None,
            gc,
            Some(&format!("string@make_string_from_ptr")),
        )
    } else {
        rvo.unwrap()
    };
    assert!(string.is_unbox(gc.type_env()));

    // Store array to data.
    let array_val = array.value(gc);
    string.store_field_nocap(gc, 0, array_val);

    string
}

pub fn make_string_from_rust_string(string: String, source: Option<Span>) -> Rc<ExprNode> {
    let generator: Rc<InlineLLVM> = Rc::new(move |gc, _, rvo| {
        let string_ptr = gc
            .builder()
            .build_global_string_ptr(&string, "string_literal")
            .as_basic_value_enum()
            .into_pointer_value();
        let len_with_null_terminator = gc
            .context
            .i64_type()
            .const_int(string.as_bytes().len() as u64 + 1, false);
        make_string_from_ptr(gc, string_ptr, len_with_null_terminator, rvo)
    });
    expr_lit(
        generator,
        vec![],
        "string_literal".to_string(),
        make_string_ty(),
        source,
    )
}

fn fix_lit(b: &str, f: &str, x: &str) -> Rc<ExprNode> {
    let f_str = FullName::local(f);
    let x_str = FullName::local(x);
    let name = format!("fix({}, {})", f_str.to_string(), x_str.to_string());
    let free_vars = vec![FullName::local(CAP_NAME), f_str.clone(), x_str.clone()];
    let generator: Rc<InlineLLVM> = Rc::new(move |gc, _ty, rvo| {
        // Get arguments
        let x = gc.get_var(&x_str).ptr.get(gc);
        let f = gc.get_var(&f_str).ptr.get(gc);

        // Create "fix(f)" closure.
        let fixf_ty = f.ty.get_lambda_dst();
        let fixf = allocate_obj(fixf_ty.clone(), &vec![], None, gc, Some("fix(f)"));
        let fixf_funptr = gc
            .builder()
            .get_insert_block()
            .unwrap()
            .get_parent()
            .unwrap()
            .as_global_value()
            .as_pointer_value();
        fixf.store_field_nocap(gc, CLOSURE_FUNPTR_IDX, fixf_funptr);
        let cap_obj = gc.get_var(&FullName::local(CAP_NAME)).ptr.get(gc);
        let cap_obj_ptr = cap_obj.ptr(gc);
        fixf.store_field_nocap(gc, CLOSURE_CAPTURE_IDX, cap_obj_ptr);

        let f_fixf = gc.apply_lambda(f, vec![fixf], None);
        let f_fixf_x = gc.apply_lambda(f_fixf, vec![x], rvo);
        f_fixf_x
    });
    expr_lit(generator, free_vars, name, type_tyvar_star(b), None)
}

// fix = \f: ((a -> b) -> (a -> b)) -> \x: a -> fix_lit(b, f, x): b
pub fn fix() -> (Rc<ExprNode>, Rc<Scheme>) {
    let expr = expr_abs(
        vec![var_local("f")],
        expr_abs(vec![var_local("x")], fix_lit("b", "f", "x"), None),
        None,
    );
    let fixed_ty = type_fun(type_tyvar_star("a"), type_tyvar_star("b"));
    let scm = Scheme::generalize(
        HashMap::from([
            ("a".to_string(), kind_star()),
            ("b".to_string(), kind_star()),
        ]),
        vec![],
        type_fun(type_fun(fixed_ty.clone(), fixed_ty.clone()), fixed_ty),
    );
    (expr, scm)
}

// number_to_string function
pub fn number_to_string_function(ty: Rc<TypeNode>) -> (Rc<ExprNode>, Rc<Scheme>) {
    const VAL_NAME: &str = "number";
    let (buf_size, specifier) = match ty.toplevel_tycon().unwrap().name.name.as_str() {
        U8_NAME => (4, C_U8_FORMATTER),
        I32_NAME => (12, C_I32_FORMATTER),
        U32_NAME => (11, C_U32_FORMATTER),
        I64_NAME => (21, C_I64_FORMATTER),
        U64_NAME => (20, C_U64_FORMATTER),
        F32_NAME => (50, C_F32_FORMATTER),
        F64_NAME => (500, C_F64_FORMATTER),
        _ => unreachable!(),
    };
    let generator: Rc<InlineLLVM> = Rc::new(move |gc, _, rvo| {
        // Get value
        let val = gc.get_var_field(&FullName::local(VAL_NAME), 0);
        gc.release(gc.get_var(&FullName::local(VAL_NAME)).ptr.get(gc));

        // Allocate buffer for sprintf.
        let buf_size = gc.context.i32_type().const_int(buf_size as u64, false);
        let buf = gc.builder().build_array_alloca(
            gc.context.i8_type(),
            buf_size,
            "buf_for_sprintf@number_to_string",
        );

        // Call sprintf.
        let format = gc
            .builder()
            .build_global_string_ptr(specifier, "format@number_to_string")
            .as_basic_value_enum()
            .into_pointer_value();
        let len = gc
            .call_runtime(
                RuntimeFunctions::Sprintf,
                &[buf.into(), format.into(), val.into()],
            )
            .try_as_basic_value()
            .unwrap_left()
            .into_int_value();

        // Make String.
        let len_with_null_terminator_i32 = gc.builder().build_int_add(
            len,
            gc.context.i32_type().const_int(1, false),
            "len_with_null_terminator_i32@number_to_string",
        );
        let len_with_null_terminator = gc.builder().build_int_cast(
            len_with_null_terminator_i32,
            gc.context.i64_type(),
            "len_with_null_terminator@number_to_string",
        );

        make_string_from_ptr(gc, buf, len_with_null_terminator, rvo)
    });
    let scm = Scheme::generalize(
        Default::default(),
        vec![],
        type_fun(ty.clone(), make_string_ty()),
    );
    let expr = expr_abs(
        vec![var_local(VAL_NAME)],
        expr_lit(
            generator,
            vec![FullName::local(VAL_NAME)],
            format!("{}_to_string({})", ty.to_string(), VAL_NAME),
            make_string_ty(),
            None,
        ),
        None,
    );
    (expr, scm)
}

// Cast function of integrals
pub fn cast_between_integral_function(
    from: Rc<TypeNode>,
    to: Rc<TypeNode>,
) -> (Rc<ExprNode>, Rc<Scheme>) {
    const FROM_NAME: &str = "from";
    let is_signed = from.toplevel_tycon().unwrap().is_singned_intger()
        && to.toplevel_tycon().unwrap().is_singned_intger();
    let generator: Rc<InlineLLVM> = Rc::new(move |gc, to_ty, rvo| {
        // Get value
        let from_val = gc
            .get_var_field(&FullName::local(FROM_NAME), 0)
            .into_int_value();
        gc.release(gc.get_var(&FullName::local(FROM_NAME)).ptr.get(gc));

        // Get target type.
        let to_int = to_ty
            .get_struct_type(gc, &vec![])
            .get_field_type_at_index(0)
            .unwrap()
            .into_int_type();

        // Perform cast.
        let to_val = gc.builder().build_int_cast_sign_flag(
            from_val,
            to_int,
            is_signed,
            "int_cast_sign_flag@cast_between_integral_function",
        );

        // Return result.
        let obj = if rvo.is_some() {
            rvo.unwrap()
        } else {
            allocate_obj(
                to_ty.clone(),
                &vec![],
                None,
                gc,
                Some("alloca@cast_between_integral_function"),
            )
        };
        obj.store_field_nocap(gc, 0, to_val);
        obj
    });
    let scm = Scheme::generalize(
        Default::default(),
        vec![],
        type_fun(from.clone(), to.clone()),
    );
    let expr = expr_abs(
        vec![var_local(FROM_NAME)],
        expr_lit(
            generator,
            vec![FullName::local(FROM_NAME)],
            format!(
                "cast_{}_to_{}({})",
                from.to_string(),
                to.to_string(),
                FROM_NAME
            ),
            to,
            None,
        ),
        None,
    );
    (expr, scm)
}

// Cast function of integrals
pub fn cast_between_float_function(
    from: Rc<TypeNode>,
    to: Rc<TypeNode>,
) -> (Rc<ExprNode>, Rc<Scheme>) {
    const FROM_NAME: &str = "from";
    let generator: Rc<InlineLLVM> = Rc::new(move |gc, to_ty, rvo| {
        // Get value
        let from_val = gc
            .get_var_field(&FullName::local(FROM_NAME), 0)
            .into_float_value();
        gc.release(gc.get_var(&FullName::local(FROM_NAME)).ptr.get(gc));

        // Get target type.
        let to_float = to_ty
            .get_struct_type(gc, &vec![])
            .get_field_type_at_index(0)
            .unwrap()
            .into_float_type();

        // Perform cast.
        let to_val = gc.builder().build_float_cast(
            from_val,
            to_float,
            "float_cast@cast_between_float_function",
        );

        // Return result.
        let obj = if rvo.is_some() {
            rvo.unwrap()
        } else {
            allocate_obj(
                to_ty.clone(),
                &vec![],
                None,
                gc,
                Some("alloca@cast_between_float_function"),
            )
        };
        obj.store_field_nocap(gc, 0, to_val);
        obj
    });
    let scm = Scheme::generalize(
        Default::default(),
        vec![],
        type_fun(from.clone(), to.clone()),
    );
    let expr = expr_abs(
        vec![var_local(FROM_NAME)],
        expr_lit(
            generator,
            vec![FullName::local(FROM_NAME)],
            format!(
                "cast_{}_to_{}({})",
                from.to_string(),
                to.to_string(),
                FROM_NAME
            ),
            to,
            None,
        ),
        None,
    );
    (expr, scm)
}

// Cast function from int to float.
pub fn cast_int_to_float_function(
    from: Rc<TypeNode>,
    to: Rc<TypeNode>,
) -> (Rc<ExprNode>, Rc<Scheme>) {
    const FROM_NAME: &str = "from";
    let is_signed = from.toplevel_tycon().unwrap().is_singned_intger();

    let generator: Rc<InlineLLVM> = Rc::new(move |gc, to_ty, rvo| {
        // Get value
        let from_val = gc
            .get_var_field(&FullName::local(FROM_NAME), 0)
            .into_int_value();
        gc.release(gc.get_var(&FullName::local(FROM_NAME)).ptr.get(gc));

        // Get target type.
        let to_float = to_ty
            .get_struct_type(gc, &vec![])
            .get_field_type_at_index(0)
            .unwrap()
            .into_float_type();

        // Perform cast.
        let to_val = if is_signed {
            gc.builder().build_signed_int_to_float(
                from_val,
                to_float,
                "signed_int_to_float@cast_int_to_float_function",
            )
        } else {
            gc.builder().build_unsigned_int_to_float(
                from_val,
                to_float,
                "unsigned_int_to_float@cast_int_to_float_function",
            )
        };

        // Return result.
        let obj = if rvo.is_some() {
            rvo.unwrap()
        } else {
            allocate_obj(
                to_ty.clone(),
                &vec![],
                None,
                gc,
                Some("alloca@cast_int_to_float_function"),
            )
        };
        obj.store_field_nocap(gc, 0, to_val);
        obj
    });
    let scm = Scheme::generalize(
        Default::default(),
        vec![],
        type_fun(from.clone(), to.clone()),
    );
    let expr = expr_abs(
        vec![var_local(FROM_NAME)],
        expr_lit(
            generator,
            vec![FullName::local(FROM_NAME)],
            format!(
                "cast_{}_to_{}({})",
                from.to_string(),
                to.to_string(),
                FROM_NAME
            ),
            to,
            None,
        ),
        None,
    );
    (expr, scm)
}

// Cast function from int to float.
pub fn cast_float_to_int_function(
    from: Rc<TypeNode>,
    to: Rc<TypeNode>,
) -> (Rc<ExprNode>, Rc<Scheme>) {
    const FROM_NAME: &str = "from";

    let generator: Rc<InlineLLVM> = Rc::new(move |gc, to_ty, rvo| {
        // Get value
        let from_val = gc
            .get_var_field(&FullName::local(FROM_NAME), 0)
            .into_float_value();
        gc.release(gc.get_var(&FullName::local(FROM_NAME)).ptr.get(gc));

        // Get target type.
        let to_int = to_ty
            .get_struct_type(gc, &vec![])
            .get_field_type_at_index(0)
            .unwrap()
            .into_int_type();

        // Perform cast.
        let to_val = gc.builder().build_float_to_signed_int(
            from_val,
            to_int,
            "float_to_signed_int@cast_float_to_int_function",
        );

        // Return result.
        let obj = if rvo.is_some() {
            rvo.unwrap()
        } else {
            allocate_obj(
                to_ty.clone(),
                &vec![],
                None,
                gc,
                Some("alloca@cast_float_to_int_function"),
            )
        };
        obj.store_field_nocap(gc, 0, to_val);
        obj
    });
    let scm = Scheme::generalize(
        Default::default(),
        vec![],
        type_fun(from.clone(), to.clone()),
    );
    let expr = expr_abs(
        vec![var_local(FROM_NAME)],
        expr_lit(
            generator,
            vec![FullName::local(FROM_NAME)],
            format!(
                "cast_{}_to_{}({})",
                from.to_string(),
                to.to_string(),
                FROM_NAME
            ),
            to,
            None,
        ),
        None,
    );
    (expr, scm)
}

// Shift functions
pub fn shift_function(ty: Rc<TypeNode>, is_left: bool) -> (Rc<ExprNode>, Rc<Scheme>) {
    const VALUE_NAME: &str = "val";
    const N_NAME: &str = "n";

    let generator: Rc<InlineLLVM> = Rc::new(move |gc, ty, rvo| {
        // Get value
        let val = gc
            .get_var_field(&FullName::local(VALUE_NAME), 0)
            .into_int_value();
        let n = gc
            .get_var_field(&FullName::local(N_NAME), 0)
            .into_int_value();

        let is_signed = ty.toplevel_tycon().unwrap().is_singned_intger();

        // Perform cast.
        let to_val = if is_left {
            gc.builder()
                .build_left_shift(val, n, "left_shift@shift_function")
        } else {
            gc.builder()
                .build_right_shift(val, n, is_signed, "right_shift@shift_function")
        };

        // Return result.
        let obj = if rvo.is_some() {
            rvo.unwrap()
        } else {
            allocate_obj(ty.clone(), &vec![], None, gc, Some("alloca@shift_function"))
        };
        obj.store_field_nocap(gc, 0, to_val);
        obj
    });
    let scm = Scheme::generalize(
        Default::default(),
        vec![],
        type_fun(ty.clone(), type_fun(ty.clone(), ty.clone())),
    );
    let expr = expr_abs(
        vec![var_local(N_NAME)],
        expr_abs(
            vec![var_local(VALUE_NAME)],
            expr_lit(
                generator,
                vec![FullName::local(VALUE_NAME), FullName::local(N_NAME)],
                format!(
                    "shift_{}({},{})",
                    if is_left { "left" } else { "right" },
                    N_NAME,
                    VALUE_NAME
                ),
                ty,
                None,
            ),
            None,
        ),
        None,
    );
    (expr, scm)
}

#[derive(Clone, Copy)]
pub enum BitOperationType {
    Xor,
    Or,
    And,
}

impl BitOperationType {
    pub fn to_string(&self) -> String {
        match self {
            BitOperationType::Xor => "xor",
            BitOperationType::Or => "or",
            BitOperationType::And => "and",
        }
        .to_string()
    }
}

pub fn bitwise_operation_function(
    ty: Rc<TypeNode>,
    op_type: BitOperationType,
) -> (Rc<ExprNode>, Rc<Scheme>) {
    const LHS_NAME: &str = "lhs";
    const RHS_NAME: &str = "rhs";

    let generator: Rc<InlineLLVM> = Rc::new(move |gc, ty, rvo| {
        // Get value
        let lhs = gc
            .get_var_field(&FullName::local(LHS_NAME), 0)
            .into_int_value();
        let rhs = gc
            .get_var_field(&FullName::local(RHS_NAME), 0)
            .into_int_value();

        // Perform cast.
        let val = match op_type {
            BitOperationType::Xor => {
                gc.builder()
                    .build_xor(lhs, rhs, "xor@bitwise_operation_function")
            }
            BitOperationType::Or => {
                gc.builder()
                    .build_or(lhs, rhs, "or@bitwise_operation_function")
            }
            BitOperationType::And => {
                gc.builder()
                    .build_and(lhs, rhs, "and@bitwise_operation_function")
            }
        };

        // Return result.
        let obj = if rvo.is_some() {
            rvo.unwrap()
        } else {
            allocate_obj(
                ty.clone(),
                &vec![],
                None,
                gc,
                Some("alloca@bitwise_operation_function"),
            )
        };
        obj.store_field_nocap(gc, 0, val);
        obj
    });
    let scm = Scheme::generalize(
        Default::default(),
        vec![],
        type_fun(ty.clone(), type_fun(ty.clone(), ty.clone())),
    );
    let expr = expr_abs(
        vec![var_local(LHS_NAME)],
        expr_abs(
            vec![var_local(RHS_NAME)],
            expr_lit(
                generator,
                vec![FullName::local(LHS_NAME), FullName::local(RHS_NAME)],
                format!("bit_{}({},{})", op_type.to_string(), LHS_NAME, RHS_NAME),
                ty,
                None,
            ),
            None,
        ),
        None,
    );
    (expr, scm)
}

// Implementation of Array::fill built-in function.
fn fill_array_lit(a: &str, size: &str, value: &str) -> Rc<ExprNode> {
    let size_str = FullName::local(size);
    let value_str = FullName::local(value);
    let name = format!("Array::fill({}, {})", size, value);
    let name_cloned = name.clone();
    let free_vars = vec![size_str.clone(), value_str.clone()];
    let generator: Rc<InlineLLVM> = Rc::new(move |gc, ty, rvo| {
        let size = gc.get_var_field(&size_str, 0).into_int_value();
        gc.release(gc.get_var(&size_str).ptr.get(gc));
        let value = gc.get_var(&value_str).ptr.get(gc);
        assert!(rvo.is_none()); // Array is boxed, and we don't perform rvo for boxed values.
        let array = allocate_obj(
            ty.clone(),
            &vec![],
            Some(size),
            gc,
            Some(name_cloned.as_str()),
        );
        array.store_field_nocap(gc, ARRAY_LEN_IDX, size);
        let buf = array.ptr_to_field_nocap(gc, ARRAY_BUF_IDX);
        ObjectFieldType::initialize_array_buf_by_value(gc, size, buf, value);
        array
    });
    expr_lit(
        generator,
        free_vars,
        name,
        type_tyapp(make_array_ty(), type_tyvar_star(a)),
        None,
    )
}

// "Array::fill : I64 -> a -> Array a" built-in function.
// Creates an array with same capacity.
pub fn fill_array() -> (Rc<ExprNode>, Rc<Scheme>) {
    let expr = expr_abs(
        vec![var_local("size")],
        expr_abs(
            vec![var_local("value")],
            fill_array_lit("a", "size", "value"),
            None,
        ),
        None,
    );
    let scm = Scheme::generalize(
        HashMap::from([("a".to_string(), kind_star())]),
        vec![],
        type_fun(
            make_i64_ty(),
            type_fun(
                type_tyvar_star("a"),
                type_tyapp(make_array_ty(), type_tyvar_star("a")),
            ),
        ),
    );
    (expr, scm)
}

// Make an empty array.
pub fn make_empty() -> (Rc<ExprNode>, Rc<Scheme>) {
    const CAP_NAME: &str = "cap";
    const ELEM_TYPE: &str = "a";

    let generator: Rc<InlineLLVM> = Rc::new(move |gc, arr_ty, rvo| {
        assert!(rvo.is_none()); // Array is boxed, and we don't perform rvo for boxed values.

        // Get capacity
        let cap = gc
            .get_var_field(&FullName::local(CAP_NAME), 0)
            .into_int_value();

        // Allocate
        let array = allocate_obj(
            arr_ty.clone(),
            &vec![],
            Some(cap),
            gc,
            Some(&format!("Array::empty({})", CAP_NAME)),
        );

        // Set size to zero.
        let cap = gc.context.i64_type().const_zero();
        array.store_field_nocap(gc, ARRAY_LEN_IDX, cap);

        array
    });

    let elem_tyvar = type_tyvar_star(ELEM_TYPE);
    let array_ty = type_tyapp(make_array_ty(), elem_tyvar);

    let expr = expr_abs(
        vec![var_local(CAP_NAME)],
        expr_lit(
            generator,
            vec![FullName::local(CAP_NAME)],
            format!("make_empty({})", CAP_NAME),
            array_ty.clone(),
            None,
        ),
        None,
    );
    let scm = Scheme::generalize(
        HashMap::from([(ELEM_TYPE.to_string(), kind_star())]),
        vec![],
        type_fun(make_i64_ty(), array_ty),
    );
    (expr, scm)
}

// Set an element to an array, with no uniqueness checking and without releasing the old value.
pub fn unsafe_set_array() -> (Rc<ExprNode>, Rc<Scheme>) {
    const IDX_NAME: &str = "idx";
    const ARR_NAME: &str = "array";
    const VALUE_NAME: &str = "val";
    const ELEM_TYPE: &str = "a";

    let generator: Rc<InlineLLVM> = Rc::new(move |gc, _, rvo| {
        assert!(rvo.is_none()); // Array is boxed, and we don't perform rvo for boxed values.

        // Get argments
        let array = gc.get_var(&FullName::local(ARR_NAME)).ptr.get(gc);
        let idx = gc
            .get_var_field(&FullName::local(IDX_NAME), 0)
            .into_int_value();
        let value = gc.get_var(&FullName::local(VALUE_NAME)).ptr.get(gc);

        // Get array cap and buffer.
        let array_buf = array.ptr_to_field_nocap(gc, ARRAY_BUF_IDX);

        // Perform write and return.
        ObjectFieldType::write_to_array_buf(gc, None, array_buf, idx, value, false);
        array
    });

    let elem_tyvar = type_tyvar_star(ELEM_TYPE);
    let array_ty = type_tyapp(make_array_ty(), elem_tyvar.clone());

    let expr = expr_abs(
        vec![var_local(IDX_NAME)],
        expr_abs(
            vec![var_local(VALUE_NAME)],
            expr_abs(
                vec![var_local(ARR_NAME)],
                expr_lit(
                    generator,
                    vec![
                        FullName::local(IDX_NAME),
                        FullName::local(VALUE_NAME),
                        FullName::local(ARR_NAME),
                    ],
                    format!("{}.unsafe_set({}, {})", ARR_NAME, IDX_NAME, VALUE_NAME),
                    array_ty.clone(),
                    None,
                ),
                None,
            ),
            None,
        ),
        None,
    );

    let scm = Scheme::generalize(
        HashMap::from([(ELEM_TYPE.to_string(), kind_star())]),
        vec![],
        type_fun(
            make_i64_ty(),
            type_fun(elem_tyvar.clone(), type_fun(array_ty.clone(), array_ty)),
        ),
    );
    (expr, scm)
}

// Gets a value from an array, without bounds checking and retaining the returned value.
pub fn unsafe_get_array() -> (Rc<ExprNode>, Rc<Scheme>) {
    const IDX_NAME: &str = "idx";
    const ARR_NAME: &str = "array";
    const ELEM_TYPE: &str = "a";

    let generator: Rc<InlineLLVM> = Rc::new(move |gc, ty, rvo| {
        // Get argments
        let array = gc.get_var(&FullName::local(ARR_NAME)).ptr.get(gc);
        let idx = gc
            .get_var_field(&FullName::local(IDX_NAME), 0)
            .into_int_value();

        // Get array buffer
        let buf = array.ptr_to_field_nocap(gc, ARRAY_BUF_IDX);

        // Get element
        let elem =
            ObjectFieldType::read_from_array_buf_noretain(gc, None, buf, ty.clone(), idx, rvo);

        // Release the array.
        gc.release(array);

        elem
    });

    let elem_tyvar = type_tyvar_star(ELEM_TYPE);
    let array_ty = type_tyapp(make_array_ty(), elem_tyvar.clone());

    let expr = expr_abs(
        vec![var_local(IDX_NAME)],
        expr_abs(
            vec![var_local(ARR_NAME)],
            expr_lit(
                generator,
                vec![FullName::local(IDX_NAME), FullName::local(ARR_NAME)],
                format!("{}.get_array_noretain({})", ARR_NAME, IDX_NAME),
                elem_tyvar.clone(),
                None,
            ),
            None,
        ),
        None,
    );

    let scm = Scheme::generalize(
        HashMap::from([(ELEM_TYPE.to_string(), kind_star())]),
        vec![],
        type_fun(make_i64_ty(), type_fun(array_ty, elem_tyvar.clone())),
    );
    (expr, scm)
}

// Set the length of an array, with no uniqueness checking, no validation of size argument.
pub fn unsafe_set_length_array() -> (Rc<ExprNode>, Rc<Scheme>) {
    const ARR_NAME: &str = "array";
    const LENGTH_NAME: &str = "length";
    const ELEM_TYPE: &str = "a";

    let generator: Rc<InlineLLVM> = Rc::new(move |gc, _, rvo| {
        assert!(rvo.is_none()); // Array is boxed, and we don't perform rvo for boxed values.

        // Get argments
        let array = gc.get_var(&FullName::local(ARR_NAME)).ptr.get(gc);
        let length = gc
            .get_var_field(&FullName::local(LENGTH_NAME), 0)
            .into_int_value();

        // Get pointer to length field.
        let ptr_to_length = array.ptr_to_field_nocap(gc, ARRAY_LEN_IDX);

        // Perform write and return.
        gc.builder().build_store(ptr_to_length, length);
        array
    });

    let elem_tyvar = type_tyvar_star(ELEM_TYPE);
    let array_ty = type_tyapp(make_array_ty(), elem_tyvar.clone());

    let expr = expr_abs(
        vec![var_local(LENGTH_NAME)],
        expr_abs(
            vec![var_local(ARR_NAME)],
            expr_lit(
                generator,
                vec![FullName::local(LENGTH_NAME), FullName::local(ARR_NAME)],
                format!("{}.unsafe_set_length({})", ARR_NAME, LENGTH_NAME),
                array_ty.clone(),
                None,
            ),
            None,
        ),
        None,
    );

    let scm = Scheme::generalize(
        HashMap::from([(ELEM_TYPE.to_string(), kind_star())]),
        vec![],
        type_fun(make_i64_ty(), type_fun(array_ty.clone(), array_ty)),
    );
    (expr, scm)
}

// Implementation of Array::get built-in function.
fn read_array_lit(a: &str, array: &str, idx: &str) -> Rc<ExprNode> {
    let elem_ty = type_tyvar_star(a);
    let array_str = FullName::local(array);
    let idx_str = FullName::local(idx);
    let name = format!("Array::get({}, {})", idx, array);
    let free_vars = vec![array_str.clone(), idx_str.clone()];
    let generator: Rc<InlineLLVM> = Rc::new(move |gc, ty, rvo| {
        // Array = [ControlBlock, PtrToArrayField], and ArrayField = [Size, PtrToBuffer].
        let array = gc.get_var(&array_str).ptr.get(gc);
        let len = array.load_field_nocap(gc, ARRAY_LEN_IDX).into_int_value();
        let buf = array.ptr_to_field_nocap(gc, ARRAY_BUF_IDX);
        let idx = gc.get_var_field(&idx_str, 0).into_int_value();
        gc.release(gc.get_var(&idx_str).ptr.get(gc));
        let elem = ObjectFieldType::read_from_array_buf(gc, Some(len), buf, ty.clone(), idx, rvo);
        gc.release(array);
        elem
    });
    expr_lit(generator, free_vars, name, elem_ty, None)
}

// "Array::get : Array a -> I64 -> a" built-in function.
pub fn read_array() -> (Rc<ExprNode>, Rc<Scheme>) {
    let expr = expr_abs(
        vec![var_local("idx")],
        expr_abs(
            vec![var_local("array")],
            read_array_lit("a", "array", "idx"),
            None,
        ),
        None,
    );
    let scm = Scheme::generalize(
        HashMap::from([("a".to_string(), kind_star())]),
        vec![],
        type_fun(
            make_i64_ty(),
            type_fun(
                type_tyapp(make_array_ty(), type_tyvar_star("a")),
                type_tyvar_star("a"),
            ),
        ),
    );
    (expr, scm)
}

// Force array object to be unique.
// If it is unique, do nothing.
// If it is shared, clone the object or panics if panic_if_shared is true.
fn make_array_unique<'c, 'm>(
    gc: &mut GenerationContext<'c, 'm>,
    array: Object<'c>,
    panic_if_shared: bool,
) -> Object<'c> {
    let elem_ty = array.ty.field_types(gc.type_env())[0].clone();
    let original_array_ptr = array.ptr(gc);

    // Get refcnt.
    let refcnt = {
        let array_ptr = array.ptr(gc);
        gc.load_obj_field(array_ptr, control_block_type(gc), 0)
            .into_int_value()
    };

    // Add shared / cont bbs.
    let current_bb = gc.builder().get_insert_block().unwrap();
    let current_func = current_bb.get_parent().unwrap();
    let shared_bb = gc
        .context
        .append_basic_block(current_func, "array_shared_bb");
    let cont_bb = gc
        .context
        .append_basic_block(current_func, "after_unique_array_bb");

    // Jump to shared_bb if refcnt > 1.
    let one = refcnt_type(gc.context).const_int(1, false);
    let is_unique = gc
        .builder()
        .build_int_compare(IntPredicate::EQ, refcnt, one, "is_unique");
    gc.builder()
        .build_conditional_branch(is_unique, cont_bb, shared_bb);

    // In shared_bb, create new array and clone array field.
    gc.builder().position_at_end(shared_bb);
    if panic_if_shared {
        // In case of unique version, panic in this case.
        gc.panic("an array is asserted as unique but is shared!\n");
    }
    // Allocate cloned array.
    let array_cap = array.load_field_nocap(gc, ARRAY_CAP_IDX).into_int_value();
    let cloned_array = allocate_obj(
        array.ty.clone(),
        &vec![],
        Some(array_cap),
        gc,
        Some("cloned_array_for_uniqueness"),
    );
    // Set the length of the cloned array.
    let array_len = array.load_field_nocap(gc, ARRAY_LEN_IDX).into_int_value();
    cloned_array.store_field_nocap(gc, ARRAY_LEN_IDX, array_len);
    // Copy elements to the cloned array.
    let cloned_array_buf = cloned_array.ptr_to_field_nocap(gc, ARRAY_BUF_IDX);
    let array_buf = array.ptr_to_field_nocap(gc, ARRAY_BUF_IDX);
    ObjectFieldType::clone_array_buf(gc, array_len, array_buf, cloned_array_buf, elem_ty);
    gc.release(array.clone()); // Given array should be released here.

    // Jump to the next bb.
    let succ_of_shared_bb = gc.builder().get_insert_block().unwrap();
    let cloned_array_ptr = cloned_array.ptr(gc);
    gc.builder().build_unconditional_branch(cont_bb);

    // Implement cont_bb
    gc.builder().position_at_end(cont_bb);

    // Build phi value of array and array_field.
    let array_phi = gc
        .builder()
        .build_phi(original_array_ptr.get_type(), "array_phi");
    array_phi.add_incoming(&[
        (&original_array_ptr, current_bb),
        (&cloned_array_ptr, succ_of_shared_bb),
    ]);
    let array = Object::new(
        array_phi.as_basic_value().into_pointer_value(),
        array.ty.clone(),
    );

    array
}

// Implementation of Array::set/Array::set! built-in function.
// is_unique_mode - if true, generate code that calls abort when given array is shared.
fn set_array_lit(
    a: &str,
    array: &str,
    idx: &str,
    value: &str,
    is_unique_version: bool,
) -> Rc<ExprNode> {
    let elem_ty = type_tyvar_star(a);
    let array_str = FullName::local(array);
    let idx_str = FullName::local(idx);
    let value_str = FullName::local(value);
    let func_name = String::from({
        if is_unique_version {
            "set!"
        } else {
            "set"
        }
    });
    let name = format!("{} {} {} {}", func_name, idx, value, array);
    let free_vars = vec![array_str.clone(), idx_str.clone(), value_str.clone()];
    let generator: Rc<InlineLLVM> = Rc::new(move |gc, _, rvo| {
        assert!(rvo.is_none());

        // Get argments
        let array = gc.get_var(&array_str).ptr.get(gc);
        let idx = gc.get_var_field(&idx_str, 0).into_int_value();
        gc.release(gc.get_var(&idx_str).ptr.get(gc));
        let value = gc.get_var(&value_str).ptr.get(gc);

        // Force array to be unique
        let array = make_array_unique(gc, array, is_unique_version);

        // Perform write and return.
        let array_len = array.load_field_nocap(gc, ARRAY_LEN_IDX).into_int_value();
        let array_buf = array.ptr_to_field_nocap(gc, ARRAY_BUF_IDX);
        ObjectFieldType::write_to_array_buf(gc, Some(array_len), array_buf, idx, value, true);
        array
    });
    expr_lit(
        generator,
        free_vars,
        name,
        type_tyapp(make_array_ty(), elem_ty),
        None,
    )
}

// Array::set built-in function.
pub fn set_array_common(is_unique_version: bool) -> (Rc<ExprNode>, Rc<Scheme>) {
    let expr = expr_abs(
        vec![var_local("idx")],
        expr_abs(
            vec![var_local("value")],
            expr_abs(
                vec![var_local("array")],
                set_array_lit("a", "array", "idx", "value", is_unique_version),
                None,
            ),
            None,
        ),
        None,
    );
    let array_ty = type_tyapp(make_array_ty(), type_tyvar_star("a"));
    let scm = Scheme::generalize(
        HashMap::from([("a".to_string(), kind_star())]),
        vec![],
        type_fun(
            make_i64_ty(),
            type_fun(type_tyvar_star("a"), type_fun(array_ty.clone(), array_ty)),
        ),
    );
    (expr, scm)
}

// set built-in function.
pub fn write_array() -> (Rc<ExprNode>, Rc<Scheme>) {
    set_array_common(false)
}

// set! built-in function.
pub fn write_array_unique() -> (Rc<ExprNode>, Rc<Scheme>) {
    set_array_common(true)
}

pub fn mod_array(is_unique_version: bool) -> (Rc<ExprNode>, Rc<Scheme>) {
    const MODIFIED_ARRAY_NAME: &str = "arr";
    const MODIFIER_NAME: &str = "f";
    const INDEX_NAME: &str = "idx";
    const ELEM_TYPE: &str = "a";

    let generator: Rc<InlineLLVM> = Rc::new(move |gc, _, rvo| {
        assert!(rvo.is_none());

        // Get argments
        let array = gc
            .get_var(&FullName::local(MODIFIED_ARRAY_NAME))
            .ptr
            .get(gc);
        let idx = gc
            .get_var_field(&FullName::local(INDEX_NAME), 0)
            .into_int_value();
        let modifier = gc.get_var(&FullName::local(MODIFIER_NAME)).ptr.get(gc);

        // Make array unique
        let array = make_array_unique(gc, array, is_unique_version);

        // Get old element without retain.
        let array_len = array.load_field_nocap(gc, ARRAY_LEN_IDX).into_int_value();
        let array_buf = array.ptr_to_field_nocap(gc, ARRAY_BUF_IDX);
        let elem_ty = array.ty.field_types(gc.type_env())[0].clone();
        let elem = ObjectFieldType::read_from_array_buf_noretain(
            gc,
            Some(array_len),
            array_buf,
            elem_ty,
            idx,
            None,
        );

        // Apply modifier to get a new value.
        let elem = gc.apply_lambda(modifier, vec![elem], None);

        // Perform write and return.
        ObjectFieldType::write_to_array_buf(gc, None, array_buf, idx, elem, false);
        array
    });

    let elem_tyvar = type_tyvar_star(ELEM_TYPE);
    let array_ty = type_tyapp(make_array_ty(), elem_tyvar.clone());

    let expr = expr_abs(
        vec![var_local(INDEX_NAME)],
        expr_abs(
            vec![var_local(MODIFIER_NAME)],
            expr_abs(
                vec![var_local(MODIFIED_ARRAY_NAME)],
                expr_lit(
                    generator,
                    vec![
                        FullName::local(INDEX_NAME),
                        FullName::local(MODIFIER_NAME),
                        FullName::local(MODIFIED_ARRAY_NAME),
                    ],
                    format!(
                        "{}.mod{}({}, {})",
                        MODIFIED_ARRAY_NAME,
                        if is_unique_version { "!" } else { "" },
                        INDEX_NAME,
                        MODIFIER_NAME
                    ),
                    array_ty.clone(),
                    None,
                ),
                None,
            ),
            None,
        ),
        None,
    );
    let scm = Scheme::generalize(
        HashMap::from([(ELEM_TYPE.to_string(), kind_star())]),
        vec![],
        type_fun(
            make_i64_ty(),
            type_fun(
                type_fun(elem_tyvar.clone(), elem_tyvar),
                type_fun(array_ty.clone(), array_ty),
            ),
        ),
    );
    (expr, scm)
}

pub fn force_unique_array(is_unique_version: bool) -> (Rc<ExprNode>, Rc<Scheme>) {
    const ARRAY_NAME: &str = "arr";
    const ELEM_TYPE: &str = "a";

    let generator: Rc<InlineLLVM> = Rc::new(move |gc, _, rvo| {
        assert!(rvo.is_none());

        // Get argments
        let array = gc.get_var(&FullName::local(ARRAY_NAME)).ptr.get(gc);

        // Make array unique
        let array = make_array_unique(gc, array, is_unique_version);

        array
    });

    let elem_tyvar = type_tyvar_star(ELEM_TYPE);
    let array_ty = type_tyapp(make_array_ty(), elem_tyvar.clone());

    let expr = expr_abs(
        vec![var_local(ARRAY_NAME)],
        expr_lit(
            generator,
            vec![FullName::local(ARRAY_NAME)],
            format!(
                "{}.force_unique{}",
                ARRAY_NAME,
                if is_unique_version { "!" } else { "" },
            ),
            array_ty.clone(),
            None,
        ),
        None,
    );
    let scm = Scheme::generalize(
        HashMap::from([(ELEM_TYPE.to_string(), kind_star())]),
        vec![],
        type_fun(array_ty.clone(), array_ty),
    );
    (expr, scm)
}

// `get_ptr` function for Array.
pub fn get_ptr_array() -> (Rc<ExprNode>, Rc<Scheme>) {
    const ARRAY_NAME: &str = "arr";
    const ELEM_TYPE: &str = "a";

    let generator: Rc<InlineLLVM> = Rc::new(move |gc, _, rvo| {
        // Get argment
        let array = gc.get_var(&FullName::local(ARRAY_NAME)).ptr.get(gc);

        // Get pointer
        let ptr = array.ptr_to_field_nocap(gc, ARRAY_BUF_IDX);
        let ptr_ty = ObjectFieldType::Ptr.to_basic_type(gc).into_pointer_type();
        let ptr = gc.cast_pointer(ptr, ptr_ty);

        // Release array
        gc.release(array);

        // Make returned object
        let obj = if rvo.is_some() {
            rvo.unwrap()
        } else {
            allocate_obj(
                make_ptr_ty(),
                &vec![],
                None,
                gc,
                Some("alloca@get_ptr_array"),
            )
        };
        obj.store_field_nocap(gc, 0, ptr);

        obj
    });

    let elem_tyvar = type_tyvar_star(ELEM_TYPE);
    let array_ty = type_tyapp(make_array_ty(), elem_tyvar.clone());

    let expr = expr_abs(
        vec![var_local(ARRAY_NAME)],
        expr_lit(
            generator,
            vec![FullName::local(ARRAY_NAME)],
            format!("{}.get_ptr", ARRAY_NAME,),
            make_ptr_ty(),
            None,
        ),
        None,
    );
    let scm = Scheme::generalize(
        HashMap::from([(ELEM_TYPE.to_string(), kind_star())]),
        vec![],
        type_fun(array_ty.clone(), make_ptr_ty()),
    );
    (expr, scm)
}

// `get_size` built-in function for Array.
pub fn get_size_array() -> (Rc<ExprNode>, Rc<Scheme>) {
    const ARR_NAME: &str = "arr";

    let generator: Rc<InlineLLVM> = Rc::new(move |gc, _ty, rvo| {
        let arr_name = FullName::local(ARR_NAME);
        // Array = [ControlBlock, PtrToArrayField], and ArrayField = [Size, PtrToBuffer].
        let array_obj = gc.get_var(&arr_name).ptr.get(gc);
        let len = array_obj
            .load_field_nocap(gc, ARRAY_LEN_IDX)
            .into_int_value();
        gc.release(array_obj);
        let int_obj = if rvo.is_none() {
            allocate_obj(make_i64_ty(), &vec![], None, gc, Some("length_of_arr"))
        } else {
            rvo.unwrap()
        };
        int_obj.store_field_nocap(gc, 0, len);
        int_obj
    });

    let expr = expr_abs(
        vec![var_local(ARR_NAME)],
        expr_lit(
            generator,
            vec![FullName::local(ARR_NAME)],
            "len arr".to_string(),
            make_i64_ty(),
            None,
        ),
        None,
    );
    let array_ty = type_tyapp(make_array_ty(), type_tyvar_star("a"));
    let scm = Scheme::generalize(
        HashMap::from([("a".to_string(), kind_star())]),
        vec![],
        type_fun(array_ty, make_i64_ty()),
    );
    (expr, scm)
}

// `Array::get_capacity : Array a -> I64` built-in function.
pub fn get_capacity_array() -> (Rc<ExprNode>, Rc<Scheme>) {
    const ARR_NAME: &str = "arr";

    let generator: Rc<InlineLLVM> = Rc::new(move |gc, _ty, rvo| {
        let arr_name = FullName::local(ARR_NAME);
        // Array = [ControlBlock, PtrToArrayField], and ArrayField = [Size, PtrToBuffer].
        let array_obj = gc.get_var(&arr_name).ptr.get(gc);
        let len = array_obj
            .load_field_nocap(gc, ARRAY_CAP_IDX)
            .into_int_value();
        gc.release(array_obj);
        let int_obj = if rvo.is_none() {
            allocate_obj(make_i64_ty(), &vec![], None, gc, Some("cap_of_arr"))
        } else {
            rvo.unwrap()
        };
        int_obj.store_field_nocap(gc, 0, len);
        int_obj
    });

    let expr = expr_abs(
        vec![var_local(ARR_NAME)],
        expr_lit(
            generator,
            vec![FullName::local(ARR_NAME)],
            "arr.get_capacity".to_string(),
            make_i64_ty(),
            None,
        ),
        None,
    );
    let array_ty = type_tyapp(make_array_ty(), type_tyvar_star("a"));
    let scm = Scheme::generalize(
        HashMap::from([("a".to_string(), kind_star())]),
        vec![],
        type_fun(array_ty, make_i64_ty()),
    );
    (expr, scm)
}

// `get` built-in function for a given struct.
pub fn struct_get_lit(
    var_name: &str,
    field_idx: usize,
    field_ty: Rc<TypeNode>,
    struct_name: &FullName,
    field_name: &str,
) -> Rc<ExprNode> {
    let var_name_clone = FullName::local(var_name);
    let generator: Rc<InlineLLVM> = Rc::new(move |gc, _ty, rvo| {
        // Get struct object.
        let str = gc.get_var(&var_name_clone).ptr.get(gc);

        // Extract field.
        // let field = ObjectFieldType::get_struct_field(gc, &str, field_idx as u32);
        // let field_val = field.value(gc);
        // let field = if rvo.is_none() {
        //     Object::create_from_value(field_val, field.ty, gc)
        // } else {
        //     let rvo = rvo.unwrap();
        //     rvo.store_unbox(gc, field_val);
        //     rvo
        // };

        // // Retain field and release struct.
        // gc.retain(field.clone());
        // gc.release(str);

        // field

        ObjectFieldType::get_struct_fields(gc, &str, vec![(field_idx as u32, rvo)])[0].clone()
    });
    let free_vars = vec![FullName::local(var_name)];
    let name = format!(
        "{}.get_{}({})",
        struct_name.to_string(),
        field_name,
        var_name
    );
    expr_lit(generator, free_vars, name, field_ty, None)
}

// field getter function for a given struct.
pub fn struct_get(
    struct_name: &FullName,
    definition: &TypeDefn,
    field_name: &str,
) -> (Rc<ExprNode>, Rc<Scheme>) {
    // Find the index of `field_name` in the given struct.
    let field = definition.get_field_by_name(field_name);
    if field.is_none() {
        error_exit(&format!(
            "No field `{}` found in the struct `{}`.",
            &field_name,
            struct_name.to_string(),
        ));
    }
    let (field_idx, field) = field.unwrap();

    let str_ty = definition.ty();
    const VAR_NAME: &str = "str_obj";
    let expr = expr_abs(
        vec![var_local(VAR_NAME)],
        struct_get_lit(
            VAR_NAME,
            field_idx as usize,
            field.ty.clone(),
            struct_name,
            field_name,
        ),
        None,
    );
    let ty = type_fun(str_ty, field.ty.clone());
    let scm = Scheme::generalize(ty.free_vars(), vec![], ty);
    (expr, scm)
}

// `mod` built-in function for a given struct.
pub fn struct_mod_lit(
    f_name: &str,
    x_name: &str,
    field_count: usize, // number of fields in this struct
    field_idx: usize,
    struct_name: &FullName,
    struct_defn: &TypeDefn,
    field_name: &str,
    is_unique_version: bool,
) -> Rc<ExprNode> {
    let name = format!(
        "{}.mod_{}{}({}, {})",
        struct_name.to_string(),
        field_name,
        if is_unique_version { "!" } else { "" },
        f_name,
        x_name
    );
    let f_name = FullName::local(f_name);
    let x_name = FullName::local(x_name);
    let free_vars = vec![f_name.clone(), x_name.clone()];
    let generator: Rc<InlineLLVM> = Rc::new(move |gc, ty, rvo| {
        let is_unbox = ty.is_unbox(gc.type_env());

        // Get arguments
        let modfier = gc.get_var(&f_name).ptr.get(gc);
        let str = gc.get_var(&x_name).ptr.get(gc);

        let mut str = make_struct_unique(gc, str, field_count as u32, is_unique_version);

        // Modify field
        let field = ObjectFieldType::get_struct_field_noclone(gc, &str, field_idx as u32);
        let field = gc.apply_lambda(modfier, vec![field], None);
        ObjectFieldType::set_struct_field_norelease(gc, &str, field_idx as u32, &field);

        if rvo.is_some() {
            assert!(is_unbox);
            // Move str to rvo.
            let rvo = rvo.unwrap();
            let str_val = str.load_nocap(gc);
            rvo.store_unbox(gc, str_val);
            str = rvo;
        }

        str
    });
    expr_lit(generator, free_vars, name, struct_defn.ty(), None)
}

// `mod` built-in function for a given struct.
pub fn struct_mod(
    struct_name: &FullName,
    definition: &TypeDefn,
    field_name: &str,
    is_unique_version: bool,
) -> (Rc<ExprNode>, Rc<Scheme>) {
    // Find the index of `field_name` in the given struct.
    let field = definition.get_field_by_name(field_name);
    if field.is_none() {
        error_exit(&format!(
            "error: no field `{}` found in the struct `{}`.",
            &field_name,
            struct_name.to_string(),
        ));
    }
    let (field_idx, field) = field.unwrap();

    let field_count = definition.fields().len();
    let str_ty = definition.ty();
    let expr = expr_abs(
        vec![var_local("f")],
        expr_abs(
            vec![var_local("x")],
            struct_mod_lit(
                "f",
                "x",
                field_count,
                field_idx as usize,
                struct_name,
                definition,
                field_name,
                is_unique_version,
            ),
            None,
        ),
        None,
    );
    let ty = type_fun(
        type_fun(field.ty.clone(), field.ty.clone()),
        type_fun(str_ty.clone(), str_ty.clone()),
    );
    let scm = Scheme::generalize(ty.free_vars(), vec![], ty);
    (expr, scm)
}

// Make struct object to unique.
// If it is (unboxed or) unique, do nothing.
// If it is shared, clone the object or panics if panic_if_shared is true.
fn make_struct_unique<'c, 'm>(
    gc: &mut GenerationContext<'c, 'm>,
    mut str: Object<'c>,
    field_count: u32,
    panic_if_shared: bool,
) -> Object<'c> {
    let is_unbox = str.ty.is_unbox(gc.type_env());
    if !is_unbox {
        // In boxed case, str should be replaced to cloned object if it is shared.
        // In unboxed case, str is always treated as unique object.

        // Get refcnt.
        let refcnt = {
            let str_ptr = str.ptr(gc);
            gc.load_obj_field(str_ptr, control_block_type(gc), 0)
                .into_int_value()
        };

        // Add shared / cont bbs.
        let current_bb = gc.builder().get_insert_block().unwrap();
        let current_func = current_bb.get_parent().unwrap();
        let shared_bb = gc.context.append_basic_block(current_func, "shared_bb");
        let cont_bb = gc
            .context
            .append_basic_block(current_func, "unique_or_cloned_bb");

        let original_str_ptr = str.ptr(gc);

        // Jump to shared_bb if refcnt > 1.
        let one = refcnt_type(gc.context).const_int(1, false);
        let is_unique = gc
            .builder()
            .build_int_compare(IntPredicate::EQ, refcnt, one, "is_unique");
        gc.builder()
            .build_conditional_branch(is_unique, cont_bb, shared_bb);

        // In shared_bb, create new struct and clone fields.
        gc.builder().position_at_end(shared_bb);
        if panic_if_shared {
            // In case of unique version, panic in this case.
            gc.panic(&format!(
                "a struct object is asserted as unique but is shared!\n"
            ));
        }
        let cloned_str = allocate_obj(str.ty.clone(), &vec![], None, gc, Some("cloned_str"));
        for i in 0..field_count {
            // Retain field.
            let field = ObjectFieldType::get_struct_field_noclone(gc, &str, i as u32);
            gc.retain(field.clone());
            // Clone field.
            ObjectFieldType::set_struct_field_norelease(gc, &cloned_str, i as u32, &field);
        }
        gc.release(str.clone());
        let cloned_str_ptr = cloned_str.ptr(gc);
        let succ_of_shared_bb = gc.builder().get_insert_block().unwrap();
        gc.builder().build_unconditional_branch(cont_bb);

        // Implement cont_bb
        gc.builder().position_at_end(cont_bb);

        // Build phi value
        let str_phi = gc.builder().build_phi(str.ptr(gc).get_type(), "str_phi");
        str_phi.add_incoming(&[
            (&original_str_ptr, current_bb),
            (&cloned_str_ptr, succ_of_shared_bb),
        ]);

        str = Object::new(
            str_phi.as_basic_value().into_pointer_value(),
            str.ty.clone(),
        );
    }
    str
}

// `set` built-in function for a given struct.
pub fn struct_set(
    struct_name: &FullName,
    definition: &TypeDefn,
    field_name: &str,
    is_unique_version: bool,
) -> (Rc<ExprNode>, Rc<Scheme>) {
    const VALUE_NAME: &str = "val";
    const STRUCT_NAME: &str = "str";

    // Find the index of `field_name` in the given struct.
    let field = definition.get_field_by_name(field_name);
    if field.is_none() {
        error_exit(&format!(
            "No field `{}` found in the struct `{}`.",
            &field_name,
            struct_name.to_string(),
        ));
    }
    let (field_idx, field) = field.unwrap();
    let field_count = definition.fields().len() as u32;

    let generator: Rc<InlineLLVM> = Rc::new(move |gc, str_ty, rvo| {
        // Get arguments
        let value = gc.get_var(&FullName::local(VALUE_NAME)).ptr.get(gc);
        let str = gc.get_var(&FullName::local(STRUCT_NAME)).ptr.get(gc);

        // Make struct object unique.
        let mut str = make_struct_unique(gc, str, field_count, is_unique_version);

        // Release old value
        let old_value = ObjectFieldType::get_struct_field_noclone(gc, &str, field_idx as u32);
        gc.release(old_value);

        // Set new value
        ObjectFieldType::set_struct_field_norelease(gc, &str, field_idx as u32, &value);

        // If rvo, store the result to the rvo.
        if rvo.is_some() {
            assert!(str_ty.is_unbox(gc.type_env()));
            // Move str to rvo.
            let rvo = rvo.unwrap();
            let str_val = str.load_nocap(gc);
            rvo.store_unbox(gc, str_val);
            str = rvo;
        }

        str
    });

    let str_ty = definition.ty();
    let expr = expr_abs(
        vec![var_local(VALUE_NAME)],
        expr_abs(
            vec![var_local(STRUCT_NAME)],
            expr_lit(
                generator,
                vec![FullName::local(VALUE_NAME), FullName::local(STRUCT_NAME)],
                format!(
                    "{}.{}{}{}({})",
                    STRUCT_NAME,
                    SETTER_SYMBOL,
                    field_name,
                    if is_unique_version { "!" } else { "" },
                    VALUE_NAME
                ),
                str_ty.clone(),
                None,
            ),
            None,
        ),
        None,
    );
    let ty = type_fun(field.ty.clone(), type_fun(str_ty.clone(), str_ty.clone()));
    let scm = Scheme::generalize(ty.free_vars(), vec![], ty);
    (expr, scm)
}

// `new_{field}` built-in function for a given union.
pub fn union_new(
    union_name: &FullName,
    field_name: &Name,
    union: &TypeDefn,
) -> (Rc<ExprNode>, Rc<Scheme>) {
    // Get field index.
    let mut field_idx = 0;
    for field in union.fields() {
        if *field_name == field.name {
            break;
        }
        field_idx += 1;
    }
    if field_idx == union.fields().len() {
        error_exit(&format!(
            "Unknown field `{}` for union `{}`",
            field_name,
            union_name.to_string()
        ));
    }
    let expr = expr_abs(
        vec![var_local(field_name)],
        union_new_lit(union_name, union, field_name, field_idx),
        None,
    );
    let union_ty = union.ty();
    let field_ty = union.fields()[field_idx].ty.clone();
    let ty = type_fun(field_ty, union_ty);
    let scm = Scheme::generalize(ty.free_vars(), vec![], ty);
    (expr, scm)
}

// constructor function for a given union.
pub fn union_new_lit(
    union_name: &FullName,
    union_defn: &TypeDefn,
    field_name: &Name,
    field_idx: usize,
) -> Rc<ExprNode> {
    let free_vars = vec![FullName::local(field_name)];
    let name = format!("{}.new_{}", union_name.to_string(), field_name);
    let name_cloned = name.clone();
    let field_name_cloned = field_name.clone();
    let generator: Rc<InlineLLVM> = Rc::new(move |gc, ty, rvo| {
        let is_unbox = ty.is_unbox(gc.type_env());
        let offset: u32 = if is_unbox { 0 } else { 1 };

        // Get field values.
        let field = gc.get_var(&FullName::local(&field_name_cloned)).ptr.get(gc);

        // Create union object.
        let obj = if rvo.is_none() {
            allocate_obj(ty.clone(), &vec![], None, gc, Some(&name_cloned))
        } else {
            rvo.unwrap()
        };

        // Set tag value.
        let tag_value = ObjectFieldType::UnionTag
            .to_basic_type(gc)
            .into_int_type()
            .const_int(field_idx as u64, false);
        obj.store_field_nocap(gc, 0 + offset, tag_value);

        // Set value.
        let buf = obj.ptr_to_field_nocap(gc, offset + 1);
        ObjectFieldType::set_value_to_union_buf(gc, buf, field);

        obj
    });
    expr_lit(generator, free_vars, name, union_defn.ty(), None)
}

// `as_{field}` built-in function for a given union.
pub fn union_as(
    union_name: &FullName,
    field_name: &Name,
    union: &TypeDefn,
) -> (Rc<ExprNode>, Rc<Scheme>) {
    // Get field index.
    let mut field_idx = 0;
    for field in union.fields() {
        if *field_name == field.name {
            break;
        }
        field_idx += 1;
    }
    if field_idx == union.fields().len() {
        error_exit(&format!(
            "Unknown field `{}` for union `{}`",
            field_name,
            union_name.to_string()
        ));
    }
    let union_arg_name = "union".to_string();
    let expr = expr_abs(
        vec![var_local(&union_arg_name)],
        union_as_lit(
            union_name,
            &union_arg_name,
            field_name,
            field_idx,
            union.fields()[field_idx].ty.clone(),
        ),
        None,
    );
    let union_ty = union.ty();
    let field_ty = union.fields()[field_idx].ty.clone();
    let ty = type_fun(union_ty, field_ty);
    let scm = Scheme::generalize(ty.free_vars(), vec![], ty);
    (expr, scm)
}

// `as_{field}` built-in function for a given union.
pub fn union_as_lit(
    union_name: &FullName,
    union_arg_name: &Name,
    field_name: &Name,
    field_idx: usize,
    field_ty: Rc<TypeNode>,
) -> Rc<ExprNode> {
    let name = format!("{}.as_{}", union_name.to_string(), field_name);
    let free_vars = vec![FullName::local(union_arg_name)];
    let union_arg_name = union_arg_name.clone();
    let generator: Rc<InlineLLVM> = Rc::new(move |gc, ty, rvo| {
        // Get union object.
        let obj = gc.get_var(&FullName::local(&union_arg_name)).ptr.get(gc);

        let elem_ty = ty.clone();

        // Create specified tag value.
        let specified_tag_value = ObjectFieldType::UnionTag
            .to_basic_type(gc)
            .into_int_type()
            .const_int(field_idx as u64, false);

        // If tag unmatch, panic.
        ObjectFieldType::panic_if_union_tag_unmatch(gc, obj.clone(), specified_tag_value);

        // If tag match, return the field value.
        ObjectFieldType::get_union_field(gc, obj, &elem_ty, rvo)
    });
    expr_lit(generator, free_vars, name, field_ty, None)
}

// `is_{field}` built-in function for a given union.
pub fn union_is(
    union_name: &FullName,
    field_name: &Name,
    union: &TypeDefn,
) -> (Rc<ExprNode>, Rc<Scheme>) {
    // Get field index.
    let mut field_idx = 0;
    for field in union.fields() {
        if *field_name == field.name {
            break;
        }
        field_idx += 1;
    }
    if field_idx == union.fields().len() {
        error_exit(&format!(
            "Unknown field `{}` for union `{}`",
            field_name,
            union_name.to_string()
        ));
    }
    let union_arg_name = "union".to_string();
    let expr = expr_abs(
        vec![var_local(&union_arg_name)],
        union_is_lit(union_name, &union_arg_name, field_name, field_idx),
        None,
    );
    let union_ty = union.ty();
    let ty = type_fun(union_ty, make_bool_ty());
    let scm = Scheme::generalize(ty.free_vars(), vec![], ty);
    (expr, scm)
}

// `is_{field}` built-in function for a given union.
pub fn union_is_lit(
    union_name: &FullName,
    union_arg_name: &Name,
    field_name: &Name,
    field_idx: usize,
) -> Rc<ExprNode> {
    let name = format!("{}.is_{}", union_name.to_string(), field_name);
    let name_cloned = name.clone();
    let free_vars = vec![FullName::local(union_arg_name)];
    let union_arg_name = union_arg_name.clone();
    let generator: Rc<InlineLLVM> = Rc::new(move |gc, _, rvo| {
        // Get union object.
        let obj = gc.get_var(&FullName::local(&union_arg_name)).ptr.get(gc);

        let is_unbox = obj.is_unbox(gc.type_env());
        let offset = if is_unbox { 0 } else { 1 };

        // Create specified tag value.
        let specified_tag_value = ObjectFieldType::UnionTag
            .to_basic_type(gc)
            .into_int_type()
            .const_int(field_idx as u64, false);

        // Get tag value.
        let tag_value = obj.load_field_nocap(gc, 0 + offset).into_int_value();

        // Create returned value.
        let ret = if rvo.is_none() {
            allocate_obj(make_bool_ty(), &vec![], None, gc, Some(&name_cloned))
        } else {
            rvo.unwrap()
        };

        // Branch and store result to ret_ptr.
        let is_tag_match = gc.builder().build_int_compare(
            IntPredicate::EQ,
            specified_tag_value,
            tag_value,
            "is_tag_match",
        );
        let current_bb = gc.builder().get_insert_block().unwrap();
        let current_func = current_bb.get_parent().unwrap();
        let match_bb = gc.context.append_basic_block(current_func, "match_bb");
        let unmatch_bb = gc.context.append_basic_block(current_func, "unmatch_bb");
        let cont_bb = gc.context.append_basic_block(current_func, "cont_bb");
        gc.builder()
            .build_conditional_branch(is_tag_match, match_bb, unmatch_bb);

        gc.builder().position_at_end(match_bb);
        let value = gc.context.i8_type().const_int(1 as u64, false);
        ret.store_field_nocap(gc, 0, value);
        gc.builder().build_unconditional_branch(cont_bb);

        gc.builder().position_at_end(unmatch_bb);
        let value = gc.context.i8_type().const_int(0 as u64, false);
        ret.store_field_nocap(gc, 0, value);
        gc.builder().build_unconditional_branch(cont_bb);

        // Return the value.
        gc.builder().position_at_end(cont_bb);
        gc.release(obj);
        ret
    });
    expr_lit(generator, free_vars, name, make_bool_ty(), None)
}

pub fn union_mod_function(
    union_name: &FullName,
    field_name: &Name,
    union: &TypeDefn,
) -> (Rc<ExprNode>, Rc<Scheme>) {
    const UNION_NAME: &str = "union_value";
    const MODIFIER_NAME: &str = "modifier";

    let field_idx = if let Some((field_idx, _)) = union.get_field_by_name(&field_name) {
        field_idx
    } else {
        error_exit(&format!(
            "Unknown field `{}` for union `{}`",
            field_name,
            union_name.to_string()
        ));
    };

    let union_ty = union.ty();
    let field_ty = union.fields()[field_idx as usize].ty.clone();

    let generator: Rc<InlineLLVM> = Rc::new(move |gc, union_ty, rvo| {
        // Get arguments
        let obj = gc.get_var(&FullName::local(&UNION_NAME)).ptr.get(gc);
        let modifier = gc.get_var(&FullName::local(&MODIFIER_NAME)).ptr.get(gc);

        let is_unbox = obj.is_unbox(gc.type_env());
        let offset = if is_unbox { 0 } else { 1 };

        // Create specified tag value.
        let specified_tag_value = ObjectFieldType::UnionTag
            .to_basic_type(gc)
            .into_int_type()
            .const_int(field_idx as u64, false);

        // Get tag value.
        let tag_value = obj.load_field_nocap(gc, 0 + offset).into_int_value();

        // Branch and store result to ret_ptr.
        let is_tag_match = gc.builder().build_int_compare(
            IntPredicate::EQ,
            specified_tag_value,
            tag_value,
            "is_tag_match@union_mod_function",
        );
        let current_bb = gc.builder().get_insert_block().unwrap();
        let current_func = current_bb.get_parent().unwrap();
        let match_bb = gc.context.append_basic_block(current_func, "match_bb");
        let unmatch_bb = gc.context.append_basic_block(current_func, "unmatch_bb");
        let cont_bb = gc.context.append_basic_block(current_func, "cont_bb");
        gc.builder()
            .build_conditional_branch(is_tag_match, match_bb, unmatch_bb);

        // Implement match_bb
        gc.builder().position_at_end(match_bb);
        let field_ty = union_ty.field_types(gc.type_env())[field_idx as usize].clone();
        let value = ObjectFieldType::get_union_field(gc, obj.clone(), &field_ty, None);
        let value = gc.apply_lambda(modifier.clone(), vec![value], None);
        // Prepare space for returned union object.
        let ret_obj = allocate_obj(
            union_ty.clone(),
            &vec![],
            None,
            gc,
            Some("alloca@union_mod_function"),
        );
        // Set values of returned union object.
        ret_obj.store_field_nocap(gc, 0 + offset, specified_tag_value);
        let buf = ret_obj.ptr_to_field_nocap(gc, offset + 1);
        ObjectFieldType::set_value_to_union_buf(gc, buf, value);
        let match_ret_obj_ptr = ret_obj.ptr(gc);
        gc.builder().build_unconditional_branch(cont_bb);

        // Implement unmatch_bb
        gc.builder().position_at_end(unmatch_bb);
        gc.release(modifier);
        let unmatch_ret_obj_ptr = obj.ptr(gc);
        gc.builder().build_unconditional_branch(cont_bb);

        // Return the value.
        gc.builder().position_at_end(cont_bb);
        let phi = gc
            .builder()
            .build_phi(match_ret_obj_ptr.get_type(), "phi@union_mod_function");
        phi.add_incoming(&[
            (&match_ret_obj_ptr, match_bb),
            (&unmatch_ret_obj_ptr, unmatch_bb),
        ]);
        let ret_obj = Object::new(phi.as_basic_value().into_pointer_value(), union_ty.clone());
        if rvo.is_some() {
            assert!(union_ty.is_unbox(gc.type_env()));
            let rvo = rvo.unwrap();
            gc.builder().build_store(rvo.ptr(gc), ret_obj.value(gc));
            rvo
        } else {
            ret_obj
        }
    });

    let expr = expr_abs(
        vec![var_local(MODIFIER_NAME)],
        expr_abs(
            vec![var_local(UNION_NAME)],
            expr_lit(
                generator,
                vec![FullName::local(MODIFIER_NAME), FullName::local(UNION_NAME)],
                format!("mod_{}({}, {})", field_name, MODIFIER_NAME, UNION_NAME),
                union_ty.clone(),
                None,
            ),
            None,
        ),
        None,
    );
    let ty = type_fun(
        type_fun(field_ty.clone(), field_ty),
        type_fun(union_ty.clone(), union_ty),
    );
    let scm = Scheme::generalize(HashMap::default(), vec![], ty);
    (expr, scm)
}

const LOOP_RESULT_CONTINUE_IDX: usize = 0;
pub fn loop_result_defn() -> TypeDefn {
    TypeDefn {
        name: FullName::from_strs(&[STD_NAME], LOOP_RESULT_NAME),
        tyvars: vec!["s".to_string(), "b".to_string()],
        value: TypeDeclValue::Union(Union {
            fields: vec![
                Field {
                    name: "continue".to_string(),
                    ty: type_tyvar("s", &kind_star()),
                },
                Field {
                    name: "break".to_string(),
                    ty: type_tyvar("b", &kind_star()),
                },
            ],
            is_unbox: true,
        }),
    }
}

// `loop` built-in function.
// loop : s -> (s -> LoopResult s b) -> b;
pub fn state_loop() -> (Rc<ExprNode>, Rc<Scheme>) {
    const S_NAME: &str = "s";
    const B_NAME: &str = "b";
    const INITIAL_STATE_NAME: &str = "initial_state";
    const LOOP_BODY_NAME: &str = "loop_body";
    let tyvar_s = type_tyvar(S_NAME, &kind_star());
    let tyvar_b = type_tyvar(B_NAME, &kind_star());
    let scm = Scheme::generalize(
        HashMap::from([
            (S_NAME.to_string(), kind_star()),
            (B_NAME.to_string(), kind_star()),
        ]),
        vec![],
        type_fun(
            tyvar_s.clone(),
            type_fun(
                type_fun(
                    tyvar_s.clone(),
                    type_tyapp(type_tyapp(make_loop_result_ty(), tyvar_s), tyvar_b.clone()),
                ),
                tyvar_b,
            ),
        ),
    );

    let generator: Rc<InlineLLVM> = Rc::new(move |gc, ty, rvo| {
        let initial_state_name = FullName::local(INITIAL_STATE_NAME);
        let loop_body_name = FullName::local(LOOP_BODY_NAME);

        // Prepare constant.
        let cont_tag_value = ObjectFieldType::UnionTag
            .to_basic_type(gc)
            .into_int_type()
            .const_int(LOOP_RESULT_CONTINUE_IDX as u64, false);

        // Get argments.
        let init_state = gc.get_var(&initial_state_name).ptr.get(gc);
        let loop_body = gc.get_var(&loop_body_name).ptr.get(gc);

        // Collect types.
        let loop_state_ty = init_state.ty.clone();
        let loop_res_ty = loop_body.ty.get_lambda_dst();
        assert!(loop_res_ty.is_unbox(gc.type_env()));

        // Allocate a space to store LoopResult on stack.
        let loop_res = allocate_obj(loop_res_ty, &vec![], None, gc, Some("LoopResult_in_loop"));

        // If loop_state_ty is boxed, allocate a space to store loop state on stack to avoid alloca in loop body.
        let loop_state_buf = if loop_state_ty.is_unbox(gc.type_env()) {
            let ty = loop_state_ty.get_embedded_type(gc, &vec![]);
            Some(Object::new(
                gc.build_alloca_at_entry(ty, "loop_state_in_loop"),
                loop_state_ty.clone(),
            ))
        } else {
            None
        };

        // Store the initial loop state to loop_res.
        let buf = loop_res.ptr_to_field_nocap(gc, 1);
        ObjectFieldType::set_value_to_union_buf(gc, buf, init_state.clone());

        // Create loop body bb and jump to it.
        let current_bb = gc.builder().get_insert_block().unwrap();
        let current_func = current_bb.get_parent().unwrap();
        let loop_bb = gc.context.append_basic_block(current_func, "loop_bb");
        gc.builder().build_unconditional_branch(loop_bb);

        // Implement loop body.
        gc.builder().position_at_end(loop_bb);

        // Run loop_body on loop state.
        gc.retain(loop_body.clone());
        let loop_state =
            ObjectFieldType::get_union_field(gc, loop_res.clone(), &loop_state_ty, loop_state_buf);
        let _ = gc.apply_lambda(loop_body.clone(), vec![loop_state], Some(loop_res.clone()));

        // Branch due to loop_res.
        let tag_value = loop_res.load_field_nocap(gc, 0).into_int_value();
        let is_continue = gc.builder().build_int_compare(
            IntPredicate::EQ,
            tag_value,
            cont_tag_value,
            "is_continue",
        );
        let break_bb = gc.context.append_basic_block(current_func, "break_bb");
        gc.builder()
            .build_conditional_branch(is_continue, loop_bb, break_bb);

        // Implement break_bb.
        gc.builder().position_at_end(break_bb);
        gc.release(loop_body);
        ObjectFieldType::get_union_field(gc, loop_res, ty, rvo)
    });

    let initial_state_name = FullName::local(INITIAL_STATE_NAME);
    let loop_body_name = FullName::local(LOOP_BODY_NAME);
    let expr = expr_abs(
        vec![var_var(initial_state_name.clone())],
        expr_abs(
            vec![var_var(loop_body_name.clone())],
            expr_lit(
                generator,
                vec![initial_state_name, loop_body_name],
                format!("loop {} {}", INITIAL_STATE_NAME, LOOP_BODY_NAME),
                type_tyvar_star(B_NAME),
                None,
            ),
            None,
        ),
        None,
    );
    (expr, scm)
}

// `abort` built-in function
pub fn abort_function() -> (Rc<ExprNode>, Rc<Scheme>) {
    const A_NAME: &str = "a";
    const UNIT_NAME: &str = "unit";
    let generator: Rc<InlineLLVM> = Rc::new(move |gc, ty, rvo| {
        // Abort
        gc.call_runtime(RuntimeFunctions::Abort, &[]);

        // Return
        if rvo.is_some() {
            assert!(ty.is_unbox(gc.type_env()));
            rvo.unwrap()
        } else {
            allocate_obj(ty.clone(), &vec![], None, gc, Some(&"abort"))
        }
    });
    let expr = expr_abs(
        vec![var_local(UNIT_NAME)],
        expr_lit(
            generator,
            vec![],
            "abort".to_string(),
            type_tyvar_star(A_NAME),
            None,
        ),
        None,
    );
    let scm = Scheme::generalize(
        HashMap::from([(A_NAME.to_string(), kind_star())]),
        vec![],
        type_fun(make_unit_ty(), type_tyvar_star(A_NAME)),
    );
    (expr, scm)
}

pub fn tuple_defn(size: u32) -> TypeDefn {
    let tyvars = (0..size)
        .map(|i| "t".to_string() + &i.to_string())
        .collect::<Vec<_>>();
    TypeDefn {
        name: make_tuple_name(size),
        tyvars: tyvars.clone(),
        value: TypeDeclValue::Struct(Struct {
            fields: (0..size)
                .map(|i| Field {
                    name: i.to_string(),
                    ty: type_tyvar_star(&tyvars[i as usize]),
                })
                .collect(),
            is_unbox: TUPLE_UNBOX,
        }),
    }
}

pub fn unary_operator_trait(trait_id: TraitId, method_name: Name) -> TraitInfo {
    const TYVAR_NAME: &str = "a";
    let kind = kind_star();
    let tv_tyvar = tyvar_from_name(TYVAR_NAME, &kind);
    let tv_type = type_tyvar(TYVAR_NAME, &kind);
    TraitInfo {
        id: trait_id,
        type_var: tv_tyvar,
        methods: HashMap::from([(
            method_name,
            QualType {
                preds: vec![],
                kind_preds: vec![],
                ty: type_fun(tv_type.clone(), tv_type.clone()),
            },
        )]),
        kind_predicates: vec![],
    }
}

pub fn unary_opeartor_instance(
    trait_id: TraitId,
    method_name: &Name,
    operand_ty: Rc<TypeNode>,
    result_ty: Rc<TypeNode>,
    generator: for<'c, 'm> fn(
        &mut GenerationContext<'c, 'm>, // gc
        Object<'c>,                     // rhs
        Option<Object<'c>>,             // rvo
    ) -> Object<'c>,
) -> TraitInstance {
    const RHS_NAME: &str = "rhs";
    let generator: Rc<InlineLLVM> = Rc::new(move |gc, _ty, rvo| {
        let rhs_name = FullName::local(RHS_NAME);
        let rhs = gc.get_var(&rhs_name).ptr.get(gc);
        generator(gc, rhs, rvo)
    });
    TraitInstance {
        qual_pred: QualPredicate {
            context: vec![],
            kind_preds: vec![],
            predicate: Predicate::make(trait_id, operand_ty),
        },
        methods: HashMap::from([(
            method_name.to_string(),
            expr_abs(
                vec![var_local(RHS_NAME)],
                expr_lit(
                    generator,
                    vec![FullName::local(RHS_NAME)],
                    method_name.to_string(),
                    result_ty,
                    None,
                ),
                None,
            ),
        )]),
        define_module: STD_NAME.to_string(),
    }
}

pub fn binary_operator_trait(
    trait_id: TraitId,
    method_name: Name,
    output_ty: Option<Rc<TypeNode>>,
) -> TraitInfo {
    const TYVAR_NAME: &str = "a";
    let kind = kind_star();
    let tv_tyvar = tyvar_from_name(TYVAR_NAME, &kind);
    let tv_type = type_tyvar(TYVAR_NAME, &kind);
    let output_ty = match output_ty {
        Some(t) => t,
        None => tv_type.clone(),
    };
    TraitInfo {
        id: trait_id,
        type_var: tv_tyvar,
        methods: HashMap::from([(
            method_name,
            QualType {
                preds: vec![],
                kind_preds: vec![],
                ty: type_fun(tv_type.clone(), type_fun(tv_type.clone(), output_ty)),
            },
        )]),
        kind_predicates: vec![],
    }
}

pub fn binary_opeartor_instance(
    trait_id: TraitId,
    method_name: &Name,
    operand_ty: Rc<TypeNode>,
    result_ty: Rc<TypeNode>,
    generator: for<'c, 'm> fn(
        &mut GenerationContext<'c, 'm>, // gc
        Object<'c>,                     // lhs
        Object<'c>,                     // rhs
        Option<Object<'c>>,             // rvo
    ) -> Object<'c>,
) -> TraitInstance {
    const LHS_NAME: &str = "lhs";
    const RHS_NAME: &str = "rhs";
    let generator: Rc<InlineLLVM> = Rc::new(move |gc, _ty, rvo| {
        let lhs = FullName::local(LHS_NAME);
        let rhs = FullName::local(RHS_NAME);
        let lhs_val = gc.get_var(&lhs).ptr.get(gc);
        let rhs_val = gc.get_var(&rhs).ptr.get(gc);
        generator(gc, lhs_val, rhs_val, rvo)
    });
    TraitInstance {
        qual_pred: QualPredicate {
            context: vec![],
            kind_preds: vec![],
            predicate: Predicate::make(trait_id, operand_ty),
        },
        methods: HashMap::from([(
            method_name.to_string(),
            expr_abs(
                vec![var_local(LHS_NAME)],
                expr_abs(
                    vec![var_local(RHS_NAME)],
                    expr_lit(
                        generator,
                        vec![FullName::local(LHS_NAME), FullName::local(RHS_NAME)],
                        method_name.to_string(),
                        result_ty,
                        None,
                    ),
                    None,
                ),
                None,
            ),
        )]),
        define_module: STD_NAME.to_string(),
    }
}

pub const EQ_TRAIT_NAME: &str = "Eq";
pub const EQ_TRAIT_EQ_NAME: &str = "eq";

pub fn eq_trait_id() -> TraitId {
    TraitId {
        name: FullName::from_strs(&[STD_NAME], EQ_TRAIT_NAME),
    }
}

pub fn eq_trait() -> TraitInfo {
    binary_operator_trait(
        eq_trait_id(),
        EQ_TRAIT_EQ_NAME.to_string(),
        Some(make_bool_ty()),
    )
}

pub fn eq_trait_instance_int(ty: Rc<TypeNode>) -> TraitInstance {
    fn generate_eq_int<'c, 'm>(
        gc: &mut GenerationContext<'c, 'm>,
        lhs: Object<'c>,
        rhs: Object<'c>,
        rvo: Option<Object<'c>>,
    ) -> Object<'c> {
        let lhs_val = lhs.load_field_nocap(gc, 0).into_int_value();
        gc.release(lhs);
        let rhs_val = rhs.load_field_nocap(gc, 0).into_int_value();
        gc.release(rhs);
        let value =
            gc.builder()
                .build_int_compare(IntPredicate::EQ, lhs_val, rhs_val, EQ_TRAIT_EQ_NAME);
        let value = gc.builder().build_int_z_extend(
            value,
            ObjectFieldType::I8.to_basic_type(gc).into_int_type(),
            "eq",
        );
        let obj = if rvo.is_none() {
            allocate_obj(
                make_bool_ty(),
                &vec![],
                None,
                gc,
                Some(&format!("{} lhs rhs", EQ_TRAIT_EQ_NAME)),
            )
        } else {
            rvo.unwrap()
        };
        obj.store_field_nocap(gc, 0, value);
        obj
    }
    binary_opeartor_instance(
        eq_trait_id(),
        &EQ_TRAIT_EQ_NAME.to_string(),
        ty,
        make_bool_ty(),
        generate_eq_int,
    )
}

pub fn eq_trait_instance_ptr(ty: Rc<TypeNode>) -> TraitInstance {
    fn generate_eq_ptr<'c, 'm>(
        gc: &mut GenerationContext<'c, 'm>,
        lhs: Object<'c>,
        rhs: Object<'c>,
        rvo: Option<Object<'c>>,
    ) -> Object<'c> {
        let lhs_val = lhs.load_field_nocap(gc, 0).into_pointer_value();
        gc.release(lhs);
        let rhs_val = rhs.load_field_nocap(gc, 0).into_pointer_value();
        gc.release(rhs);
        let diff = gc
            .builder()
            .build_ptr_diff(lhs_val, rhs_val, "ptr_diff@eq_trait_instance_ptr");
        let value = gc.builder().build_int_compare(
            IntPredicate::EQ,
            diff,
            diff.get_type().const_zero(),
            EQ_TRAIT_EQ_NAME,
        );
        let value = gc.builder().build_int_z_extend(
            value,
            ObjectFieldType::I8.to_basic_type(gc).into_int_type(),
            "eq_of_int",
        );
        let obj = if rvo.is_none() {
            allocate_obj(
                make_bool_ty(),
                &vec![],
                None,
                gc,
                Some(&format!("{} lhs rhs", EQ_TRAIT_EQ_NAME)),
            )
        } else {
            rvo.unwrap()
        };
        obj.store_field_nocap(gc, 0, value);
        obj
    }
    binary_opeartor_instance(
        eq_trait_id(),
        &EQ_TRAIT_EQ_NAME.to_string(),
        ty,
        make_bool_ty(),
        generate_eq_ptr,
    )
}

pub fn eq_trait_instance_float(ty: Rc<TypeNode>) -> TraitInstance {
    fn generate_eq_float<'c, 'm>(
        gc: &mut GenerationContext<'c, 'm>,
        lhs: Object<'c>,
        rhs: Object<'c>,
        rvo: Option<Object<'c>>,
    ) -> Object<'c> {
        let lhs_val = lhs.load_field_nocap(gc, 0).into_float_value();
        gc.release(lhs);
        let rhs_val = rhs.load_field_nocap(gc, 0).into_float_value();
        gc.release(rhs);
        let value = gc.builder().build_float_compare(
            inkwell::FloatPredicate::OEQ,
            lhs_val,
            rhs_val,
            EQ_TRAIT_EQ_NAME,
        );
        let value = gc.builder().build_int_z_extend(
            value,
            ObjectFieldType::I8.to_basic_type(gc).into_int_type(),
            "eq_of_float",
        );
        let obj = if rvo.is_none() {
            allocate_obj(
                make_bool_ty(),
                &vec![],
                None,
                gc,
                Some(&format!("{} lhs rhs", EQ_TRAIT_EQ_NAME)),
            )
        } else {
            rvo.unwrap()
        };
        obj.store_field_nocap(gc, 0, value);
        obj
    }
    binary_opeartor_instance(
        eq_trait_id(),
        &EQ_TRAIT_EQ_NAME.to_string(),
        ty,
        make_bool_ty(),
        generate_eq_float,
    )
}

pub const LESS_THAN_TRAIT_NAME: &str = "LessThan";
pub const LESS_THAN_TRAIT_LT_NAME: &str = "less_than";

pub fn less_than_trait_id() -> TraitId {
    TraitId {
        name: FullName::from_strs(&[STD_NAME], LESS_THAN_TRAIT_NAME),
    }
}

pub fn less_than_trait() -> TraitInfo {
    binary_operator_trait(
        less_than_trait_id(),
        LESS_THAN_TRAIT_LT_NAME.to_string(),
        Some(make_bool_ty()),
    )
}

pub fn less_than_trait_instance_int(ty: Rc<TypeNode>) -> TraitInstance {
    fn generate_less_than_int<'c, 'm>(
        gc: &mut GenerationContext<'c, 'm>,
        lhs: Object<'c>,
        rhs: Object<'c>,
        rvo: Option<Object<'c>>,
    ) -> Object<'c> {
        let is_singed = lhs.ty.toplevel_tycon().unwrap().is_singned_intger();

        let lhs_val = lhs.load_field_nocap(gc, 0).into_int_value();
        gc.release(lhs);
        let rhs_val = rhs.load_field_nocap(gc, 0).into_int_value();
        gc.release(rhs);
        let value = gc.builder().build_int_compare(
            if is_singed {
                IntPredicate::SLT
            } else {
                IntPredicate::ULT
            },
            lhs_val,
            rhs_val,
            LESS_THAN_TRAIT_LT_NAME,
        );
        let value = gc.builder().build_int_z_extend(
            value,
            ObjectFieldType::I8.to_basic_type(gc).into_int_type(),
            LESS_THAN_TRAIT_LT_NAME,
        );
        let obj = if rvo.is_none() {
            allocate_obj(
                make_bool_ty(),
                &vec![],
                None,
                gc,
                Some(&format!("{} lhs rhs", LESS_THAN_TRAIT_LT_NAME)),
            )
        } else {
            rvo.unwrap()
        };
        obj.store_field_nocap(gc, 0, value);
        obj
    }
    binary_opeartor_instance(
        less_than_trait_id(),
        &LESS_THAN_TRAIT_LT_NAME.to_string(),
        ty,
        make_bool_ty(),
        generate_less_than_int,
    )
}

pub fn less_than_trait_instance_float(ty: Rc<TypeNode>) -> TraitInstance {
    fn generate_less_than_float<'c, 'm>(
        gc: &mut GenerationContext<'c, 'm>,
        lhs: Object<'c>,
        rhs: Object<'c>,
        rvo: Option<Object<'c>>,
    ) -> Object<'c> {
        let lhs_val = lhs.load_field_nocap(gc, 0).into_float_value();
        gc.release(lhs);
        let rhs_val = rhs.load_field_nocap(gc, 0).into_float_value();
        gc.release(rhs);
        let value = gc.builder().build_float_compare(
            inkwell::FloatPredicate::OLT,
            lhs_val,
            rhs_val,
            LESS_THAN_TRAIT_LT_NAME,
        );
        let value = gc.builder().build_int_z_extend(
            value,
            ObjectFieldType::I8.to_basic_type(gc).into_int_type(),
            LESS_THAN_TRAIT_LT_NAME,
        );
        let obj = if rvo.is_none() {
            allocate_obj(
                make_bool_ty(),
                &vec![],
                None,
                gc,
                Some(&format!("{} lhs rhs", LESS_THAN_TRAIT_LT_NAME)),
            )
        } else {
            rvo.unwrap()
        };
        obj.store_field_nocap(gc, 0, value);
        obj
    }
    binary_opeartor_instance(
        less_than_trait_id(),
        &LESS_THAN_TRAIT_LT_NAME.to_string(),
        ty,
        make_bool_ty(),
        generate_less_than_float,
    )
}

pub const LESS_THAN_OR_EQUAL_TO_TRAIT_NAME: &str = "LessThanOrEq";
pub const LESS_THAN_OR_EQUAL_TO_TRAIT_OP_NAME: &str = "less_than_or_eq";

pub fn less_than_or_equal_to_trait_id() -> TraitId {
    TraitId {
        name: FullName::from_strs(&[STD_NAME], LESS_THAN_OR_EQUAL_TO_TRAIT_NAME),
    }
}

pub fn less_than_or_equal_to_trait() -> TraitInfo {
    binary_operator_trait(
        less_than_or_equal_to_trait_id(),
        LESS_THAN_OR_EQUAL_TO_TRAIT_OP_NAME.to_string(),
        Some(make_bool_ty()),
    )
}

pub fn less_than_or_equal_to_trait_instance_int(ty: Rc<TypeNode>) -> TraitInstance {
    fn generate_less_than_or_equal_to_int<'c, 'm>(
        gc: &mut GenerationContext<'c, 'm>,
        lhs: Object<'c>,
        rhs: Object<'c>,
        rvo: Option<Object<'c>>,
    ) -> Object<'c> {
        let is_singed = lhs.ty.toplevel_tycon().unwrap().is_singned_intger();

        let lhs_val = lhs.load_field_nocap(gc, 0).into_int_value();
        gc.release(lhs);
        let rhs_val = rhs.load_field_nocap(gc, 0).into_int_value();
        gc.release(rhs);
        let value = gc.builder().build_int_compare(
            if is_singed {
                IntPredicate::SLE
            } else {
                IntPredicate::ULE
            },
            lhs_val,
            rhs_val,
            LESS_THAN_OR_EQUAL_TO_TRAIT_OP_NAME,
        );
        let value = gc.builder().build_int_z_extend(
            value,
            ObjectFieldType::I8.to_basic_type(gc).into_int_type(),
            LESS_THAN_OR_EQUAL_TO_TRAIT_OP_NAME,
        );
        let obj = if rvo.is_none() {
            allocate_obj(
                make_bool_ty(),
                &vec![],
                None,
                gc,
                Some(&format!("{} lhs rhs", LESS_THAN_OR_EQUAL_TO_TRAIT_OP_NAME)),
            )
        } else {
            rvo.unwrap()
        };
        obj.store_field_nocap(gc, 0, value);
        obj
    }
    binary_opeartor_instance(
        less_than_or_equal_to_trait_id(),
        &LESS_THAN_OR_EQUAL_TO_TRAIT_OP_NAME.to_string(),
        ty,
        make_bool_ty(),
        generate_less_than_or_equal_to_int,
    )
}

pub fn less_than_or_equal_to_trait_instance_float(ty: Rc<TypeNode>) -> TraitInstance {
    fn generate_less_than_or_equal_to_float<'c, 'm>(
        gc: &mut GenerationContext<'c, 'm>,
        lhs: Object<'c>,
        rhs: Object<'c>,
        rvo: Option<Object<'c>>,
    ) -> Object<'c> {
        let lhs_val = lhs.load_field_nocap(gc, 0).into_float_value();
        gc.release(lhs);
        let rhs_val = rhs.load_field_nocap(gc, 0).into_float_value();
        gc.release(rhs);
        let value = gc.builder().build_float_compare(
            inkwell::FloatPredicate::OLE,
            lhs_val,
            rhs_val,
            LESS_THAN_OR_EQUAL_TO_TRAIT_OP_NAME,
        );
        let value = gc.builder().build_int_z_extend(
            value,
            ObjectFieldType::I8.to_basic_type(gc).into_int_type(),
            LESS_THAN_OR_EQUAL_TO_TRAIT_OP_NAME,
        );
        let obj = if rvo.is_none() {
            allocate_obj(
                make_bool_ty(),
                &vec![],
                None,
                gc,
                Some(&format!("{} lhs rhs", LESS_THAN_OR_EQUAL_TO_TRAIT_OP_NAME)),
            )
        } else {
            rvo.unwrap()
        };
        obj.store_field_nocap(gc, 0, value);
        obj
    }
    binary_opeartor_instance(
        less_than_or_equal_to_trait_id(),
        &LESS_THAN_OR_EQUAL_TO_TRAIT_OP_NAME.to_string(),
        ty,
        make_bool_ty(),
        generate_less_than_or_equal_to_float,
    )
}

pub const ADD_TRAIT_NAME: &str = "Add";
pub const ADD_TRAIT_ADD_NAME: &str = "add";

pub fn add_trait_id() -> TraitId {
    TraitId {
        name: FullName::from_strs(&[STD_NAME], ADD_TRAIT_NAME),
    }
}

pub fn add_trait() -> TraitInfo {
    binary_operator_trait(add_trait_id(), ADD_TRAIT_ADD_NAME.to_string(), None)
}

pub fn add_trait_instance_int(ty: Rc<TypeNode>) -> TraitInstance {
    fn generate_add_int<'c, 'm>(
        gc: &mut GenerationContext<'c, 'm>,
        lhs: Object<'c>,
        rhs: Object<'c>,
        rvo: Option<Object<'c>>,
    ) -> Object<'c> {
        let lhs_val = lhs.load_field_nocap(gc, 0).into_int_value();
        gc.release(lhs.clone());
        let rhs_val = rhs.load_field_nocap(gc, 0).into_int_value();
        gc.release(rhs);
        let value = gc
            .builder()
            .build_int_add(lhs_val, rhs_val, ADD_TRAIT_ADD_NAME);
        let obj = if rvo.is_none() {
            allocate_obj(
                lhs.ty.clone(),
                &vec![],
                None,
                gc,
                Some(&format!("{} lhs rhs", ADD_TRAIT_ADD_NAME)),
            )
        } else {
            rvo.unwrap()
        };
        obj.store_field_nocap(gc, 0, value);
        obj
    }
    binary_opeartor_instance(
        add_trait_id(),
        &ADD_TRAIT_ADD_NAME.to_string(),
        ty.clone(),
        ty,
        generate_add_int,
    )
}

pub fn add_trait_instance_float(ty: Rc<TypeNode>) -> TraitInstance {
    fn generate_add_float<'c, 'm>(
        gc: &mut GenerationContext<'c, 'm>,
        lhs: Object<'c>,
        rhs: Object<'c>,
        rvo: Option<Object<'c>>,
    ) -> Object<'c> {
        let lhs_val = lhs.load_field_nocap(gc, 0).into_float_value();
        gc.release(lhs.clone());
        let rhs_val = rhs.load_field_nocap(gc, 0).into_float_value();
        gc.release(rhs);
        let value = gc
            .builder()
            .build_float_add(lhs_val, rhs_val, ADD_TRAIT_ADD_NAME);
        let obj = if rvo.is_none() {
            allocate_obj(
                lhs.ty.clone(),
                &vec![],
                None,
                gc,
                Some(&format!("{} lhs rhs", ADD_TRAIT_ADD_NAME)),
            )
        } else {
            rvo.unwrap()
        };
        obj.store_field_nocap(gc, 0, value);
        obj
    }
    binary_opeartor_instance(
        add_trait_id(),
        &ADD_TRAIT_ADD_NAME.to_string(),
        ty.clone(),
        ty,
        generate_add_float,
    )
}

pub const SUBTRACT_TRAIT_NAME: &str = "Sub";
pub const SUBTRACT_TRAIT_SUBTRACT_NAME: &str = "sub";

pub fn subtract_trait_id() -> TraitId {
    TraitId {
        name: FullName::from_strs(&[STD_NAME], SUBTRACT_TRAIT_NAME),
    }
}

pub fn subtract_trait() -> TraitInfo {
    binary_operator_trait(
        subtract_trait_id(),
        SUBTRACT_TRAIT_SUBTRACT_NAME.to_string(),
        None,
    )
}

pub fn subtract_trait_instance_int(ty: Rc<TypeNode>) -> TraitInstance {
    fn generate_subtract_int<'c, 'm>(
        gc: &mut GenerationContext<'c, 'm>,
        lhs: Object<'c>,
        rhs: Object<'c>,
        rvo: Option<Object<'c>>,
    ) -> Object<'c> {
        let lhs_val = lhs.load_field_nocap(gc, 0).into_int_value();
        gc.release(lhs.clone());
        let rhs_val = rhs.load_field_nocap(gc, 0).into_int_value();
        gc.release(rhs);
        let value = gc
            .builder()
            .build_int_sub(lhs_val, rhs_val, SUBTRACT_TRAIT_SUBTRACT_NAME);
        let obj = if rvo.is_none() {
            allocate_obj(
                lhs.ty.clone(),
                &vec![],
                None,
                gc,
                Some(&format!("{} lhs rhs", SUBTRACT_TRAIT_SUBTRACT_NAME)),
            )
        } else {
            rvo.unwrap()
        };
        obj.store_field_nocap(gc, 0, value);
        obj
    }
    binary_opeartor_instance(
        subtract_trait_id(),
        &SUBTRACT_TRAIT_SUBTRACT_NAME.to_string(),
        ty.clone(),
        ty,
        generate_subtract_int,
    )
}

pub fn subtract_trait_instance_float(ty: Rc<TypeNode>) -> TraitInstance {
    fn generate_subtract_float<'c, 'm>(
        gc: &mut GenerationContext<'c, 'm>,
        lhs: Object<'c>,
        rhs: Object<'c>,
        rvo: Option<Object<'c>>,
    ) -> Object<'c> {
        let lhs_val = lhs.load_field_nocap(gc, 0).into_float_value();
        gc.release(lhs.clone());
        let rhs_val = rhs.load_field_nocap(gc, 0).into_float_value();
        gc.release(rhs);
        let value = gc
            .builder()
            .build_float_sub(lhs_val, rhs_val, SUBTRACT_TRAIT_SUBTRACT_NAME);
        let obj = if rvo.is_none() {
            allocate_obj(
                lhs.ty.clone(),
                &vec![],
                None,
                gc,
                Some(&format!("{} lhs rhs", SUBTRACT_TRAIT_SUBTRACT_NAME)),
            )
        } else {
            rvo.unwrap()
        };
        obj.store_field_nocap(gc, 0, value);
        obj
    }
    binary_opeartor_instance(
        subtract_trait_id(),
        &SUBTRACT_TRAIT_SUBTRACT_NAME.to_string(),
        ty.clone(),
        ty,
        generate_subtract_float,
    )
}

pub const MULTIPLY_TRAIT_NAME: &str = "Mul";
pub const MULTIPLY_TRAIT_MULTIPLY_NAME: &str = "mul";

pub fn multiply_trait_id() -> TraitId {
    TraitId {
        name: FullName::from_strs(&[STD_NAME], MULTIPLY_TRAIT_NAME),
    }
}

pub fn multiply_trait() -> TraitInfo {
    binary_operator_trait(
        multiply_trait_id(),
        MULTIPLY_TRAIT_MULTIPLY_NAME.to_string(),
        None,
    )
}

pub fn multiply_trait_instance_int(ty: Rc<TypeNode>) -> TraitInstance {
    fn generate_multiply_int<'c, 'm>(
        gc: &mut GenerationContext<'c, 'm>,
        lhs: Object<'c>,
        rhs: Object<'c>,
        rvo: Option<Object<'c>>,
    ) -> Object<'c> {
        let lhs_val = lhs.load_field_nocap(gc, 0).into_int_value();
        gc.release(lhs.clone());
        let rhs_val = rhs.load_field_nocap(gc, 0).into_int_value();
        gc.release(rhs);
        let value = gc
            .builder()
            .build_int_mul(lhs_val, rhs_val, MULTIPLY_TRAIT_MULTIPLY_NAME);
        let obj = if rvo.is_none() {
            allocate_obj(
                lhs.ty.clone(),
                &vec![],
                None,
                gc,
                Some(&format!("{} lhs rhs", MULTIPLY_TRAIT_MULTIPLY_NAME)),
            )
        } else {
            rvo.unwrap()
        };
        obj.store_field_nocap(gc, 0, value);
        obj
    }
    binary_opeartor_instance(
        multiply_trait_id(),
        &MULTIPLY_TRAIT_MULTIPLY_NAME.to_string(),
        ty.clone(),
        ty,
        generate_multiply_int,
    )
}

pub fn multiply_trait_instance_float(ty: Rc<TypeNode>) -> TraitInstance {
    fn generate_multiply_float<'c, 'm>(
        gc: &mut GenerationContext<'c, 'm>,
        lhs: Object<'c>,
        rhs: Object<'c>,
        rvo: Option<Object<'c>>,
    ) -> Object<'c> {
        let lhs_val = lhs.load_field_nocap(gc, 0).into_float_value();
        gc.release(lhs.clone());
        let rhs_val = rhs.load_field_nocap(gc, 0).into_float_value();
        gc.release(rhs);
        let value = gc
            .builder()
            .build_float_mul(lhs_val, rhs_val, MULTIPLY_TRAIT_MULTIPLY_NAME);
        let obj = if rvo.is_none() {
            allocate_obj(
                lhs.ty.clone(),
                &vec![],
                None,
                gc,
                Some(&format!("{} lhs rhs", MULTIPLY_TRAIT_MULTIPLY_NAME)),
            )
        } else {
            rvo.unwrap()
        };
        obj.store_field_nocap(gc, 0, value);
        obj
    }
    binary_opeartor_instance(
        multiply_trait_id(),
        &MULTIPLY_TRAIT_MULTIPLY_NAME.to_string(),
        ty.clone(),
        ty,
        generate_multiply_float,
    )
}

pub const DIVIDE_TRAIT_NAME: &str = "Div";
pub const DIVIDE_TRAIT_DIVIDE_NAME: &str = "div";

pub fn divide_trait_id() -> TraitId {
    TraitId {
        name: FullName::from_strs(&[STD_NAME], DIVIDE_TRAIT_NAME),
    }
}

pub fn divide_trait() -> TraitInfo {
    binary_operator_trait(
        divide_trait_id(),
        DIVIDE_TRAIT_DIVIDE_NAME.to_string(),
        None,
    )
}

pub fn divide_trait_instance_int(ty: Rc<TypeNode>) -> TraitInstance {
    fn generate_divide_int<'c, 'm>(
        gc: &mut GenerationContext<'c, 'm>,
        lhs: Object<'c>,
        rhs: Object<'c>,
        rvo: Option<Object<'c>>,
    ) -> Object<'c> {
        let is_singed = lhs.ty.toplevel_tycon().unwrap().is_singned_intger();

        let lhs_val = lhs.load_field_nocap(gc, 0).into_int_value();
        gc.release(lhs.clone());
        let rhs_val = rhs.load_field_nocap(gc, 0).into_int_value();
        gc.release(rhs);
        let value = if is_singed {
            gc.builder()
                .build_int_signed_div(lhs_val, rhs_val, DIVIDE_TRAIT_DIVIDE_NAME)
        } else {
            gc.builder()
                .build_int_unsigned_div(lhs_val, rhs_val, DIVIDE_TRAIT_DIVIDE_NAME)
        };
        let obj = if rvo.is_none() {
            allocate_obj(
                lhs.ty.clone(),
                &vec![],
                None,
                gc,
                Some(&format!("{} lhs rhs", DIVIDE_TRAIT_DIVIDE_NAME)),
            )
        } else {
            rvo.unwrap()
        };
        obj.store_field_nocap(gc, 0, value);
        obj
    }
    binary_opeartor_instance(
        divide_trait_id(),
        &DIVIDE_TRAIT_DIVIDE_NAME.to_string(),
        ty.clone(),
        ty,
        generate_divide_int,
    )
}

pub fn divide_trait_instance_float(ty: Rc<TypeNode>) -> TraitInstance {
    fn generate_divide_float<'c, 'm>(
        gc: &mut GenerationContext<'c, 'm>,
        lhs: Object<'c>,
        rhs: Object<'c>,
        rvo: Option<Object<'c>>,
    ) -> Object<'c> {
        let lhs_val = lhs.load_field_nocap(gc, 0).into_float_value();
        gc.release(lhs.clone());
        let rhs_val = rhs.load_field_nocap(gc, 0).into_float_value();
        gc.release(rhs);
        let value = gc
            .builder()
            .build_float_div(lhs_val, rhs_val, DIVIDE_TRAIT_DIVIDE_NAME);
        let obj = if rvo.is_none() {
            allocate_obj(
                lhs.ty.clone(),
                &vec![],
                None,
                gc,
                Some(&format!("{} lhs rhs", DIVIDE_TRAIT_DIVIDE_NAME)),
            )
        } else {
            rvo.unwrap()
        };
        obj.store_field_nocap(gc, 0, value);
        obj
    }
    binary_opeartor_instance(
        divide_trait_id(),
        &DIVIDE_TRAIT_DIVIDE_NAME.to_string(),
        ty.clone(),
        ty,
        generate_divide_float,
    )
}

pub const REMAINDER_TRAIT_NAME: &str = "Rem";
pub const REMAINDER_TRAIT_REMAINDER_NAME: &str = "rem";

pub fn remainder_trait_id() -> TraitId {
    TraitId {
        name: FullName::from_strs(&[STD_NAME], REMAINDER_TRAIT_NAME),
    }
}

pub fn remainder_trait() -> TraitInfo {
    binary_operator_trait(
        remainder_trait_id(),
        REMAINDER_TRAIT_REMAINDER_NAME.to_string(),
        None,
    )
}

pub fn remainder_trait_instance_int(ty: Rc<TypeNode>) -> TraitInstance {
    fn generate_remainder_int<'c, 'm>(
        gc: &mut GenerationContext<'c, 'm>,
        lhs: Object<'c>,
        rhs: Object<'c>,
        rvo: Option<Object<'c>>,
    ) -> Object<'c> {
        let is_singed = lhs.ty.toplevel_tycon().unwrap().is_singned_intger();

        let lhs_val = lhs.load_field_nocap(gc, 0).into_int_value();
        gc.release(lhs.clone());
        let rhs_val = rhs.load_field_nocap(gc, 0).into_int_value();
        gc.release(rhs);
        let value = if is_singed {
            gc.builder()
                .build_int_signed_rem(lhs_val, rhs_val, REMAINDER_TRAIT_REMAINDER_NAME)
        } else {
            gc.builder()
                .build_int_unsigned_rem(lhs_val, rhs_val, REMAINDER_TRAIT_REMAINDER_NAME)
        };
        let obj = if rvo.is_none() {
            allocate_obj(
                lhs.ty.clone(),
                &vec![],
                None,
                gc,
                Some(&format!("{} lhs rhs", REMAINDER_TRAIT_REMAINDER_NAME)),
            )
        } else {
            rvo.unwrap()
        };
        obj.store_field_nocap(gc, 0, value);
        obj
    }
    binary_opeartor_instance(
        remainder_trait_id(),
        &REMAINDER_TRAIT_REMAINDER_NAME.to_string(),
        ty.clone(),
        ty,
        generate_remainder_int,
    )
}

pub const NEGATE_TRAIT_NAME: &str = "Neg";
pub const NEGATE_TRAIT_NEGATE_NAME: &str = "neg";

pub fn negate_trait_id() -> TraitId {
    TraitId {
        name: FullName::from_strs(&[STD_NAME], NEGATE_TRAIT_NAME),
    }
}

pub fn negate_trait() -> TraitInfo {
    unary_operator_trait(negate_trait_id(), NEGATE_TRAIT_NEGATE_NAME.to_string())
}

pub fn negate_trait_instance_int(ty: Rc<TypeNode>) -> TraitInstance {
    fn generate_negate_int<'c, 'm>(
        gc: &mut GenerationContext<'c, 'm>,
        rhs: Object<'c>,
        rvo: Option<Object<'c>>,
    ) -> Object<'c> {
        let rhs_val = rhs.load_field_nocap(gc, 0).into_int_value();
        gc.release(rhs.clone());
        let value = gc
            .builder()
            .build_int_neg(rhs_val, NEGATE_TRAIT_NEGATE_NAME);
        let obj = if rvo.is_none() {
            allocate_obj(
                rhs.ty.clone(),
                &vec![],
                None,
                gc,
                Some(&format!("{} rhs", NEGATE_TRAIT_NEGATE_NAME)),
            )
        } else {
            rvo.unwrap()
        };
        obj.store_field_nocap(gc, 0, value);
        obj
    }
    unary_opeartor_instance(
        negate_trait_id(),
        &NEGATE_TRAIT_NEGATE_NAME.to_string(),
        ty.clone(),
        ty,
        generate_negate_int,
    )
}

pub fn negate_trait_instance_float(ty: Rc<TypeNode>) -> TraitInstance {
    fn generate_negate_float<'c, 'm>(
        gc: &mut GenerationContext<'c, 'm>,
        rhs: Object<'c>,
        rvo: Option<Object<'c>>,
    ) -> Object<'c> {
        let rhs_val = rhs.load_field_nocap(gc, 0).into_float_value();
        gc.release(rhs.clone());
        let value = gc
            .builder()
            .build_float_neg(rhs_val, NEGATE_TRAIT_NEGATE_NAME);
        let obj = if rvo.is_none() {
            allocate_obj(
                rhs.ty.clone(),
                &vec![],
                None,
                gc,
                Some(&format!("{} rhs", NEGATE_TRAIT_NEGATE_NAME)),
            )
        } else {
            rvo.unwrap()
        };
        obj.store_field_nocap(gc, 0, value);
        obj
    }
    unary_opeartor_instance(
        negate_trait_id(),
        &NEGATE_TRAIT_NEGATE_NAME.to_string(),
        ty.clone(),
        ty,
        generate_negate_float,
    )
}

pub const NOT_TRAIT_NAME: &str = "Not";
pub const NOT_TRAIT_OP_NAME: &str = "not";

pub fn not_trait_id() -> TraitId {
    TraitId {
        name: FullName::from_strs(&[STD_NAME], NOT_TRAIT_NAME),
    }
}

pub fn not_trait() -> TraitInfo {
    unary_operator_trait(not_trait_id(), NOT_TRAIT_OP_NAME.to_string())
}

pub fn not_trait_instance_bool() -> TraitInstance {
    fn generate_not_bool<'c, 'm>(
        gc: &mut GenerationContext<'c, 'm>,
        rhs: Object<'c>,
        rvo: Option<Object<'c>>,
    ) -> Object<'c> {
        let rhs_val = rhs.load_field_nocap(gc, 0).into_int_value();
        gc.release(rhs);
        let bool_ty = ObjectFieldType::I8.to_basic_type(gc).into_int_type();
        let false_val = bool_ty.const_zero();
        let value =
            gc.builder()
                .build_int_compare(IntPredicate::EQ, rhs_val, false_val, NOT_TRAIT_OP_NAME);
        let value = gc
            .builder()
            .build_int_z_extend(value, bool_ty, NOT_TRAIT_OP_NAME);
        let obj = if rvo.is_none() {
            allocate_obj(
                make_bool_ty(),
                &vec![],
                None,
                gc,
                Some(&format!("{} rhs", NOT_TRAIT_OP_NAME)),
            )
        } else {
            rvo.unwrap()
        };
        obj.store_field_nocap(gc, 0, value);
        obj
    }
    unary_opeartor_instance(
        not_trait_id(),
        &NOT_TRAIT_OP_NAME.to_string(),
        make_bool_ty(),
        make_bool_ty(),
        generate_not_bool,
    )
}

// Std::is_unique : a -> (Bool, a)
pub fn is_unique_function() -> (Rc<ExprNode>, Rc<Scheme>) {
    const TYPE_NAME: &str = "a";
    const VAR_NAME: &str = "x";
    let generator: Rc<InlineLLVM> = Rc::new(move |gc, ret_ty, rvo| {
        let bool_ty = ObjectFieldType::I8.to_basic_type(gc).into_int_type();

        // Get argument
        let obj = gc.get_var(&FullName::local(VAR_NAME)).ptr.get(gc);

        // Prepare returned object.
        let ret = if rvo.is_some() {
            rvo.unwrap()
        } else {
            allocate_obj(ret_ty.clone(), &vec![], None, gc, Some("ret@is_unique"))
        };

        // Get whether argument is unique.
        let is_unique = if obj.is_box(gc.type_env()) {
            // Get refcnt.
            let refcnt = {
                let obj_ptr = obj.ptr(gc);
                gc.load_obj_field(obj_ptr, control_block_type(gc), 0)
                    .into_int_value()
            };

            // Check if obj is unique.
            let one = refcnt_type(gc.context).const_int(1, false);
            let is_unique = gc.builder().build_int_compare(
                IntPredicate::EQ,
                refcnt,
                one,
                "is_unique@is_unique",
            );

            gc.builder()
                .build_int_z_extend(is_unique, bool_ty, "int_z_extend@is_unique")
        } else {
            bool_ty.const_int(1, false)
        };
        let bool_val = make_bool_ty().get_struct_type(gc, &vec![]).get_undef();
        let bool_val = gc
            .builder()
            .build_insert_value(bool_val, is_unique, 0, "insert@is_unique")
            .unwrap();

        // Store the result
        ret.store_field_nocap(gc, 0, bool_val);
        let obj_val = obj.value(gc);
        ret.store_field_nocap(gc, 1, obj_val);

        ret
    });
    let obj_type = type_tyvar(TYPE_NAME, &kind_star());
    let ret_type = make_tuple_ty(vec![make_bool_ty(), obj_type.clone()]);
    let scm = Scheme::generalize(
        HashMap::from([(TYPE_NAME.to_string(), kind_star())]),
        vec![],
        type_fun(obj_type.clone(), ret_type.clone()),
    );
    let expr = expr_abs(
        vec![var_local(VAR_NAME)],
        expr_lit(
            generator,
            vec![FullName::local(VAR_NAME)],
            format!("is_unique({})", VAR_NAME),
            ret_type,
            None,
        ),
        None,
    );
    (expr, scm)
}
