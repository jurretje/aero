mod context;
mod environment;
mod error;

pub use error::ElabError;

use context::LocalContext;
use environment::Environment;
use surface::SurfaceExpr;

use core::{CoreExpr, Level, Name};

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
        let name = (&definition.name).into();

        if self.env.contains(&name) {
            return Err(ElabError::DuplicateDeclaration(name.literal().to_owned()));
        }

        let binders = definition
            .parameters
            .iter()
            .map(|binder| binder.node.clone())
            .collect::<Vec<_>>();

        let ty = definition
            .ty
            .as_ref()
            .ok_or(ElabError::CannotInferDeclarationType)?;
        let core_ty = self.elaborate_binders(&binders, ty, false)?;
        let core_value = self.elaborate_binders(&binders, &definition.value, true)?;

        let definition = core::Definition {
            name: name.clone(),
            ty: core_ty,
            value: core_value,
        };

        self.env
            .insert(&name, core::Declaration::Definition(definition.clone()));

        Ok(definition)
    }

    fn elaborate_inductive(
        &mut self,
        inductive: &surface::Inductive,
    ) -> Result<core::Inductive, ElabError> {
        let name = Name::from(&inductive.name);

        if self.env.contains(&name) {
            return Err(ElabError::DuplicateDeclaration(name.literal().to_owned()));
        }

        let ty = inductive
            .ty
            .as_ref()
            .ok_or(ElabError::CannotInferDeclarationType)?;

        let binders = inductive
            .parameters
            .iter()
            .map(|binder| binder.node.clone())
            .collect::<Vec<_>>();

        let core_ty = self.elaborate_binders(&binders, ty, false)?;

        self.ctx.set_self(name.clone());

        let mut cons = Vec::with_capacity(inductive.cons.len());
        for constructor in &inductive.cons {
            cons.push(self.elaborate_constructor(&constructor.node, &binders, &name)?);
        }

        self.ctx.reset_self();

        Ok(core::Inductive {
            name,
            ty: core_ty,
            cons,
        })
    }

    fn elaborate_constructor(
        &mut self,
        constructor: &surface::Constructor,
        parameters: &[surface::BinderGroup],
        inductive_name: &Name,
    ) -> Result<core::Constructor, ElabError> {
        let name = Name::from(&constructor.name);

        let mut binders = parameters.to_vec();
        binders.extend(constructor.parameters.iter().map(|bg| &bg.node).cloned());

        let ty = self.elaborate_binders(&binders, &constructor.ty, false)?;

        Ok(core::Constructor { name, ty })
    }

    fn elaborate_recursor(
        &mut self,
        name: Name,
        inductive: &core::Inductive,
    ) -> Result<core::Recursor, ElabError> {
        todo!()
    }

    pub fn elaborate_expr(&mut self, expr: &SurfaceExpr) -> Result<CoreExpr, ElabError> {
        match &expr.node {
            surface::ExprKind::Identifier(ident) => {
                if let Some((index, _)) = self.ctx.lookup_name(&ident.literal()) {
                    return Ok(CoreExpr::BoundVar(index));
                }

                if self.env.contains(&Name::from(ident)) {
                    return Ok(CoreExpr::Const(ident.into()));
                }

                Err(ElabError::UnknownIdentifier(ident.literal().to_owned()))
            }
            surface::ExprKind::Sort(sort) => {
                let level = match sort {
                    surface::Sort::Prop => Level::Zero,
                    surface::Sort::Sort => Level::Zero,
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
        binders: &[surface::BinderGroup],
        body: &SurfaceExpr,
        is_lambda: bool,
    ) -> Result<CoreExpr, ElabError> {
        let Some((binder, remaining)) = binders.split_first() else {
            return self.elaborate_expr(body);
        };

        let ty = binder.ty.as_ref().ok_or(ElabError::CannotInferBinderType)?;

        let domain = self.elaborate_expr(ty)?;

        // TODO
        if binder.names.len() != 1 {
            return Err(ElabError::TooManyBinders);
        }

        let name = binder.names[0].clone();

        let bi = match binder.plicity {
            surface::Plicity::Explicit => core::BinderInfo::Explicit,
            surface::Plicity::Implicit => core::BinderInfo::Implicit,
        };

        let checkpoint = self.ctx.checkpoint();

        self.ctx.push(LocalDecl {
            name,
            ty: domain.clone(),
            bi,
        });

        let result = self.elaborate_binders(remaining, body, is_lambda);

        self.ctx.truncate(checkpoint);

        let body = result?;
        if is_lambda {
            Ok(CoreExpr::Lam {
                info: bi,
                domain: Box::new(domain),
                body: Box::new(body),
            })
        } else {
            Ok(CoreExpr::Pi {
                info: bi,
                domain: Box::new(domain),
                body: Box::new(body),
            })
        }
    }
}
