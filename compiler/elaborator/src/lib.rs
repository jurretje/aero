mod context;
mod environment;
mod error;
mod resolver;

pub use error::ElabError;

use context::LocalContext;
use environment::Environment;
use surface::SurfaceExpr;
use syntax::source::Spanned;

use core::{CoreExpr, NameId};

use crate::context::LocalDecl;

pub struct Elaborator {
    env: Environment,
    ctx: LocalContext,
}

impl Elaborator {
    pub fn new() -> Self {
        Self {
            env: Environment::default(),
            ctx: LocalContext::default(),
        }
    }

    pub fn elaborate_module(
        &mut self,
        module: &surface::Module,
    ) -> Result<core::Module, Vec<ElabError>> {
        let mut decls = Vec::new();
        let mut errors = Vec::new();

        for decl in &module.declarations {
            let result = match &decl.node {
                surface::Declaration::Definition(definition) => self
                    .elaborate_definition(definition)
                    .map(core::Declaration::Definition),
                surface::Declaration::Inductive(inductive) => self
                    .elaborate_inductive(inductive)
                    .map(core::Declaration::Inductive),
            };

            match result {
                Ok(declaration) => decls.push(declaration),
                Err(error) => errors.push(error),
            }
        }

        if errors.is_empty() {
            Ok(core::Module { decls })
        } else {
            Err(errors)
        }
    }

    fn elaborate_definition(
        &mut self,
        definition: &surface::Definition,
    ) -> Result<core::Definition, ElabError> {
        let name: NameId = NameId(0);

        if self.env.contains(&name) {
            todo!() // DUPLICATE DECLARATION
        }

        let ty = definition.ty.as_ref().ok_or(todo!())?;
        let core_ty = self.elaborate_binders(&definition.parameters, ty, false)?;
        let core_value = self.elaborate_binders(&definition.parameters, &definition.value, true)?;

        let definition = core::Definition {
            name: name.clone(),
            ty: core_ty,
            value: core_value,
        };

        // INSERT DECLARATION INTO ENV

        Ok(definition)
    }

    fn elaborate_inductive(
        &mut self,
        inductive: &surface::Inductive,
    ) -> Result<core::Inductive, ElabError> {
        let name = NameId(0);

        if self.env.contains(&name) {
            todo!() // DUPLICATE DECLARATION
        }

        let ty = inductive
            .ty
            .as_ref()
            .ok_or(ElabError::CannotInferDeclarationType)?;

        let core_ty = self.elaborate_binders(&inductive.parameters, ty, false)?;

        self.ctx.set_self(name.clone());

        let mut cons = Vec::with_capacity(inductive.cons.len());
        for constructor in &inductive.cons {
            cons.push(self.elaborate_constructor(&constructor.node, &inductive.parameters)?);
        }

        self.ctx.reset_self();

        let core_inductive = core::Inductive {
            name: name.clone(),
            ty: core_ty,
            cons,
        };

        // INSERT DECLARATION INTO ENV

        Ok(core_inductive)
    }

    fn elaborate_constructor(
        &mut self,
        constructor: &surface::Constructor,
        parameters: &[Spanned<surface::BinderGroup>],
    ) -> Result<core::Constructor, ElabError> {
        let name = todo!();

        let mut binders = parameters.to_vec();
        binders.extend(constructor.parameters.iter().cloned());

        let ty = self.elaborate_binders(&binders, &constructor.ty, false)?;

        Ok(core::Constructor { name, ty })
    }

    fn elaborate_recursor(
        &mut self,
        name: NameId,
        inductive: &core::Inductive,
    ) -> Result<core::Recursor, ElabError> {
        todo!()
    }

    fn elaborate_level(&self, level: &surface::Level) -> Result<core::Level, ElabError> {
        Ok(match level {
            surface::Level::Ident(ident) => core::Level::Param(todo!()),
            surface::Level::Zero => core::Level::Zero,
            surface::Level::Succ(level) => {
                core::Level::Succ(Box::new(self.elaborate_level(level)?))
            }
            surface::Level::Max(lhs, rhs) => core::Level::Max(
                Box::new(self.elaborate_level(lhs)?),
                Box::new(self.elaborate_level(rhs)?),
            ),
        })
    }

    pub fn elaborate_expr(&mut self, expr: &SurfaceExpr) -> Result<CoreExpr, ElabError> {
        match &expr.node {
            surface::ExprKind::Identifier(id) => {
                if let Some((index, _)) = self.ctx.lookup_name(id) {
                    return Ok(CoreExpr::BoundVar(index));
                }

                todo!();
                // if self.env.contains(&Name::from(id)) {
                //     return Ok(CoreExpr::Const(id.into()));
                // }

                Err(ElabError::UnknownIdentifier(*id))
            }
            surface::ExprKind::Sort(sort) => {
                let level = match sort {
                    surface::Sort::Prop => core::Level::Zero,
                    surface::Sort::Sort(Some(level)) => self.elaborate_level(level)?,
                    surface::Sort::Sort(None) => todo!(),
                };

                Ok(CoreExpr::Sort(level))
            }
            surface::ExprKind::SelfType => self
                .ctx
                .current_inductive()
                .clone()
                .ok_or(ElabError::UnsupportedExpression),
            surface::ExprKind::Application { function, argument } => {
                let function = Box::new(self.elaborate_expr(function)?);
                let argument = Box::new(self.elaborate_expr(argument)?);

                Ok(CoreExpr::App { function, argument })
            }
            surface::ExprKind::Lambda { binders, body } => {
                self.elaborate_binders(binders, body, true)
            }
            surface::ExprKind::Pi { binders, body } => self.elaborate_binders(binders, body, false),
        }
    }

    fn elaborate_binders(
        &mut self,
        binders: &[Spanned<surface::BinderGroup>],
        body: &SurfaceExpr,
        is_lambda: bool,
    ) -> Result<CoreExpr, ElabError> {
        let Some((binder, remaining)) = binders.split_first() else {
            return self.elaborate_expr(body);
        };

        let binder = &binder.node;

        let ty = binder.ty.as_ref().ok_or(ElabError::CannotInferBinderType)?;

        let domain = self.elaborate_expr(ty)?;

        let bi = match binder.plicity {
            surface::Plicity::Explicit => core::BinderInfo::Explicit,
            surface::Plicity::Implicit => core::BinderInfo::Implicit,
        };

        let checkpoint = self.ctx.checkpoint();

        for name in &binder.names {
            self.ctx.push(LocalDecl {
                ident: name.clone(),
                ty: domain.clone(),
                bi,
            });
        }

        let result = self.elaborate_binders(remaining, body, is_lambda);
        self.ctx.truncate(checkpoint);

        let mut body = result?;

        for _ in binder.names.iter().rev() {
            body = if is_lambda {
                CoreExpr::Lam {
                    info: bi,
                    domain: Box::new(domain.clone()),
                    body: Box::new(body),
                }
            } else {
                CoreExpr::Pi {
                    info: bi,
                    domain: Box::new(domain.clone()),
                    body: Box::new(body),
                }
            }
        }

        Ok(body)
    }
}
