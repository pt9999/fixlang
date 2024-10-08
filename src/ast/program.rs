use crate::ast::export_statement::{ExportStatement, ExportedFunctionType};
use crate::error::Errors;
use import::{ImportItem, ImportStatement};
use name::{FullName, Name};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, vec};

use super::*;

#[derive(Clone)]
pub struct TypeEnv {
    // List of type constructors including user-defined types.
    pub tycons: Arc<HashMap<TyCon, TyConInfo>>,
    // List of type aliases.
    pub aliases: Arc<HashMap<TyCon, TyAliasInfo>>,
}

impl Default for TypeEnv {
    fn default() -> Self {
        Self {
            tycons: Arc::new(Default::default()),
            aliases: Arc::new(Default::default()),
        }
    }
}

impl TypeEnv {
    pub fn new(tycons: HashMap<TyCon, TyConInfo>, aliases: HashMap<TyCon, TyAliasInfo>) -> TypeEnv {
        TypeEnv {
            tycons: Arc::new(tycons),
            aliases: Arc::new(aliases),
        }
    }

    pub fn kinds(&self) -> HashMap<TyCon, Arc<Kind>> {
        let mut res = HashMap::default();
        for (tc, ti) in self.tycons.as_ref().iter() {
            res.insert(tc.clone(), ti.kind.clone());
        }
        for (tc, ta) in self.aliases.as_ref().iter() {
            res.insert(tc.clone(), ta.kind.clone());
        }
        res
    }
}

#[derive(Clone)]
pub struct InstantiatedSymbol {
    pub instantiated_name: FullName,
    pub generic_name: FullName,
    pub ty: Arc<TypeNode>,
    pub expr: Option<Arc<ExprNode>>,
}

impl InstantiatedSymbol {
    // The set of modules that this symbol depends on.
    // If any of these modules, or any of their importee are changed, then they are required to be re-compiled.
    // Note that this set may not be fully spanned in the importing graph.
    pub fn dependent_modules(&self) -> HashSet<Name> {
        let mut dep_mods = HashSet::default();
        dep_mods.insert(self.instantiated_name.module());
        self.ty.define_modules_of_tycons(&mut dep_mods);
        dep_mods
        // Even for implemented trait methods, it is enough to add the module where the trait is defined and the modules where the types of the symbol are defined.
        // This is because,
        // - By orphan rule, trait implementations are given in the module where the trait is defined, or the module where the type is defined.
        // - Moreover, we forbid unrelated trait implementation (see `test_unrelated_trait_method()`),
        // so the type the trait is implemented appears in the type of the symbol.
    }
}

// Declaration (name and type signature) of global value.
// `main : IO()`
pub struct GlobalValueDecl {
    pub name: FullName,
    pub ty: Arc<Scheme>,
    pub src: Option<Span>,
}

// Definition (name and expression)
// `main = println("Hello World")`
pub struct GlobalValueDefn {
    pub name: FullName,
    pub expr: Arc<ExprNode>,
    pub src: Option<Span>,
}

// The global value, which is either a value or trait method.
pub struct GlobalValue {
    // Type of this symbol.
    // For example, in case `trait a : Show { show : a -> String; }`, the type of method `show` is `[a : Show] a -> String`.
    pub scm: Arc<Scheme>,
    // The expression or implementation of this value.
    pub expr: SymbolExpr,
    // Source code where this value is defined.
    // For trait methods, this is the source code where the trait method is defined.
    pub def_src: Option<Span>,
    // The document of this value.
    // If `def_src` is available, we can also get document from the source code.
    // We use this field only when document is not available in the source code.
    pub document: Option<String>,
}

impl GlobalValue {
    pub fn resolve_namespace_in_declaration(
        &mut self,
        ctx: &NameResolutionContext,
    ) -> Result<(), Errors> {
        // If this function is called for methods, we must call resolve_namespace on MethodImpl.ty.
        assert!(matches!(self.expr, SymbolExpr::Simple(_)));
        self.scm = self.scm.resolve_namespace(ctx)?;
        Ok(())
    }

    pub fn resolve_type_aliases(&mut self, type_env: &TypeEnv) -> Result<(), Errors> {
        self.scm = self.scm.resolve_type_aliases(type_env)?;
        self.expr.resolve_type_aliases(type_env)?;
        Ok(())
    }

    pub fn set_kinds(&mut self, kind_env: &KindEnv) -> Result<(), Errors> {
        self.scm = self.scm.set_kinds(kind_env)?;
        self.scm.check_kinds(kind_env)?;
        match &mut self.expr {
            SymbolExpr::Simple(_) => {}
            SymbolExpr::Method(ms) => {
                for m in ms {
                    m.ty = m.ty.set_kinds(kind_env)?;
                    m.ty.check_kinds(kind_env)?;
                }
            }
        }
        Ok(())
    }

    // Check if this value is a simple value, not a trait method.
    pub fn is_simple_value(&self) -> bool {
        matches!(self.expr, SymbolExpr::Simple(_))
    }

    // Get the document of this value.
    pub fn get_document(&self) -> Option<String> {
        // Try to get document from the source code.
        let docs = self
            .def_src
            .as_ref()
            .and_then(|src| src.get_document().ok());

        // If the documentation is empty, treat it as None.
        let docs = match docs {
            Some(docs) if docs.is_empty() => None,
            _ => docs,
        };

        // If the document is not available in the source code, use the document field.
        let docs = match docs {
            Some(_) => docs,
            None => self.document.clone(),
        };

        // Again, if the documentation is empty, treat it as None.
        match docs {
            Some(docs) if docs.is_empty() => None,
            _ => docs,
        }
    }

    // Find the minimum node which includes the specified source code position.
    pub fn find_node_at(&self, pos: &SourcePos) -> Option<EndNode> {
        let node = self.expr.find_node_at(pos);
        if node.is_some() {
            return node;
        }
        self.scm.find_node_at(pos)
    }
}

// Expression of global symbol.
#[derive(Clone)]
pub enum SymbolExpr {
    Simple(TypedExpr),       // Definition such as "id : a -> a; id = \x -> x".
    Method(Vec<MethodImpl>), // Trait method implementations.
}

impl SymbolExpr {
    pub fn resolve_type_aliases(&mut self, type_env: &TypeEnv) -> Result<(), Errors> {
        match self {
            SymbolExpr::Simple(_) => Ok(()),
            SymbolExpr::Method(impls) => {
                let mut errors = Errors::empty();
                for method_impl in impls {
                    errors.eat_err(method_impl.resolve_type_aliases(type_env));
                }
                errors.to_result()
            }
        }
    }

    #[allow(dead_code)]
    pub fn source(&self) -> Option<Span> {
        match self {
            SymbolExpr::Simple(e) => e.expr.source.clone(),
            SymbolExpr::Method(ms) => ms.first().map(|m| m.expr.expr.source.clone()).flatten(),
        }
    }

    // Find the minimum expression node which includes the specified source code position.
    pub fn find_node_at(&self, pos: &SourcePos) -> Option<EndNode> {
        match self {
            SymbolExpr::Simple(e) => e.find_node_at(pos),
            SymbolExpr::Method(ms) => ms.iter().filter_map(|m| m.find_node_at(pos)).next(),
        }
    }
}

// Pair of expression and type resolver for it.
#[derive(Clone, Serialize, Deserialize)]
pub struct TypedExpr {
    pub expr: Arc<ExprNode>,
    pub substitution: Substitution,
}

impl TypedExpr {
    pub fn from_expr(expr: Arc<ExprNode>) -> Self {
        TypedExpr {
            expr,
            substitution: Substitution::default(),
        }
    }

    pub fn calculate_free_vars(&mut self) {
        self.expr = calculate_free_vars(self.expr.clone());
    }

    // Find the minimum expression node which includes the specified source code position.
    pub fn find_node_at(&self, pos: &SourcePos) -> Option<EndNode> {
        let node = self.expr.find_node_at(pos);
        if node.is_none() {
            return None;
        }
        let mut node = node.unwrap();
        match &mut node {
            EndNode::Expr(_, ty) => {
                if let Some(ty) = ty {
                    *ty = self.substitution.substitute_type(ty);
                }
            }
            EndNode::Pattern(_, ty) => {
                if let Some(ty) = ty {
                    *ty = self.substitution.substitute_type(ty);
                }
            }
            EndNode::Type(_) => {}
            EndNode::Trait(_) => {}
        }
        Some(node)
    }
}

// Trait method implementation
#[derive(Clone)]
pub struct MethodImpl {
    // Type of this method.
    // For example, in case "impl [a: Show, b: Show] (a, b): Show {...}",
    // the type of method "show" is "[a: Show, b: Show] (a, b) -> String",
    pub ty: Arc<Scheme>,
    // Expression of this implementation
    pub expr: TypedExpr,
    // Module where this implmentation is given.
    // NOTE:
    // For trait method, `define_module` may differ to the first component of namespace of the function.
    // For example, if `Main` module implements `SomeType : Eq`, then implementation of `eq` for `SomeType` is defined in `Main` module,
    // but its name as a function is still `Std::Eq::eq`.
    pub define_module: Name,
}

impl MethodImpl {
    pub fn resolve_type_aliases(&mut self, type_env: &TypeEnv) -> Result<(), Errors> {
        self.ty = self.ty.resolve_type_aliases(type_env)?;
        Ok(())
    }

    // Find the minimum expression node which includes the specified source code position.
    pub fn find_node_at(&self, pos: &SourcePos) -> Option<EndNode> {
        self.expr.find_node_at(pos)
    }
}

pub struct NameResolutionContext {
    pub candidates: HashMap<FullName, NameResolutionType>,
    pub assoc_ty_to_arity: HashMap<FullName, usize>,
    pub import_statements: Vec<ImportStatement>,
}

impl<'a> NameResolutionContext {
    pub fn new(
        tycon_names_with_aliases: &HashSet<FullName>,
        trait_names_with_aliases: &HashSet<FullName>,
        assoc_ty_to_arity: HashMap<FullName, usize>,
        import_statements: Vec<ImportStatement>,
    ) -> Self {
        let mut candidates: HashMap<FullName, NameResolutionType> = HashMap::new();
        fn check_insert(
            candidates: &mut HashMap<FullName, NameResolutionType>,
            name: FullName,
            nrt: NameResolutionType,
        ) {
            assert!(!candidates.contains_key(&name) || candidates[&name] == nrt); // This is assured by `validate_capital_name_confliction`.
            candidates.insert(name, nrt);
        }
        for name in tycon_names_with_aliases {
            check_insert(&mut candidates, name.clone(), NameResolutionType::TyCon);
        }
        for name in trait_names_with_aliases {
            check_insert(&mut candidates, name.clone(), NameResolutionType::Trait);
        }
        for (name, _arity) in &assoc_ty_to_arity {
            check_insert(&mut candidates, name.clone(), NameResolutionType::AssocTy);
        }
        NameResolutionContext {
            candidates,
            import_statements,
            assoc_ty_to_arity,
        }
    }

    pub fn resolve(
        &self,
        short_name: &FullName,
        accept_types: &[NameResolutionType],
        span: &Option<Span>,
    ) -> Result<FullName, Errors> {
        let accept_type_string = accept_types
            .iter()
            .map(|nrt| nrt.to_string())
            .collect::<Vec<_>>()
            .join(" or ");
        let candidates = self
            .candidates
            .iter()
            .filter_map(|(full_name, nrt)| {
                if !import::is_accessible(&self.import_statements, full_name) {
                    None
                } else if !accept_types.contains(nrt) {
                    None
                } else if !short_name.is_suffix(full_name) {
                    None
                } else {
                    Some(full_name.clone())
                }
            })
            .collect::<Vec<_>>();
        if candidates.len() == 0 {
            Err(Errors::from_msg_srcs(
                format!(
                    "Unknown {} name `{}`.",
                    accept_type_string,
                    short_name.to_string()
                ),
                &[span],
            ))
        } else if candidates.len() == 1 {
            Ok(candidates[0].clone())
        } else {
            // candidates.len() >= 2
            let mut candidates = candidates
                .iter()
                .map(|id| "`".to_string() + &id.to_string() + "`")
                .collect::<Vec<_>>();
            candidates.sort(); // Sort for deterministic error message.
            let candidates = candidates.join(", ");
            let msg = format!(
                "Name `{}` is ambiguous. There are {}.",
                short_name.to_string(),
                candidates
            );
            Err(Errors::from_msg_srcs(msg, &[span]))
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum NameResolutionType {
    TyCon,
    Trait,
    AssocTy,
}

impl NameResolutionType {
    pub fn to_string(&self) -> &'static str {
        match self {
            NameResolutionType::TyCon => "type",
            NameResolutionType::Trait => "trait",
            NameResolutionType::AssocTy => "associated type",
        }
    }
}

// Module information.
#[derive(Clone)]
pub struct ModuleInfo {
    // Module name.
    pub name: Name,
    // Source code location where this module is defined.
    pub source: Span,
}

// Program of fix a collection of modules.
// A program can link another program which consists of a single module.
pub struct Program {
    /* AST */
    // Global values.
    pub global_values: HashMap<FullName, GlobalValue>,
    // Type definitions.
    pub type_defns: Vec<TypeDefn>,
    // Type environment, which is calculated from `type_defns` once and cached.
    pub type_env: TypeEnv,
    // Trait environment.
    pub trait_env: TraitEnv,
    // Export statements.
    pub export_statements: Vec<ExportStatement>,
    // List of tuple sizes used in this program.
    pub used_tuple_sizes: Vec<u32>,
    // Import statements.
    // Key is the name of the importer module.
    // Each module implicitly imports itself.
    // This is used to namespace resolution and overloading resolution.
    pub mod_to_import_stmts: HashMap<Name, Vec<ImportStatement>>,

    /* Instantiation */
    // Instantiated symbols.
    pub instantiated_symbols: HashMap<FullName, InstantiatedSymbol>,
    // Deferred instantiation, which is a state variable for the instantiation process.
    pub deferred_instantiation: Vec<InstantiatedSymbol>,

    /* Dependency information */
    pub modules: Vec<ModuleInfo>,
}

impl Program {
    // Get the list of module names from a list of files.
    pub fn modules_from_files(&self, files: &[PathBuf]) -> Vec<Name> {
        let mut mod_names = vec![];
        for mod_info in &self.modules {
            if files
                .iter()
                .any(|f| to_absolute_path(f) == to_absolute_path(&mod_info.source.input.file_path))
            {
                mod_names.push(mod_info.name.clone());
            }
        }
        mod_names
    }

    // Create a program consists of single module.
    pub fn single_module(mod_info: ModuleInfo) -> Program {
        let mut fix_mod = Program {
            mod_to_import_stmts: Default::default(),
            type_defns: Default::default(),
            global_values: Default::default(),
            instantiated_symbols: Default::default(),
            deferred_instantiation: Default::default(),
            trait_env: Default::default(),
            type_env: Default::default(),
            used_tuple_sizes: (0..=TUPLE_SIZE_BASE).filter(|i| *i != 1).collect(),
            modules: Default::default(),
            export_statements: vec![],
        };
        fix_mod.add_import_statement_no_verify(ImportStatement::implicit_self_import(
            mod_info.name.clone(),
        ));
        fix_mod.add_import_statement_no_verify(ImportStatement::implicit_std_import(
            mod_info.name.clone(),
        ));
        fix_mod.modules.push(mod_info);
        fix_mod
    }

    // Add `Std::TupleN` type
    fn add_tuple_defn(&mut self, tuple_size: u32) {
        self.type_defns.push(tuple_defn(tuple_size));
    }

    // Add `Std::TupleN` type for each `n` in `used_tuple_sizes`.
    pub fn add_tuple_defns(&mut self) {
        // Make elements of used_tuple_sizes unique.
        self.used_tuple_sizes.sort();
        self.used_tuple_sizes.dedup();
        let used_tuple_sizes = std::mem::replace(&mut self.used_tuple_sizes, vec![]);
        for tuple_size in &used_tuple_sizes {
            self.add_tuple_defn(*tuple_size);
        }
        self.used_tuple_sizes = used_tuple_sizes;
    }

    // If this program consists of single module, returns its name.
    pub fn get_name_if_single_module(&self) -> Name {
        let linked_mods = self.linked_mods();
        if linked_mods.len() == 1 {
            return linked_mods.into_iter().next().unwrap();
        }
        panic!("")
    }

    pub fn is_linked(&self, mod_name: &Name) -> bool {
        self.mod_to_import_stmts.contains_key(mod_name)
    }

    // Add import statements.
    pub fn add_import_statements(&mut self, imports: Vec<ImportStatement>) -> Result<(), Errors> {
        let mut errors = Errors::empty();
        for stmt in imports {
            errors.eat_err(self.add_import_statement(stmt));
        }
        errors.to_result()
    }

    // Add an import statement.
    pub fn add_import_statement(
        &mut self,
        import_statement: ImportStatement,
    ) -> Result<(), Errors> {
        // Refuse importing the module itself.
        if import_statement.module.0 == import_statement.importer {
            return Err(Errors::from_msg_srcs(
                format!(
                    "Module `{}` cannot import itself.",
                    import_statement.module.0.to_string()
                ),
                &[&import_statement.source],
            ));
        }

        // When user imports `Std` explicitly, remove implicit `Std` import statement.
        if import_statement.module.0 == STD_NAME {
            let stmts = self
                .mod_to_import_stmts
                .get_mut(&import_statement.importer)
                .unwrap();
            *stmts = std::mem::replace(stmts, vec![])
                .into_iter()
                .filter(|stmt| !(stmt.module.0 == STD_NAME && stmt.implicit))
                .collect();
        }

        self.add_import_statement_no_verify(import_statement);

        Ok(())
    }

    pub fn add_import_statement_no_verify(&mut self, import_statement: ImportStatement) {
        let importer = &import_statement.importer;
        if let Some(stmts) = self.mod_to_import_stmts.get_mut(importer) {
            stmts.push(import_statement);
        } else {
            self.mod_to_import_stmts
                .insert(importer.clone(), vec![import_statement]);
        }
    }

    pub fn import_statements(&self) -> Vec<ImportStatement> {
        self.mod_to_import_stmts
            .values()
            .flat_map(|stmts| stmts.iter())
            .cloned()
            .collect()
    }

    // Add traits.
    pub fn add_traits(
        &mut self,
        trait_infos: Vec<TraitInfo>,
        trait_impls: Vec<TraitInstance>,
        trait_aliases: Vec<TraitAlias>,
    ) -> Result<(), Errors> {
        self.trait_env.add(trait_infos, trait_impls, trait_aliases)
    }

    // Register declarations of user-defined types.
    pub fn add_type_defns(&mut self, mut type_defns: Vec<TypeDefn>) {
        self.type_defns.append(&mut type_defns);
    }

    // Calculate list of type constructors including user-defined types.
    pub fn calculate_type_env(&mut self) -> Result<(), Errors> {
        let mut errors = Errors::empty();
        let mut tycons = bulitin_tycons();
        let mut aliases: HashMap<TyCon, TyAliasInfo> = HashMap::new();
        for type_decl in &mut self.type_defns {
            // Set kinds of type variables in the right hand side of type definition.
            type_decl.set_kinds_in_value();

            // Check duplicate type definition.
            let tycon = type_decl.tycon();
            if tycons.contains_key(&tycon) || aliases.contains_key(&tycon) {
                let other_src = if tycons.contains_key(&tycon) {
                    let tc = tycons.get(&tycon).unwrap();
                    tc.source.clone()
                } else {
                    let ta = aliases.get(&tycon).unwrap();
                    ta.source.clone()
                };
                errors.append(Errors::from_msg_srcs(
                    format!("Duplicate definitions of type `{}`.", tycon.to_string()),
                    &[
                        &type_decl.source.as_ref().map(|s| s.to_head_character()),
                        &other_src.as_ref().map(|s| s.to_head_character()),
                    ],
                ));
                continue;
            }
            if type_decl.is_alias() {
                aliases.insert(tycon.clone(), type_decl.alias_info());
            } else {
                tycons.insert(tycon.clone(), type_decl.tycon_info(&[]));
            }
            // If the type is a boxed struct, add punched struct types to tycons.
            if let TypeDeclValue::Struct(s) = &type_decl.value {
                for i in 0..s.fields.len() {
                    let mut punched_tycon = tycon.clone();
                    punched_tycon.into_punched_type_name(i);
                    tycons.insert(punched_tycon, type_decl.tycon_info(&[i]));
                }
            }
        }
        // Create type environment.
        self.type_env = TypeEnv::new(tycons, aliases);

        errors.to_result()
    }

    // Get list of type constructors including user-defined types.
    pub fn type_env(&self) -> TypeEnv {
        self.type_env.clone()
    }

    // Get of list of tycons that can be used for namespace resolution.
    pub fn tycon_names_with_aliases(&self) -> HashSet<FullName> {
        let mut res: HashSet<FullName> = Default::default();
        for (k, _) in self.type_env().tycons.iter() {
            res.insert(k.name.clone());
        }
        for (k, _) in self.type_env().aliases.iter() {
            res.insert(k.name.clone());
        }
        res
    }

    pub fn assoc_ty_to_arity(&self) -> HashMap<FullName, usize> {
        self.trait_env.assoc_ty_to_arity()
    }

    // Get of list of traits that can be used for namespace resolution.
    pub fn trait_names_with_aliases(&self) -> HashSet<FullName> {
        self.trait_env.trait_names()
    }

    // Add a global value.
    pub fn add_global_value(
        &mut self,
        name: FullName,
        (expr, scm): (Arc<ExprNode>, Arc<Scheme>),
        def_src: Option<Span>,
        document: Option<String>,
    ) -> Result<(), Errors> {
        let gv = GlobalValue {
            scm,
            expr: SymbolExpr::Simple(TypedExpr::from_expr(expr)),
            def_src,
            document,
        };
        self.add_global_value_gv(name, gv)
    }

    // Add a global value.
    pub fn add_global_value_gv(&mut self, name: FullName, gv: GlobalValue) -> Result<(), Errors> {
        // Check duplicate definition.
        if self.global_values.contains_key(&name) {
            let this = gv.def_src.map(|s| s.to_head_character());
            let other = self
                .global_values
                .get(&name)
                .unwrap()
                .def_src
                .as_ref()
                .map(|s| s.to_head_character());
            return Err(Errors::from_msg_srcs(
                format!(
                    "Duplicated definition for global value: `{}`",
                    name.to_string()
                ),
                &[&this, &other],
            ));
        }
        self.global_values.insert(name, gv);
        Ok(())
    }

    // Add global values.
    pub fn add_global_values(
        &mut self,
        exprs: Vec<GlobalValueDefn>,
        types: Vec<GlobalValueDecl>,
    ) -> Result<(), Errors> {
        let mut errors = Errors::empty();

        struct GlobalValue {
            defn: Option<GlobalValueDefn>,
            decl: Option<GlobalValueDecl>,
        }
        let mut global_values: HashMap<FullName, GlobalValue> = Default::default();

        // Register definitions checking duplication.
        for defn in exprs {
            if !global_values.contains_key(&defn.name) {
                global_values.insert(
                    defn.name.clone(),
                    GlobalValue {
                        defn: Some(defn),
                        decl: None,
                    },
                );
            } else {
                let gv = global_values.get_mut(&defn.name).unwrap();
                if gv.defn.is_some() {
                    errors.append(Errors::from_msg_srcs(
                        format!(
                            "Duplicate definition for global value: `{}`.",
                            defn.name.to_string()
                        ),
                        &[
                            &defn.src.map(|s| s.to_head_character()),
                            &gv.defn
                                .as_ref()
                                .unwrap()
                                .src
                                .as_ref()
                                .map(|s| s.to_head_character()),
                        ],
                    ));
                } else {
                    gv.defn = Some(defn);
                }
            }
        }

        // Register declarations checking duplication.
        for decl in types {
            if !global_values.contains_key(&decl.name) {
                global_values.insert(
                    decl.name.clone(),
                    GlobalValue {
                        decl: Some(decl),
                        defn: None,
                    },
                );
            } else {
                let gv = global_values.get_mut(&decl.name).unwrap();
                if gv.decl.is_some() {
                    errors.append(Errors::from_msg_srcs(
                        format!("Duplicate declaration for `{}`.", decl.name.to_string()),
                        &[
                            &decl.src.map(|s| s.to_head_character()),
                            &gv.decl
                                .as_ref()
                                .unwrap()
                                .src
                                .as_ref()
                                .map(|s| s.to_head_character()),
                        ],
                    ));
                } else {
                    gv.decl = Some(decl);
                }
            }
        }

        // Check that definitions and declarations are paired.
        for (name, gv) in global_values {
            if gv.defn.is_none() {
                errors.append(Errors::from_msg_srcs(
                    format!("Global value `{}` lacks its expression.", name.to_string()),
                    &[&gv.decl.unwrap().src.as_ref().map(|s| s.to_head_character())],
                ));
            } else if gv.decl.is_none() {
                errors.append(Errors::from_msg_srcs(
                    format!(
                        "Global value `{}` lacks its type signature.",
                        name.to_string()
                    ),
                    &[&gv.defn.unwrap().src.as_ref().map(|s| s.to_head_character())],
                ));
            } else {
                let decl_src = gv.decl.as_ref().unwrap().src.clone();
                errors.eat_err(self.add_global_value(
                    name,
                    (gv.defn.unwrap().expr, gv.decl.unwrap().ty),
                    decl_src,
                    None,
                ));
            }
        }

        errors.to_result()
    }

    // This function performs the following tasks:
    // - Resolve namespace of type and traits in the expression.
    // - Resolve type aliases in the expression.
    // - Perform typechecking.
    //
    // Parameters:
    // - `te` : The expression to be namespace-resolved and type-checked.
    // - `req_scm` : The type scheme that the expression should have.
    // - `val_name` : The name of the expression, e.g., `Std::ToString::to_string`.
    // - `def_mod` : The module where the expression is defined. Note that if `te` is a trait method implementation, this may differ from `name.module()`.
    // - `nrctx` : The name resolution context. Pass one created by `program.create_name_resolution_context(define_module)`.
    // - `ver_hash` : A hash value of source codes `te` depends on. This is used to detect or invalidate the cache file. Pass one created by `program.module_dependency_hash(define_module)`.
    fn resolve_namespace_and_check_type_sub(
        mut te: TypedExpr,
        req_scm: &Arc<Scheme>,
        val_name: &FullName,
        def_mod: &Name,
        nrctx: &NameResolutionContext,
        ver_hash: &str,
        mut tc: TypeCheckContext,
    ) -> Result<TypedExpr, Errors> {
        // Load type-checking cache file.
        let cache = tc.cache.load_cache(val_name, req_scm, ver_hash);
        if cache.is_some() {
            // If cache is available,
            te = cache.unwrap();
            return Ok(te);
        }

        // Perform namespace inference.
        te.expr = te.expr.resolve_namespace(&nrctx)?;

        // Resolve type aliases in expression.
        te.expr = te.expr.resolve_type_aliases(&tc.type_env)?;

        // Perform type-checking.
        tc.current_module = Some(def_mod.clone());
        te.expr = tc.check_type(te.expr.clone(), req_scm.clone())?;
        te.substitution = tc.substitution;

        // Save the result to cache file.
        tc.cache.save_cache(&te, val_name, req_scm, ver_hash);

        Ok(te)
    }

    // Create NameResolutionContext used for symbols defined in the specified module.
    pub fn create_name_resolution_context(&self, mod_name: &Name) -> NameResolutionContext {
        NameResolutionContext::new(
            &self.tycon_names_with_aliases(),
            &self.trait_names_with_aliases(),
            self.assoc_ty_to_arity(),
            self.mod_to_import_stmts[mod_name].clone(),
        )
    }

    pub fn resolve_namespace_and_check_type_in_modules(
        &mut self,
        tc: &TypeCheckContext,
        modules: &[Name],
    ) -> Result<(), Errors> {
        let mut errors = Errors::empty();

        // Names of global values to be checked.
        let mut checked_names: Vec<FullName> = vec![];
        for (name, gv) in self.global_values.iter() {
            match gv.expr {
                SymbolExpr::Simple(_) => {
                    // Check simple values only if they are in `modules`.
                    if modules.contains(&name.module()) {
                        checked_names.push(name.clone());
                    }
                }
                SymbolExpr::Method(_) => {
                    // We filter methods by `method_impl_filter`.
                    checked_names.push(name.clone());
                }
            }
        }

        // Method implementations to be checked.
        let modules = modules.to_vec();
        let method_impl_filter = |method: &MethodImpl| Ok(modules.contains(&method.define_module));

        errors.eat_err(self.resolve_namespace_and_check_type(
            tc,
            &checked_names,
            method_impl_filter,
        ));
        errors.to_result()
    }

    // Perform namespace resolution and type-checking for the specified expression.
    // This function updates `TypedExpr` in `self.global_values` in-place.
    pub fn resolve_namespace_and_check_type(
        &mut self,
        tc: &TypeCheckContext,
        val_names: &[FullName],
        method_impl_filter: impl Fn(&MethodImpl) -> Result<bool, Errors>,
    ) -> Result<(), Errors> {
        struct CheckTask {
            val_name: FullName,
            task: Box<dyn FnOnce() -> Result<TypedExpr, Errors> + Send>,
            method_impl_idx: Option<usize>,
        }
        let mut tasks: Vec<CheckTask> = vec![];

        let mut mod_to_nrctx: HashMap<Name, Arc<NameResolutionContext>> = HashMap::new();
        let mut get_nrctx = |mod_name: &Name| -> Arc<NameResolutionContext> {
            if !mod_to_nrctx.contains_key(mod_name) {
                mod_to_nrctx.insert(
                    mod_name.clone(),
                    Arc::new(self.create_name_resolution_context(mod_name)),
                );
            }
            mod_to_nrctx.get(mod_name).unwrap().clone()
        };

        // Create tasks.
        for val_name in val_names {
            let gv = self.global_values.get(&val_name).unwrap();
            match &gv.expr {
                SymbolExpr::Simple(e) => {
                    // Create a task for simple value.
                    let te = e.clone();
                    let scm = gv.scm.clone();
                    let val_name_clone = val_name.clone();
                    let def_mod = val_name_clone.module();
                    let nrctx = get_nrctx(&def_mod);
                    let ver_hash = self.module_dependency_hash(&def_mod);
                    let tc = tc.clone();
                    let task = Box::new(move || -> Result<TypedExpr, Errors> {
                        // Perform type-checking.
                        let mut te = Program::resolve_namespace_and_check_type_sub(
                            te,
                            &scm,
                            &val_name_clone,
                            &def_mod,
                            &nrctx,
                            &ver_hash,
                            tc,
                        )?;
                        // Calculate free vars.
                        te.calculate_free_vars();
                        Ok(te)
                    });

                    tasks.push(CheckTask {
                        val_name: val_name.clone(),
                        task,
                        method_impl_idx: None,
                    });
                }
                SymbolExpr::Method(impls) => {
                    for (i, method) in impls.iter().enumerate() {
                        // Select method implementation.
                        if !method_impl_filter(method)? {
                            continue;
                        }

                        // Create a task for method implementation.
                        let te = method.expr.clone();
                        let method_ty = method.ty.clone();
                        let val_name_clone = val_name.clone();
                        let def_mod = method.define_module.clone();
                        let nrctx = get_nrctx(&def_mod);
                        let ver_hash = self.module_dependency_hash(&def_mod);
                        let tc = tc.clone();
                        let task = Box::new(move || -> Result<TypedExpr, Errors> {
                            // Perform type-checking.
                            let mut te = Program::resolve_namespace_and_check_type_sub(
                                te,
                                &method_ty,
                                &val_name_clone,
                                &def_mod,
                                &nrctx,
                                &ver_hash,
                                tc,
                            )?;
                            // Calculate free vars.
                            te.calculate_free_vars();
                            Ok(te)
                        });

                        tasks.push(CheckTask {
                            val_name: val_name.clone(),
                            task,
                            method_impl_idx: Some(i),
                        });
                    }
                }
            };
        }

        // Run all tasks.
        struct CheckResult {
            val_name: FullName,
            te: Result<TypedExpr, Errors>,
            method_impl_idx: Option<usize>,
        }
        let results = if tc.num_worker_threads <= 1 {
            // Run tasks in the main thread.
            let mut results = vec![];
            for task in tasks {
                let te = (task.task)();
                results.push(CheckResult {
                    val_name: task.val_name,
                    te,
                    method_impl_idx: task.method_impl_idx,
                });
            }
            results
        } else {
            // Run tasks in parallel.
            let mut threads = vec![];
            let tasks_per_thread = tasks.len() / tc.num_worker_threads;
            for i in (0..tc.num_worker_threads).rev() {
                let mut tasks = if i == 0 {
                    std::mem::take(&mut tasks)
                } else {
                    tasks.split_off(tasks.len() - tasks_per_thread)
                };
                let thread = std::thread::spawn(move || {
                    let mut results = vec![];
                    while let Some(task) = tasks.pop() {
                        let te = (task.task)();
                        results.push(CheckResult {
                            val_name: task.val_name,
                            te,
                            method_impl_idx: task.method_impl_idx,
                        });
                    }
                    results
                });
                threads.push(thread);
            }
            let mut results = vec![];
            for thread in threads {
                results.append(&mut thread.join().unwrap());
            }
            results
        };

        // Store results to `self.global_values`.
        let mut errors = Errors::empty();
        for result in results {
            if result.te.is_err() {
                errors.append(result.te.err().unwrap());
                continue;
            }
            let te = result.te.ok().unwrap();
            let gv = self.global_values.get_mut(&result.val_name).unwrap();
            match &mut gv.expr {
                SymbolExpr::Simple(e) => {
                    *e = te;
                }
                SymbolExpr::Method(impls) => {
                    let i = result.method_impl_idx.unwrap();
                    impls[i].expr = te;
                }
            };
        }

        errors.to_result()
    }

    // Instantiate symbol.
    fn instantiate_symbol(
        &mut self,
        sym: &mut InstantiatedSymbol,
        tc: &TypeCheckContext,
    ) -> Result<(), Errors> {
        assert!(sym.expr.is_none());
        if !sym.ty.free_vars().is_empty() {
            return Err(Errors::from_msg_srcs(
                format!(
                    "Cannot instantiate global value `{}` of type `{}` since the type contains undetermined type variable. Maybe you need to add type annotation.",
                    sym.generic_name.to_string(),
                    sym.ty.to_string_normalize()
                ),
                &[&sym.expr.as_ref().map(|expr| expr.source.clone()).flatten()],
            ));
        }
        // First, perform namespace resolution and type-checking.
        let method_selector = |method: &MethodImpl| -> Result<bool, Errors> {
            // Select method implementation whose type unifies with the required type `sym.ty`.
            //
            // NOTE: Since overlapping implementations and unrelated methods are forbidden,
            // we only need to check the unifiability here,
            // and we do not need to check whether predicates or equality constraints are satisfiable or not.
            let mut tc0 = tc.clone();
            Ok(UnifOrOtherErr::extract_others(tc0.unify(&method.ty.ty, &sym.ty))?.is_ok())
        };
        self.resolve_namespace_and_check_type(tc, &[sym.generic_name.clone()], method_selector)?;

        // Then perform instantiation.
        let global_sym = self.global_values.get(&sym.generic_name).unwrap();
        let expr = match &global_sym.expr {
            SymbolExpr::Simple(e) => {
                // Specialize e's type to the required type `sym.ty`.
                let mut tc = tc.clone();
                assert!(tc.substitution.is_empty());
                tc.substitution = e.substitution.clone();
                tc.unify(e.expr.ty.as_ref().unwrap(), &sym.ty).ok().unwrap();
                tc.finish_inferred_types(e.expr.clone())?
            }
            SymbolExpr::Method(impls) => {
                let mut opt_e: Option<Arc<ExprNode>> = None;
                for method in impls {
                    // Select method implementation.
                    if !method_selector(method)? {
                        continue;
                    }
                    let e = method.expr.clone();

                    // Specialize e's type to the required type `sym.ty`.
                    let mut tc = tc.clone();
                    assert!(tc.substitution.is_empty());
                    tc.substitution = e.substitution;
                    tc.unify(e.expr.ty.as_ref().unwrap(), &sym.ty).ok().unwrap();
                    opt_e = Some(tc.finish_inferred_types(e.expr)?);
                    break;
                }
                opt_e.unwrap()
            }
        };
        sym.expr = Some(self.instantiate_expr(&expr)?);
        Ok(())
    }

    // Instantiate all symbols.
    pub fn instantiate_symbols(&mut self, tc: &TypeCheckContext) -> Result<(), Errors> {
        let mut errors = Errors::empty();
        while !self.deferred_instantiation.is_empty() {
            let sym = self.deferred_instantiation.pop().unwrap();
            let name = sym.instantiated_name.clone();
            let mut sym = sym.clone();
            errors.eat_err(self.instantiate_symbol(&mut sym, tc));
            self.instantiated_symbols.insert(name, sym);
        }
        errors.to_result()
    }

    // Instantiate `Main::main` (or `Test::test` if `fix test` is running).
    pub fn instantiate_main_function(
        &mut self,
        tc: &TypeCheckContext,
        test_mode: bool,
    ) -> Result<Arc<ExprNode>, Errors> {
        let main_func_name = if test_mode {
            FullName::from_strs(&[TEST_MODULE_NAME], TEST_FUNCTION_NAME)
        } else {
            FullName::from_strs(&[MAIN_MODULE_NAME], MAIN_FUNCTION_NAME)
        };
        let main_ty = make_io_unit_ty();
        let (expr, _ty) =
            self.instantiate_exported_value(&main_func_name, Some(main_ty), &None, tc)?;
        Ok(expr)
    }

    // Instantiate exported values.
    // In this function, `ExportStatement`s are updated.
    pub fn instantiate_exported_values(&mut self, tc: &TypeCheckContext) -> Result<(), Errors> {
        let mut errors = Errors::empty();
        let mut export_stmts = std::mem::replace(&mut self.export_statements, vec![]);
        for stmt in &mut export_stmts {
            errors.eat_err_or(
                self.instantiate_exported_value(&stmt.fix_value_name, None, &stmt.src, tc),
                |(instantiated_expr, eft)| {
                    stmt.exported_function_type = Some(eft);
                    stmt.instantiated_value_expr = Some(instantiated_expr);
                },
            );
        }
        errors.to_result()?;
        self.export_statements = export_stmts;
        Ok(())
    }

    // Instantiate a global value.
    // - required_ty: for `Main::main`, pass `IO ()` to check that the specified type is correct. If none, then use the type specified by user.
    // - required_src: source place where the value is exported. Used to show error message.
    pub fn instantiate_exported_value(
        &mut self,
        value_name: &FullName,
        required_ty: Option<Arc<TypeNode>>,
        required_src: &Option<Span>,
        tc: &TypeCheckContext,
    ) -> Result<(Arc<ExprNode>, ExportedFunctionType), Errors> {
        // Check if the value is defined.
        let gv = self.global_values.get(value_name);
        if gv.is_none() {
            return Err(Errors::from_msg_srcs(
                format!("Value `{}` is not found.", value_name.to_string()),
                &[required_src],
            ));
        }

        // Validate the type of the value.
        let gv: &GlobalValue = gv.unwrap();
        let (required_ty, eft) = if let Some(required_ty) = required_ty {
            // If the type of the value is specified, check if it matches the required type.
            if gv.scm.to_string_normalize() != required_ty.to_string() {
                let gv_src = gv.scm.ty.get_source();
                return Err(Errors::from_msg_srcs(
                    format!(
                        "The value `{}` should have type `{}`.",
                        value_name.to_string(),
                        required_ty.to_string()
                    ),
                    &[gv_src, required_src],
                ));
            }
            let eft = ExportedFunctionType {
                doms: vec![],
                codom: make_unit_ty(),
                is_io: true,
            };
            (required_ty, eft)
        } else {
            // If the type of the value is not specified, check if it is good as the type of an exported value.
            let err_msg_prefix = format!(
                "The type of the value `{}` is not suitable for export: ",
                value_name.to_string(),
            );
            let eft = ExportedFunctionType::validate(
                gv.scm.clone(),
                &tc.type_env,
                err_msg_prefix,
                required_src,
            )?;
            (gv.scm.ty.clone(), eft)
        };
        let inst_name = self.require_instantiated_symbol(&value_name, &required_ty)?;
        self.instantiate_symbols(tc)?;
        let expr = expr_var(inst_name, None).set_inferred_type(required_ty);
        Ok((expr, eft))
    }

    // Instantiate expression.
    fn instantiate_expr(&mut self, expr: &Arc<ExprNode>) -> Result<Arc<ExprNode>, Errors> {
        let ret = match &*expr.expr {
            Expr::Var(v) => {
                if v.name.is_local() {
                    expr.clone()
                } else {
                    let instance =
                        self.require_instantiated_symbol(&v.name, &expr.ty.as_ref().unwrap())?;
                    let v = v.set_name(instance);
                    expr.set_var_var(v)
                }
            }
            Expr::LLVM(_) => expr.clone(),
            Expr::App(fun, args) => {
                let fun = self.instantiate_expr(fun)?;
                let args = collect_results(args.iter().map(|arg| self.instantiate_expr(arg)))?;
                expr.set_app_func(fun).set_app_args(args)
            }
            Expr::Lam(_, body) => expr.set_lam_body(self.instantiate_expr(body)?),
            Expr::Let(_, bound, val) => {
                let bound = self.instantiate_expr(bound)?;
                let val = self.instantiate_expr(val)?;
                expr.set_let_bound(bound).set_let_value(val)
            }
            Expr::If(cond, then_expr, else_expr) => {
                let cond = self.instantiate_expr(cond)?;
                let then_expr = self.instantiate_expr(then_expr)?;
                let else_expr = self.instantiate_expr(else_expr)?;
                expr.set_if_cond(cond)
                    .set_if_then(then_expr)
                    .set_if_else(else_expr)
            }
            Expr::TyAnno(e, _) => {
                let e = self.instantiate_expr(e)?;
                expr.set_tyanno_expr(e)
            }
            Expr::MakeStruct(_, fields) => {
                let mut expr = expr.clone();
                for (field_name, field_expr) in fields {
                    let field_expr = self.instantiate_expr(field_expr)?;
                    expr = expr.set_make_struct_field(field_name, field_expr);
                }
                expr
            }
            Expr::ArrayLit(elems) => {
                let mut expr = expr.clone();
                for (i, e) in elems.iter().enumerate() {
                    let e = self.instantiate_expr(e)?;
                    expr = expr.set_array_lit_elem(e, i);
                }
                expr
            }
            Expr::FFICall(_, _, _, args) => {
                let mut expr = expr.clone();
                for (i, e) in args.iter().enumerate() {
                    let e = self.instantiate_expr(e)?;
                    expr = expr.set_ffi_call_arg(e, i);
                }
                expr
            }
        };
        // If the type of an expression contains undetermied type variable after instantiation, raise an error.
        if !ret.ty.as_ref().unwrap().free_vars().is_empty() {
            return Err(Errors::from_msg_srcs(
                "The type of an expression cannot be determined. You need to add type annotation to help type inference.".to_string(),
                &[&ret.source],
            ));
        }
        Ok(calculate_free_vars(ret))
    }

    // Require instantiate generic symbol such that it has a specified type.
    pub fn require_instantiated_symbol(
        &mut self,
        name: &FullName,
        ty: &Arc<TypeNode>,
    ) -> Result<FullName, Errors> {
        let inst_name = self.determine_instantiated_symbol_name(name, ty)?;
        if !self.instantiated_symbols.contains_key(&inst_name)
            && self
                .deferred_instantiation
                .iter()
                .all(|symbol| symbol.instantiated_name != inst_name)
        {
            self.deferred_instantiation.push(InstantiatedSymbol {
                instantiated_name: inst_name.clone(),
                generic_name: name.clone(),
                ty: ty.clone(),
                expr: None,
            });
        }
        Ok(inst_name)
    }

    // Determine the name of instantiated generic symbol so that it has a specified type.
    // tc: a typechecker (substituion) under which ty should be interpreted.
    fn determine_instantiated_symbol_name(
        &self,
        name: &FullName,
        ty: &Arc<TypeNode>,
    ) -> Result<FullName, Errors> {
        let ty = ty.resolve_type_aliases(&self.type_env())?;
        let hash = ty.hash();
        let mut name = name.clone();
        name.name += INSTANCIATED_NAME_SEPARATOR;
        name.name += &hash;
        Ok(name)
    }

    // Create symbols of trait methods from TraitEnv.
    pub fn create_trait_method_symbols(&mut self) {
        for (trait_id, trait_info) in &self.trait_env.traits {
            for method_info in &trait_info.methods {
                let method_ty = trait_info.method_scheme(&method_info.name);
                let mut method_impls: Vec<MethodImpl> = vec![];
                let instances = self.trait_env.instances.get(trait_id);
                if let Some(insntances) = instances {
                    for trait_impl in insntances {
                        let scm = trait_impl.method_scheme(&method_info.name, trait_info);
                        let expr = trait_impl.method_expr(&method_info.name);
                        method_impls.push(MethodImpl {
                            ty: scm,
                            expr: TypedExpr::from_expr(expr),
                            define_module: trait_impl.define_module.clone(),
                        });
                    }
                }
                let method_name = FullName::new(&trait_id.name.to_namespace(), &method_info.name);
                self.global_values.insert(
                    method_name,
                    GlobalValue {
                        scm: method_ty,
                        expr: SymbolExpr::Method(method_impls),
                        def_src: method_info.source.clone(),
                        document: method_info.document.clone(),
                    },
                );
            }
        }
    }

    pub fn validate_global_value_type_constraints(&self) -> Result<(), Errors> {
        let mut errors = Errors::empty();
        for (_name, gv) in &self.global_values {
            if let Err(e) = gv.scm.validate_constraints(&self.trait_env) {
                errors.append(e);
                continue;
            };
            match gv.expr {
                SymbolExpr::Simple(ref _e) => {}
                SymbolExpr::Method(ref impls) => {
                    for impl_ in impls {
                        errors.eat_err(impl_.ty.validate_constraints(&self.trait_env));
                    }
                }
            }
        }
        errors.to_result()
    }

    // Validate and update export statements.
    pub fn validate_export_statements(&self) -> Result<(), Errors> {
        let mut errors = Errors::empty();

        // Validate each export statement.
        for stmt in &self.export_statements {
            errors.eat_err(stmt.validate_names(&stmt.src));
        }

        // Throw errors if any.
        errors.to_result()?;

        // Check if there are multiple export statements having the same `c_function_name`.
        let mut c_function_names: Vec<(String, Option<Span>)> = Default::default();
        for stmt in &self.export_statements {
            if let Some((_, span)) = c_function_names
                .iter()
                .find(|(name, _)| *name == stmt.c_function_name)
            {
                errors.append(Errors::from_msg_srcs(
                    format!(
                        "Multiple export statements have the same C function name `{}`.",
                        stmt.c_function_name
                    ),
                    &[&stmt.src, span],
                ));
                continue;
            }
            c_function_names.push((stmt.c_function_name.clone(), stmt.src.clone()));
        }

        errors.to_result()?;
        Ok(())
    }

    pub fn set_kinds(&mut self) -> Result<(), Errors> {
        self.trait_env.set_kinds_in_trait_and_alias_defns()?;
        let kind_env = self.kind_env();
        self.trait_env.set_kinds_in_trait_instances(&kind_env)?;
        let mut errors = Errors::empty();
        for (_name, sym) in &mut self.global_values {
            errors.eat_err(sym.set_kinds(&kind_env));
        }
        errors.to_result()
    }

    pub fn kind_env(&self) -> KindEnv {
        KindEnv {
            tycons: self.type_env().kinds(),
            assoc_tys: self.trait_env.assoc_ty_kind_info(),
            traits_and_aliases: self.trait_env.trait_kind_map_with_aliases(),
        }
    }

    // Infer namespaces of traits and types that appear in declarations and associated type implementations.
    // NOTE: names in the lhs of definition of types/traits/global_values have to be full-named already when this function called.
    pub fn resolve_namespace_in_type_signs(&mut self) -> Result<(), Errors> {
        let mut ctx = NameResolutionContext::new(
            &self.tycon_names_with_aliases(),
            &self.trait_names_with_aliases(),
            self.assoc_ty_to_arity(),
            vec![],
        );
        // Resolve namespaces in type constructors.
        {
            let mut tycons = (*self.type_env.tycons).clone();
            for (tc, ti) in &mut tycons {
                ctx.import_statements = self.mod_to_import_stmts[&tc.name.module()].clone();
                ti.resolve_namespace(&ctx)?;
            }
            self.type_env.tycons = Arc::new(tycons);
        }
        // Resolve namespaces in type aliases.
        {
            let mut aliases = (*self.type_env.aliases).clone();
            for (tc, ta) in &mut aliases {
                ctx.import_statements = self.mod_to_import_stmts[&tc.name.module()].clone();
                ta.resolve_namespace(&ctx)?;
            }
            self.type_env.aliases = Arc::new(aliases);
        }

        self.trait_env
            .resolve_namespace(&mut ctx, &self.mod_to_import_stmts)?;
        for decl in &mut self.type_defns {
            ctx.import_statements = self.mod_to_import_stmts[&decl.name.module()].clone();
            decl.resolve_namespace(&ctx)?;
        }
        for (name, sym) in &mut self.global_values {
            ctx.import_statements = self.mod_to_import_stmts[&name.module()].clone();
            sym.resolve_namespace_in_declaration(&ctx)?;
        }
        Ok(())
    }

    // Resolve type aliases that appear in declarations and associated type implementations.
    pub fn resolve_type_aliases_in_declaration(&mut self) -> Result<(), Errors> {
        let mut errors = Errors::empty();

        // Resolve aliases in type constructors.
        let type_env = self.type_env();
        let mut tycons = (*self.type_env.tycons).clone();
        for (_, ti) in &mut tycons {
            errors.eat_err(ti.resolve_type_aliases(&type_env));
        }
        errors.to_result()?; // Throw errors if any.
        self.type_env.tycons = Arc::new(tycons);

        // Get the updated type env.
        let type_env = self.type_env();

        // Resolve aliases in type aliases.
        errors.eat_err(self.trait_env.resolve_type_aliases(&type_env));

        // Resolve aliases in type definitions.
        for decl in &mut self.type_defns {
            errors.eat_err(decl.resolve_type_aliases(&type_env));
        }

        // Resolve aliases in type signatures of global values.
        for (_, sym) in &mut self.global_values {
            errors.eat_err(sym.resolve_type_aliases(&type_env));
        }

        errors.to_result()
    }

    // Validate user-defined types
    pub fn validate_type_defns(&self) -> Result<(), Errors> {
        let mut errors = Errors::empty();
        for type_defn in &self.type_defns {
            type_defn.check_tyvars();
            let type_name = &type_defn.name;
            match &type_defn.value {
                TypeDeclValue::Struct(str) => {
                    for field in &str.fields {
                        if !field.ty.is_assoc_ty_free() {
                            errors.append(Errors::from_msg_srcs(
                                "Associated type is not allowed in the field type of a struct."
                                    .to_string(),
                                &[&type_defn.source.as_ref().map(|s| s.to_head_character())],
                            ));
                        }
                    }
                    match Field::check_duplication(&str.fields) {
                        Some(field_name) => {
                            errors.append(Errors::from_msg_srcs(
                                format!(
                                    "Duplicate field `{}` in the definition of struct `{}`.",
                                    field_name,
                                    type_name.to_string()
                                ),
                                &[&type_defn.source.as_ref().map(|s| s.to_head_character())],
                            ));
                        }
                        _ => {}
                    }
                }
                TypeDeclValue::Union(union) => {
                    for field in &union.fields {
                        if !field.ty.is_assoc_ty_free() {
                            errors.append(Errors::from_msg_srcs(
                                "Associated type is not allowed in the field type of a union."
                                    .to_string(),
                                &[&type_defn.source.as_ref().map(|s| s.to_head_character())],
                            ));
                        }
                    }
                    match Field::check_duplication(&union.fields) {
                        Some(field_name) => {
                            errors.append(Errors::from_msg_srcs(
                                format!(
                                    "Duplicate field `{}` in the definition of union `{}`.",
                                    field_name,
                                    type_name.to_string()
                                ),
                                &[&type_defn.source.as_ref().map(|s| s.to_head_character())],
                            ));
                        }
                        _ => {}
                    }
                }
                TypeDeclValue::Alias(ta) => {
                    if !ta.value.is_assoc_ty_free() {
                        errors.append(Errors::from_msg_srcs(
                            "Associated type is not allowed in the right-hand side of a type alias.".to_string(),
                            &[&type_defn.source.as_ref().map(|s| s.to_head_character())],
                        ));
                    }
                } // Nothing to do.
            }
        }
        errors.to_result()
    }

    pub fn validate_trait_env(&mut self) -> Result<(), Errors> {
        self.trait_env.validate(self.kind_env())
    }

    // Validate name confliction between types, traits and global values.
    pub fn validate_capital_name_confliction(&self) -> Result<(), Errors> {
        let mut errors = Errors::empty();

        let types = self.tycon_names_with_aliases();
        let traits = self.trait_names_with_aliases();
        let assc_tys = self.assoc_ty_to_arity();

        // Check if there is a name confliction between types and traits.
        for name in types.iter() {
            if traits.contains(name) {
                errors.append(Errors::from_msg(format!(
                    "Name confliction: `{}` is both a type and a trait.",
                    name.to_string()
                )));
            }
        }

        // Check if there is a name confliction between types and traits.
        for name in types.iter() {
            if assc_tys.contains_key(name) {
                errors.append(Errors::from_msg(format!(
                    "Name confliction: `{}` is both a type and an associated type.",
                    name.to_string()
                )));
            }
        }

        // Check if there is a name confliction between traits and associated types.
        for name in traits.iter() {
            if assc_tys.contains_key(name) {
                errors.append(Errors::from_msg(format!(
                    "Name confliction: `{}` is both a trait and an associated type.",
                    name.to_string()
                )));
            }
        }

        errors.to_result()
    }

    pub fn add_methods(self: &mut Program) -> Result<(), Errors> {
        let mut errors = Errors::empty();
        for decl in &self.type_defns.clone() {
            match &decl.value {
                TypeDeclValue::Struct(str) => {
                    let struct_name = decl.name.clone();
                    for field in &str.fields {
                        // Add getter function
                        errors.eat_err(self.add_global_value(
                            FullName::new(
                                &decl.name.to_namespace(),
                                &format!("{}{}", STRUCT_GETTER_SYMBOL, &field.name),
                            ),
                            struct_get(&struct_name, decl, &field.name),
                            None,
                            Some(format!(
                                "Retrieves the field `{}` from a value of `{}`.",
                                &field.name, struct_name.name
                            )),
                        ));
                        // Add setter function
                        errors.eat_err(self.add_global_value(
                            FullName::new(
                                &decl.name.to_namespace(),
                                &format!("{}{}", STRUCT_SETTER_SYMBOL, &field.name),
                            ),
                            struct_set(&struct_name, decl, &field.name),
                            None,
                            Some(format!(
                                "Updates a value of `{}` by setting field `{}` to a specified one.",
                                struct_name.name, &field.name,
                            )),
                        ));
                        // Add modifier functions.
                        errors.eat_err(self.add_global_value(
                            FullName::new(
                                &decl.name.to_namespace(),
                                &format!("{}{}", STRUCT_MODIFIER_SYMBOL, &field.name,),
                            ),
                            struct_mod(&struct_name, decl, &field.name),
                            None,
                            Some(format!(
                                "Updates a value of `{}` by applying a function to field `{}`.",
                                struct_name.name, &field.name,
                            )),
                        ));
                        // Add act functions
                        errors.eat_err(self.add_global_value(
                            FullName::new(
                                &decl.name.to_namespace(),
                                &format!("{}{}", STRUCT_ACT_SYMBOL, &field.name),
                            ),
                            struct_act(&struct_name, decl, &field.name),
                            None,
                            Some(format!(
                                "Updates a value of `{}` by applying a functorial action to field `{}`.",
                                struct_name.name, &field.name,
                            )),
                        ));
                        // Add punch functions.
                        errors.eat_err(self.add_global_value(
                            FullName::new(
                                &decl.name.to_namespace(),
                                &format!("{}{}", STRUCT_PUNCH_SYMBOL, &field.name),
                            ),
                            struct_punch(&struct_name, decl, &field.name),
                            None,
                            None,
                        ));
                        // Add plug-in functions.
                        errors.eat_err(self.add_global_value(
                            FullName::new(
                                &decl.name.to_namespace(),
                                &format!("{}{}", STRUCT_PLUG_IN_SYMBOL, &field.name),
                            ),
                            struct_plug_in(&struct_name, decl, &field.name),
                            None,
                            None,
                        ));
                    }
                }
                TypeDeclValue::Union(union) => {
                    let union_name = &decl.name;
                    for field in &union.fields {
                        errors.eat_err(self.add_global_value(
                            FullName::new(&decl.name.to_namespace(), &field.name),
                            union_new(&union_name, &field.name, decl),
                            None,
                            Some(format!(
                                "Constructs a value of union `{}` taking the variant `{}`.",
                                union_name.name, &field.name
                            )),
                        ));
                        errors.eat_err(self.add_global_value(
                            FullName::new(&decl.name.to_namespace(), &format!("as_{}", field.name)),
                            union_as(&union_name, &field.name, decl),
                            None,
                            Some(format!(
                                "Unwraps a union value of `{}` as the variant `{}`.\nIf the value is not the variant `{}`, this function aborts the program.",
                                union_name.name, &field.name, &field.name,
                            )),
                        ));
                        errors.eat_err(self.add_global_value(
                            FullName::new(&decl.name.to_namespace(), &format!("is_{}", field.name)),
                            union_is(&union_name, &field.name, decl),
                            None,
                            Some(format!(
                                "Checks if a union value of `{}` is the variant `{}`.",
                                union_name.name, &field.name,
                            )),
                        ));
                        errors.eat_err(self.add_global_value(
                            FullName::new(
                                &decl.name.to_namespace(),
                                &format!("mod_{}", field.name),
                            ),
                            union_mod_function(&union_name, &field.name, decl),
                            None,
                            Some(format!(
                                "Updates a value of union `{}` by applying a function if it is the variant `{}`, or doing nothing otherwise.",
                                union_name.name, &field.name,
                            )),
                        ));
                    }
                }
                TypeDeclValue::Alias(_) => {} // Nothing to do
            }
        }
        errors.to_result()?;
        Ok(())
    }

    pub fn linked_mods(&self) -> HashSet<Name> {
        self.mod_to_import_stmts.keys().cloned().collect()
    }

    // Link an module.
    // * extend - If true, the module defined in `other` allowed to conflict with a module already in `self`.
    //            This is used for extending implementation of a module already linked to `self`.
    pub fn link(&mut self, mut other: Program, extend: bool) -> Result<(), Errors> {
        let mut errors = Errors::empty();

        // Merge `module_to_files`.
        // Also, check if there is a module defined in multiple files.
        for mod_info in &other.modules {
            let file = mod_info.source.input.file_path.clone();
            if let Some(other_mod) = self.modules.iter().find(|mi| mi.name == mod_info.name) {
                // If the module is already defined,
                if extend {
                    // If extending mode, this is not a problem.
                    continue;
                }
                let other_file = other_mod.source.input.file_path.clone();
                if to_absolute_path(&other_file) == to_absolute_path(&file) {
                    // If the module is defined in the same file, this is not a problem.
                    continue;
                }
                let msg = format!(
                    "Module `{}` is defined in two files: \"{}\" and \"{}\".",
                    mod_info.name,
                    other_file.to_string_lossy().to_string(),
                    file.to_string_lossy().to_string()
                );
                errors.append(Errors::from_msg(msg));
                continue;
            }
            self.modules.push(mod_info.clone());
        }

        // Throw an error if necessary.
        errors.to_result()?;

        // If already linked, do nothing.
        if !extend
            && self
                .linked_mods()
                .contains(&other.get_name_if_single_module())
        {
            return Ok(());
        }

        // Merge `mod_to_import_stmts`.
        for (importer, importee) in &other.mod_to_import_stmts {
            if let Some(old_importee) = self.mod_to_import_stmts.get_mut(importer) {
                old_importee.extend(importee.iter().cloned());
            } else {
                self.mod_to_import_stmts
                    .insert(importer.clone(), importee.clone());
            }
        }

        // Merge types.
        self.add_type_defns(other.type_defns);

        // Merge traits and instances.
        errors.eat_err(self.trait_env.import(other.trait_env));

        // Merge global values.
        for (name, gv) in other.global_values {
            if gv.is_simple_value() {
                errors.eat_err(self.add_global_value_gv(name, gv));
            }
        }

        // Merge export statements.
        self.export_statements.append(&mut other.export_statements);

        // Merge used_tuple_sizes.
        self.used_tuple_sizes.append(&mut other.used_tuple_sizes);

        errors.to_result()
    }

    // Check that all imported modules are linked.
    pub fn check_imports(&mut self) -> Result<(), Errors> {
        let mut unresolved_imports = self.import_statements();

        loop {
            if unresolved_imports.is_empty() {
                break Ok(());
            }
            let import_stmt = unresolved_imports.pop().unwrap();
            let module = &import_stmt.module.0;

            // If import is already resolved, do nothing.
            if self.is_linked(&module) {
                continue;
            }

            return Err(Errors::from_msg_srcs(
                format!("Cannot find module `{}`.", module),
                &[&import_stmt.source],
            ));
        }
    }

    // Create a graph of modules. If module A imports module B, an edge from A to B is added.
    pub fn importing_module_graph(&self) -> (Graph<Name>, HashMap<Name, usize>) {
        let (mut graph, elem_to_idx) = Graph::from_set(self.linked_mods());
        for (importer, stmts) in &self.mod_to_import_stmts {
            for stmt in stmts {
                graph.connect(
                    *elem_to_idx.get(importer).unwrap(),
                    *elem_to_idx.get(&stmt.module.0).unwrap(),
                );
            }
        }
        (graph, elem_to_idx)
    }

    // Calculate a set of modules on which a module depends.
    pub fn dependent_modules(&self, module: &Name) -> HashSet<Name> {
        let (importing_graph, mod_to_node) = self.importing_module_graph();
        importing_graph
            .reachable_nodes(*mod_to_node.get(module).unwrap())
            .iter()
            .map(|idx| importing_graph.get(*idx).clone())
            .collect()
    }

    // Calculate a map from a module to a set of modules on which the module depends.
    pub fn module_dependency_map(&self) -> HashMap<Name, HashSet<Name>> {
        // TODO: Improve time complexity.
        let mods = self.linked_mods();
        let mut dependency = HashMap::new();
        for module in &mods {
            dependency.insert(module.clone(), self.dependent_modules(&module));
        }
        dependency
    }

    // Calculate a hash value of a module which is affected by source codes of all dependent modules.
    pub fn module_dependency_hash(&self, module: &Name) -> String {
        let mut dependent_module_names = self
            .dependent_modules(module)
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        dependent_module_names.sort(); // To remove randomness introduced by HashSet, we sort it.
        let concatenated_source_hashes = dependent_module_names
            .iter()
            .map(|mod_name| {
                exit_if_err(
                    self.modules
                        .iter()
                        .find(|mi| mi.name == *mod_name)
                        .unwrap()
                        .source
                        .input
                        .hash(),
                )
            })
            .collect::<Vec<_>>()
            .join("");
        format!("{:x}", md5::compute(concatenated_source_hashes))
    }

    // Calculate a map from a module to a hash value of the module which is affected by source codes of all dependent modules.
    pub fn module_dependency_hash_map(&self) -> HashMap<Name, String> {
        // TODO: Improve time complexity.
        let mods = self.linked_mods();
        let mut mod_to_hash = HashMap::new();
        for module in &mods {
            mod_to_hash.insert(module.clone(), self.module_dependency_hash(&module));
        }
        mod_to_hash
    }

    // Check if all items referred in import statements are defined.
    pub fn validate_import_statements(&self) -> Result<(), Errors> {
        let mut errors = Errors::empty();

        let stmts = self.import_statements();
        let items = stmts.iter().map(|stmt| stmt.referred_items()).flatten();

        let values = self.global_values.keys().collect::<HashSet<_>>();
        let types = self.tycon_names_with_aliases();
        let traits = self.trait_names_with_aliases();

        for item in items {
            match item {
                ImportItem::Symbol(name, src) => {
                    if values.contains(&name) {
                        continue;
                    }
                    errors.append(Errors::from_msg_srcs(
                        format!("Cannot find value named `{}`.", name.to_string()),
                        &[&src],
                    ));
                }
                ImportItem::TypeOrTrait(name, src) => {
                    if types.contains(&name) || traits.contains(&name) {
                        continue;
                    }
                    errors.append(Errors::from_msg_srcs(
                        format!("Cannot find entity named `{}`.", name.to_string()),
                        &[&src],
                    ));
                }
                ImportItem::NameSpace(namespace, src) => {
                    // Search for an entity that is in the namespace.
                    if values.iter().any(|name| name.is_in_namespace(&namespace)) {
                        continue;
                    }
                    if types.iter().any(|name| name.is_in_namespace(&namespace)) {
                        continue;
                    }
                    if traits.iter().any(|name| name.is_in_namespace(&namespace)) {
                        continue;
                    }
                    errors.append(Errors::from_msg_srcs(
                        format!(
                            "Namespace `{}` is not defined or empty.",
                            namespace.to_string()
                        ),
                        &[&src],
                    ));
                }
            }
        }
        errors.to_result()
    }

    // Find the minimum node which includes the specified source code position.
    pub fn find_node_at(&self, pos: &SourcePos) -> Option<EndNode> {
        for (_, gv) in &self.global_values {
            let node = gv.find_node_at(pos);
            if node.is_some() {
                return node;
            }
        }
        for td in &self.type_defns {
            let node = td.find_node_at(pos);
            if node.is_some() {
                return node;
            }
        }
        self.trait_env.find_node_at(pos)
    }
}

pub enum EndNode {
    Expr(Var, Option<Arc<TypeNode>>),
    Pattern(Var, Option<Arc<TypeNode>>),
    Type(TyCon),
    Trait(Trait),
}
