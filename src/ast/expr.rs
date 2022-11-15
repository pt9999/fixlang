use super::*;
use std::{collections::HashSet, sync::Arc};

#[derive(Clone, PartialEq)]
pub enum AppSourceCodeOrderType {
    FunctionIsFormer,
    ArgumentIsFormer,
}

#[derive(Clone)]
pub struct ExprNode {
    pub expr: Arc<Expr>,
    pub free_vars: Option<HashSet<String>>,
    pub source: Option<Span>,
    pub app_order: AppSourceCodeOrderType,
}

impl ExprNode {
    // Set free vars
    fn set_free_vars(&self, free_vars: HashSet<String>) -> Arc<Self> {
        let mut ret = self.clone();
        ret.free_vars = Some(free_vars);
        Arc::new(ret)
    }

    // Get free vars
    pub fn free_vars(self: &Self) -> &HashSet<String> {
        self.free_vars.as_ref().unwrap()
    }

    // Set source
    pub fn set_source(&self, src: Option<Span>) -> Arc<Self> {
        let mut ret = self.clone();
        ret.source = src;
        Arc::new(ret)
    }

    // Set app order
    pub fn set_app_order(&self, app_order: AppSourceCodeOrderType) -> Arc<Self> {
        let mut ret = self.clone();
        ret.app_order = app_order;
        Arc::new(ret)
    }

    pub fn set_app_func(&self, func: Arc<ExprNode>) -> Arc<Self> {
        let mut ret = self.clone();
        match &*self.expr {
            Expr::App(_, arg) => {
                ret.expr = Arc::new(Expr::App(func, arg.clone()));
            }
            _ => {
                panic!()
            }
        }
        Arc::new(ret)
    }

    pub fn set_app_arg(&self, arg: Arc<ExprNode>) -> Arc<Self> {
        let mut ret = self.clone();
        match &*self.expr {
            Expr::App(func, _) => {
                ret.expr = Arc::new(Expr::App(func.clone(), arg));
            }
            _ => {
                panic!()
            }
        }
        Arc::new(ret)
    }

    pub fn set_lam_body(&self, body: Arc<ExprNode>) -> Arc<Self> {
        let mut ret = self.clone();
        match &*self.expr {
            Expr::Lam(arg, _) => {
                ret.expr = Arc::new(Expr::Lam(arg.clone(), body));
            }
            _ => {
                panic!()
            }
        }
        Arc::new(ret)
    }

    pub fn set_let_bound(&self, bound: Arc<ExprNode>) -> Arc<Self> {
        let mut ret = self.clone();
        match &*self.expr {
            Expr::Let(var, _, val) => {
                ret.expr = Arc::new(Expr::Let(var.clone(), bound, val.clone()));
            }
            _ => {
                panic!()
            }
        }
        Arc::new(ret)
    }

    pub fn set_let_value(&self, value: Arc<ExprNode>) -> Arc<Self> {
        let mut ret = self.clone();
        match &*self.expr {
            Expr::Let(var, bound, _) => {
                ret.expr = Arc::new(Expr::Let(var.clone(), bound.clone(), value));
            }
            _ => {
                panic!()
            }
        }
        Arc::new(ret)
    }

    pub fn set_if_cond(&self, cond: Arc<ExprNode>) -> Arc<Self> {
        let mut ret = self.clone();
        match &*self.expr {
            Expr::If(_, then_expr, else_expr) => {
                ret.expr = Arc::new(Expr::If(cond, then_expr.clone(), else_expr.clone()));
            }
            _ => {
                panic!()
            }
        }
        Arc::new(ret)
    }

    pub fn set_if_then(&self, then_expr: Arc<ExprNode>) -> Arc<Self> {
        let mut ret = self.clone();
        match &*self.expr {
            Expr::If(cond, _, else_expr) => {
                ret.expr = Arc::new(Expr::If(cond.clone(), then_expr, else_expr.clone()));
            }
            _ => {
                panic!()
            }
        }
        Arc::new(ret)
    }

    pub fn set_if_else(&self, else_expr: Arc<ExprNode>) -> Arc<Self> {
        let mut ret = self.clone();
        match &*self.expr {
            Expr::If(cond, then_expr, _) => {
                ret.expr = Arc::new(Expr::If(cond.clone(), then_expr.clone(), else_expr));
            }
            _ => {
                panic!()
            }
        }
        Arc::new(ret)
    }
}

#[derive(Clone)]
pub enum Expr {
    Var(Arc<Var>),
    Lit(Arc<Literal>),
    App(Arc<ExprNode>, Arc<ExprNode>),
    Lam(Arc<Var>, Arc<ExprNode>),
    Let(Arc<Var>, Arc<ExprNode>, Arc<ExprNode>),
    If(Arc<ExprNode>, Arc<ExprNode>, Arc<ExprNode>), // TODO: Implement case
}

impl Expr {
    pub fn into_expr_info(self: &Arc<Self>, src: Option<Span>) -> Arc<ExprNode> {
        Arc::new(ExprNode {
            expr: self.clone(),
            free_vars: Default::default(),
            source: src,
            app_order: AppSourceCodeOrderType::FunctionIsFormer,
        })
    }
    pub fn to_string(&self) -> String {
        match self {
            Expr::Var(v) => v.name.clone(),
            Expr::Lit(l) => l.name.clone(),
            Expr::App(f, a) => format!("({}) ({})", f.expr.to_string(), a.expr.to_string()),
            Expr::Lam(x, fx) => format!("\\{}->({})", x.name, fx.expr.to_string()),
            Expr::Let(x, b, v) => format!(
                "let {}={} in ({})",
                x.name,
                b.expr.to_string(),
                v.expr.to_string()
            ),
            Expr::If(c, t, e) => format!(
                "if {} then {} else ({})",
                c.expr.to_string(),
                t.expr.to_string(),
                e.expr.to_string()
            ),
        }
    }
}

pub type LiteralGenerator =
    dyn Send + Sync + for<'c, 'm, 'b> Fn(&mut GenerationContext<'c, 'm>) -> PointerValue<'c>;

pub struct Literal {
    pub generator: Arc<LiteralGenerator>,
    pub free_vars: Vec<String>, // e.g. "+" literal has two free variables.
    name: String,
    pub ty: Arc<TypeNode>,
}

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct NameSpace {
    names: Vec<String>, // Empty implies it is local.
}

impl NameSpace {
    pub fn local() -> Self {
        Self { names: vec![] }
    }

    pub fn new(names: Vec<String>) -> Self {
        Self { names }
    }

    pub fn is_local(&self) -> bool {
        self.names.len() == 0
    }

    pub fn to_string(&self) -> String {
        self.names.join("::")
    }

    pub fn is_suffix(&self, rhs: &NameSpace) -> bool {
        let n = self.names.len();
        let m = rhs.names.len();
        if n > m {
            return false;
        }
        for i in 0..n {
            if self.names[n - 1 - i] != rhs.names[m - i - 1] {
                return false;
            }
        }
        return true;
    }
}

pub struct Var {
    pub name: String,
    pub namespace: Option<NameSpace>, // None implies namespace to be inferred
    pub type_annotation: Option<Arc<Scheme>>,
    pub source: Option<Span>,
}

pub fn var_var(
    var_name: &str,
    namespace: Option<NameSpace>,
    type_annotation: Option<Arc<Scheme>>,
    src: Option<Span>,
) -> Arc<Var> {
    Arc::new(Var {
        name: String::from(var_name),
        namespace,
        type_annotation,
        source: src,
    })
}

pub fn var_local(
    var_name: &str,
    type_annotation: Option<Arc<Scheme>>,
    src: Option<Span>,
) -> Arc<Var> {
    var_var(var_name, Some(NameSpace::local()), type_annotation, src)
}

pub fn expr_lit(
    generator: Arc<LiteralGenerator>,
    free_vars: Vec<String>,
    name: String,
    ty: Arc<TypeNode>,
    src: Option<Span>,
) -> Arc<ExprNode> {
    Arc::new(Expr::Lit(Arc::new(Literal {
        generator,
        free_vars,
        name,
        ty,
    })))
    .into_expr_info(src)
}

pub fn expr_let(
    var: Arc<Var>,
    bound: Arc<ExprNode>,
    expr: Arc<ExprNode>,
    src: Option<Span>,
) -> Arc<ExprNode> {
    Arc::new(Expr::Let(var, bound, expr)).into_expr_info(src)
}

pub fn expr_abs(var: Arc<Var>, val: Arc<ExprNode>, src: Option<Span>) -> Arc<ExprNode> {
    Arc::new(Expr::Lam(var, val)).into_expr_info(src)
}

pub fn expr_app(lam: Arc<ExprNode>, arg: Arc<ExprNode>, src: Option<Span>) -> Arc<ExprNode> {
    Arc::new(Expr::App(lam, arg)).into_expr_info(src)
}

// Make variable expression.
pub fn expr_var(var_name: &str, src: Option<Span>) -> Arc<ExprNode> {
    Arc::new(Expr::Var(var_var(var_name, None, None, src.clone()))).into_expr_info(src)
}

pub fn expr_if(
    cond: Arc<ExprNode>,
    then_expr: Arc<ExprNode>,
    else_expr: Arc<ExprNode>,
    src: Option<Span>,
) -> Arc<ExprNode> {
    Arc::new(Expr::If(cond, then_expr, else_expr)).into_expr_info(src)
}

// TODO: use persistent binary search tree as ExprAuxInfo to avoid O(n^2) complexity of calculate_free_vars.
pub fn calculate_free_vars(ei: Arc<ExprNode>) -> Arc<ExprNode> {
    match &*ei.expr {
        Expr::Var(var) => {
            let free_vars = vec![var.name.clone()].into_iter().collect();
            ei.set_free_vars(free_vars)
        }
        Expr::Lit(lit) => {
            let free_vars = lit.free_vars.clone().into_iter().collect();
            ei.set_free_vars(free_vars)
        }
        Expr::App(func, arg) => {
            let func = calculate_free_vars(func.clone());
            let arg = calculate_free_vars(arg.clone());
            let mut free_vars = func.free_vars.clone().unwrap();
            free_vars.extend(arg.free_vars.clone().unwrap());
            ei.set_app_func(func)
                .set_app_arg(arg)
                .set_free_vars(free_vars)
        }
        Expr::Lam(arg, body) => {
            let body = calculate_free_vars(body.clone());
            let mut free_vars = body.free_vars.clone().unwrap();
            free_vars.remove(&arg.name);
            free_vars.remove(SELF_NAME);
            ei.set_lam_body(body).set_free_vars(free_vars)
        }
        Expr::Let(var, bound, val) => {
            // NOTE: Our Let is non-recursive let, i.e.,
            // "let x = f x in g x" is equal to "let y = f x in g y",
            // and x ∈ FreeVars("let x = f x in g x") = (FreeVars(g x) - {x}) + FreeVars(f x) != (FreeVars(g x) + FreeVars(f x)) - {x}.
            let bound = calculate_free_vars(bound.clone());
            let val = calculate_free_vars(val.clone());
            let mut free_vars = val.free_vars.clone().unwrap();
            free_vars.remove(&var.name);
            free_vars.extend(bound.free_vars.clone().unwrap());
            ei.set_let_bound(bound)
                .set_let_value(val)
                .set_free_vars(free_vars)
        }
        Expr::If(cond, then_expr, else_expr) => {
            let cond = calculate_free_vars(cond.clone());
            let then_expr = calculate_free_vars(then_expr.clone());
            let else_expr = calculate_free_vars(else_expr.clone());
            let mut free_vars = cond.free_vars.clone().unwrap();
            free_vars.extend(then_expr.free_vars.clone().unwrap());
            free_vars.extend(else_expr.free_vars.clone().unwrap());
            ei.set_if_cond(cond)
                .set_if_then(then_expr)
                .set_if_else(else_expr)
                .set_free_vars(free_vars)
        }
    }
}
