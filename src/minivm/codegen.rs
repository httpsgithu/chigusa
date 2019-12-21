use super::err::*;
use super::*;
use crate::c0::ast::{self, *};
use crate::prelude::*;
use indexmap::IndexMap;

struct Data {
    typ: Ptr<ast::TypeDef>,
    init_val: Option<Vec<u8>>,
    offset: u16,
}

/// A sink of global data
struct DataSink {
    map: IndexMap<String, Vec<u8>>,
}

impl DataSink {
    pub fn new() -> DataSink {
        DataSink {
            map: IndexMap::new(),
        }
    }
}

struct GlobalData {
    glob_var: DataSink,
    glob_const: DataSink,
}

impl GlobalData {
    pub fn new() -> GlobalData {
        GlobalData {
            glob_var: DataSink::new(),
            glob_const: DataSink::new(),
        }
    }
}

pub type Type = Ptr<ast::TypeDef>;

/// An opaque sink of instructions
struct InstSink(Vec<Ins>);

impl InstSink {
    pub fn new() -> InstSink {
        InstSink(Vec::new())
    }

    pub fn inner(&self) -> &Vec<Ins> {
        &self.0
    }

    /// Append all instruction from the other InstSink
    pub fn append_all(&mut self, other: &mut InstSink) {
        self.0.append(&mut other.0);
    }
}

pub struct Codegen<'a> {
    prog: &'a ast::Program,
    glob: GlobalData,
}

impl<'a> Codegen<'a> {
    pub fn new(prog: &'a ast::Program) -> Codegen<'a> {
        Codegen {
            prog,
            glob: GlobalData::new(),
        }
    }

    pub fn compile(&mut self) {}

    pub fn compile_fn(&mut self, func: &ast::FunctionType, name: &str) -> CompileResult<()> {
        // TODO: Add signature extractor

        // let sig = FnCodegen::extract_sig(func, self);
        let fnc = FnCodegen::new(func, name, self);

        Ok(())
    }
}

/// Resolve all named types into their definitions, and strip function types' bodies
fn resolve_ty(ty: &ast::TypeDef, scope: Ptr<ast::Scope>) -> ast::TypeDef {
    match ty {
        ast::TypeDef::NamedType(n) => {
            let scope_c = scope.cp();
            let scope_b = scope_c.borrow();
            let sty = scope_b.find_def(n).expect("Unknown type inside AST");
            let sty = sty.borrow().get_typ().unwrap();
            let sty = sty.borrow();

            resolve_ty(&*sty, scope.cp())
        }
        prim @ ast::TypeDef::Primitive(..) => prim.clone(),
        ast::TypeDef::Ref(r) => {
            let src = r.target.borrow();
            let res = Ptr::new(resolve_ty(&*src, scope.cp()));
            ast::TypeDef::Ref(ast::RefType { target: res })
        }
        ast::TypeDef::Function(f) => {
            let params = f
                .params
                .iter()
                .map(|a| {
                    let a = a.borrow();
                    Ptr::new(resolve_ty(&*a, scope.cp()))
                })
                .collect();
            let ret = Ptr::new(resolve_ty(&*f.return_type.borrow(), scope.cp()));
            ast::TypeDef::Function(ast::FunctionType {
                params,
                return_type: ret,
                body: None,
                is_extern: f.is_extern,
            })
        }
        _ => todo!("Type resolve not implemented"),
    }
}

/// Calculate the bits needed for a type to contain a value
fn type_bits(len: u32) -> Option<u16> {
    if len > 128 {
        None
    } else if len > 64 {
        Some(128)
    } else if len > 32 {
        Some(64)
    } else if len > 16 {
        Some(32)
    } else if len > 8 {
        Some(16)
    } else {
        Some(8)
    }
}

struct FnCodegen<'b> {
    f: &'b ast::FunctionType,
    name: &'b str,
    // ctx: &'b mut Codegen<'a, T>,
    ebb_cnt: u32,
    loc_cnt: u16,
    data: &'b mut GlobalData,
    loc: IndexMap<String, Data>,
}

impl<'b> FnCodegen<'b> {
    pub fn new<'a>(
        f: &'b ast::FunctionType,
        name: &'b str,
        ctx: &'b mut Codegen<'a>,
    ) -> FnCodegen<'b> {
        FnCodegen {
            f,
            name,
            ebb_cnt: 0,
            loc_cnt: 0,
            data: &mut ctx.glob,
            loc: IndexMap::new(),
            // module: &mut ctx.module,
        }
    }

    pub fn gen(&mut self) {
        if let Some(b) = &self.f.body {
            let scope = b.scope.borrow();
            let stmts = &b.stmts;
            for stmt in stmts {
                self.gen_stmt(stmt);
            }
        } else {
            if self.f.is_extern {
                return;
            } else {
                panic!("No body in function")
            }
        }
    }

    fn add_local(
        &mut self,
        name: &str,
        var: &ast::SymbolDef,
        depth: usize,
        scope: Ptr<ast::Scope>,
    ) {
        match var {
            ast::SymbolDef::Typ { .. } => (),
            ast::SymbolDef::Var { typ, is_const } => {
                if depth != 0 {
                    // Who cares about constants?
                    let var_name = format!("{}`{}", name, depth);
                    let var_loc = self.loc_cnt;
                    self.loc_cnt = self.loc_cnt + 1;
                    // let var = Variable::with_u32(var_loc);
                    // let typ = extract_ty(typ, scope, self.module);
                    // self.builder.declare_var(var, typ);
                    todo!()
                } else {
                    todo!("Add global values");
                    // self.builder.create_global_value();
                }
            }
        }
    }

    fn gen_stmt(&mut self, stmt: &ast::Stmt) {
        match &stmt.var {
            ast::StmtVariant::Expr(e) => {
                self.gen_expr(e.cp());
            }
            ast::StmtVariant::ManyExpr(e) => {
                for e in e {
                    self.gen_expr(e.cp());
                }
            }
            ast::StmtVariant::Return(e) => todo!("Generate code for return"),
            ast::StmtVariant::Block(e) => todo!("Generate code for block"),
            ast::StmtVariant::Print(e) => todo!("Generate code for print"),
            ast::StmtVariant::Scan(e) => todo!("Generate code for scan"),
            ast::StmtVariant::Break => todo!("Generate code for return"),
            ast::StmtVariant::If(e) => todo!("Generate code for return`"),
            ast::StmtVariant::While(e) => todo!("Generate code for ret`urn"),
            ast::StmtVariant::Empty => (),
        }
    }

    /// Generate implicit conversion for `a` and `b` to match their types.
    fn flatten_typ(&mut self, a: Type, b: Type) -> (Type, Type) {
        todo!()
    }

    /// Generate implicit conversion for `val` to match `tgt` type
    fn implicit_conv(&mut self, val: Type, tgt: Ptr<ast::TypeDef>) -> CompileResult<Type> {
        match &*val.borrow() {
            ast::TypeDef::Unit => Err(CompileError::AssignVoid),
            ast::TypeDef::Primitive(p) => {
                todo!()
                //
            }
            ast::TypeDef::Ref(r) => {
                todo!()
                //
            }
            ast::TypeDef::NamedType(..) => Err(CompileError::InternalError(
                "Named types shouldn't appear in type calculation".into(),
            )),
            _ => Err(CompileError::UnsupportedType),
        }
    }

    fn gen_expr(&mut self, expr: Ptr<ast::Expr>) -> CompileResult<Type> {
        let expr = expr.borrow();
        let expr = &*expr;
        match &expr.var {
            ast::ExprVariant::BinaryOp(b) => {
                let lhs = self.gen_expr(b.lhs.cp())?;
                let rhs = self.gen_expr(b.rhs.cp())?;
                todo!()
                // Ok(b.op.build_inst_bin(self.builder.ins(), lhs, rhs)?)
            }

            ast::ExprVariant::UnaryOp(u) => {
                // TODO
                todo!("Implement unary operators")
            }

            ast::ExprVariant::Ident(i) => {
                todo!()
                // let Type = self.loc.get(&i.name).unwrap();
                // let val = self.builder.use_var(Type.0);
                // let typ = Type.1.cp();
                // Ok(Type(val, typ))
            }

            ast::ExprVariant::FunctionCall(f) => {
                // TODO
                todo!("Implement function calls")
            }

            ast::ExprVariant::Literal(lit) => match lit {
                ast::Literal::Boolean { val } => {
                    // let val = self.builder.ins().bconst(types::B8, *val);
                    let typ = Ptr::new(ast::TypeDef::Primitive(ast::PrimitiveType {
                        var: ast::PrimitiveTypeVar::UnsignedInt,
                        occupy_bytes: 1,
                    }));
                    Ok(typ)
                }

                ast::Literal::Integer { val } => {
                    let l = type_bits(val.bit_length()).unwrap();
                    // let val = self
                    //     .builder
                    //     .ins()
                    //     .iconst(Type::int(l).unwrap(), Imm64::new(val.into()));
                    let typ = Ptr::new(ast::TypeDef::Primitive(ast::PrimitiveType {
                        var: ast::PrimitiveTypeVar::UnsignedInt,
                        occupy_bytes: (l / 8) as usize,
                    }));
                    Ok(typ)
                }

                ast::Literal::Float { val } => {
                    todo!();
                    // let val = self.builder.ins().f64const(val.to_f64());
                    let typ = Ptr::new(ast::TypeDef::Primitive(ast::PrimitiveType {
                        var: ast::PrimitiveTypeVar::Float,
                        occupy_bytes: 8,
                    }));
                    Ok(typ)
                }

                ast::Literal::String { val } => {
                    // let data_name = format!("data`{}`{}", self.name, self.dat_cnt);
                    // let data = self.module.declare_data(
                    //     &data_name,
                    //     cranelift_module::Linkage::Local,
                    //     false,
                    //     None,
                    // )?;
                    // self.dat_cnt += 1;
                    // let mut data_ctx = DataContext::new();
                    // let str_val: Vec<_> = val.as_bytes().iter().map(|x| *x).collect();
                    // let str_val = std::ffi::CString::new(str_val).unwrap();
                    // let str_val = str_val.into_bytes_with_nul();
                    // let str_val = str_val.into_boxed_slice();
                    // data_ctx.define(str_val);
                    // self.module.define_data(data, &data_ctx)?;
                    // let global = self.module.declare_data_in_func(data, self.builder.func);
                    // let ptr_typ = self.module.isa().pointer_type();

                    // let val = self.builder.ins().global_value(ptr_typ, global);
                    let typ = Ptr::new(ast::TypeDef::Ref(ast::RefType {
                        target: Ptr::new(ast::TypeDef::Primitive(ast::PrimitiveType {
                            var: ast::PrimitiveTypeVar::UnsignedInt,
                            occupy_bytes: 1,
                        })),
                    }));
                    Ok(typ)
                }

                ast::Literal::Struct { .. } => Err(CompileError::InternalError(
                    "Structs are not yet supported!".into(),
                )),
            },
            _ => todo!("Implement other expression variants"),
        }
    }
}

impl ast::OpVar {
    fn build_inst_bin<'a>(&self, sink: &InstSink, lhs: Type, rhs: Type) -> CompileResult<Type> {
        if lhs != rhs {}
        match self {
            ast::OpVar::Add => {
                todo!()
                // inst_builder.iadd(lhs, rhs);
            }
            _ => todo!(),
        }
    }
}
