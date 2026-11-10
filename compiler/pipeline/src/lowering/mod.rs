use surface::{BinderGroup, SurfaceExpr};
use syntax::{
    grammar::RuleName,
    node::{Node, NodeKind},
    source::Spanned,
    token::TokenKind,
};

use crate::diagnostics::Diagnostic;

fn find_child(node: &Node, predicate: impl Fn(&NodeKind) -> bool) -> Result<&Node, Diagnostic> {
    node.children
        .iter()
        .find(|&child| predicate(&child.kind))
        .ok_or_else(|| Diagnostic::unexpected_node(node))
}

fn spanned<T>(construct: T, node: &Node) -> Spanned<T> {
    Spanned::new(construct, node.span.clone())
}

pub fn lower(root: &Node, source: &str) -> Result<surface::Module, Vec<Diagnostic>> {
    use surface::Module;

    let mut declarations = Vec::new();
    let mut diagnostics = Vec::new();

    for child in &root.children {
        match lower_declaration(child, source) {
            Ok(declaration) => declarations.push(declaration),
            Err(error) => diagnostics.push(error),
        }
    }

    if diagnostics.is_empty() {
        Ok(Module { declarations })
    } else {
        Err(diagnostics)
    }
}

fn lower_ident(node: &Node, source: &str) -> Result<surface::Ident, Diagnostic> {
    match node.kind {
        NodeKind::Token(TokenKind::Ident) => Ok(surface::Ident::new(
            node.span.text(source).to_owned(),
            node.span.clone(),
        )),
        _ => Err(Diagnostic::unexpected_node(node)),
    }
}

fn lower_expression(node: &Node, source: &str) -> Result<surface::SurfaceExpr, Diagnostic> {
    Ok(match &node.kind {
        NodeKind::Token(TokenKind::Prop) => {
            spanned(surface::ExprKind::Sort(surface::Sort::Prop), node)
        }
        NodeKind::Token(TokenKind::Sort) => {
            spanned(surface::ExprKind::Sort(surface::Sort::Sort), node)
        }
        NodeKind::Token(TokenKind::SelfType) => spanned(surface::ExprKind::SelfType, node),
        NodeKind::Token(TokenKind::Ident) => spanned(
            surface::ExprKind::Identifier(lower_ident(node, source)?),
            node,
        ),
        NodeKind::Rule(RuleName::Expr) => lower_arrow_expression(node, source)?,
        NodeKind::Rule(RuleName::AppExpr) => lower_application(node, source)?,
        NodeKind::Rule(RuleName::QualifiedIdent) => lower_qualified_ident(node, source)?,
        NodeKind::Rule(RuleName::ParenExpr) => lower_parenthesized(node, source)?,
        NodeKind::Rule(RuleName::Lambda) => lower_lambda(node, source)?,
        NodeKind::Rule(RuleName::Forall) => lower_forall(node, source)?,
        _ => return Err(Diagnostic::unexpected_node(node)),
    })
}

fn lower_parenthesized(node: &Node, source: &str) -> Result<surface::SurfaceExpr, Diagnostic> {
    let expression = find_child(node, |kind| matches!(kind, NodeKind::Rule(RuleName::Expr)))?;
    lower_expression(expression, source)
}

fn lower_application(node: &Node, source: &str) -> Result<surface::SurfaceExpr, Diagnostic> {
    let mut expressions = node
        .children
        .iter()
        .filter(|child| matches!(&child.kind, NodeKind::Rule(RuleName::AtomicExpr)))
        .map(|child| lower_atomic_expression(child, source));

    let mut result = expressions
        .next()
        .ok_or_else(|| Diagnostic::unexpected_node(node))??;

    for expression in expressions {
        result = spanned(
            surface::ExprKind::Application {
                function: Box::new(result),
                argument: Box::new(expression?),
            },
            node,
        );
    }

    Ok(result)
}

fn lower_atomic_expression(node: &Node, source: &str) -> Result<surface::SurfaceExpr, Diagnostic> {
    let child = node
        .children
        .first()
        .ok_or_else(|| Diagnostic::unexpected_node(node))?;

    lower_expression(child, source)
}

fn lower_arrow_expression(node: &Node, source: &str) -> Result<surface::SurfaceExpr, Diagnostic> {
    let app = find_child(node, |kind| {
        matches!(kind, NodeKind::Rule(RuleName::AppExpr))
    })?;

    let left = lower_expression(app, source)?;

    let Some(right) = node.children.iter().find(|child| {
        matches!(&child.kind, NodeKind::Rule(RuleName::Expr))
            && child.span.start.pos > app.span.start.pos
    }) else {
        return Ok(left);
    };

    let right = lower_expression(right, source)?;

    Ok(spanned(
        surface::ExprKind::Pi {
            binders: vec![BinderGroup {
                names: vec![],
                ty: Some(left),
                plicity: surface::Plicity::Explicit,
            }],
            body: Box::new(right),
        },
        node,
    ))
}

fn lower_binder_group(node: &Node, source: &str) -> Result<Spanned<BinderGroup>, Diagnostic> {
    let plicity = match &node.kind {
        NodeKind::Rule(RuleName::ExplicitSection) => surface::Plicity::Explicit,
        NodeKind::Rule(RuleName::ImplicitSection) => surface::Plicity::Implicit,
        _ => return Err(Diagnostic::unexpected_node(node)),
    };

    let mut names = Vec::new();

    for child in &node.children {
        if matches!(&child.kind, NodeKind::Token(TokenKind::Ident)) {
            names.push(surface::Ident::new(
                child.span.text(source).to_owned(),
                child.span.clone(),
            ));
        }
    }

    let type_node = find_child(node, |kind| {
        matches!(kind, NodeKind::Rule(RuleName::TypeSpec))
    })?;
    let expression_node = find_child(type_node, |kind| {
        matches!(kind, NodeKind::Rule(RuleName::Expr))
    })?;

    if names.is_empty() {
        return Err(Diagnostic::unexpected_node(node));
    }

    Ok(spanned(
        surface::BinderGroup {
            names,
            ty: Some(lower_expression(expression_node, source)?),
            plicity,
        },
        node,
    ))
}

fn lower_parameters(node: &Node, source: &str) -> Result<Vec<Spanned<BinderGroup>>, Diagnostic> {
    node.children
        .iter()
        .filter(|child| {
            matches!(
                &child.kind,
                NodeKind::Rule(RuleName::ExplicitSection | RuleName::ImplicitSection)
            )
        })
        .map(|child| lower_binder_group(child, source))
        .collect()
}

fn lower_lambda(node: &Node, source: &str) -> Result<surface::SurfaceExpr, Diagnostic> {
    let ident = find_child(node, |kind| {
        matches!(kind, NodeKind::Token(TokenKind::Ident))
    })?;

    let type_spec = find_child(node, |kind| {
        matches!(kind, NodeKind::Rule(RuleName::TypeSpec))
    })?;

    let body = find_child(node, |kind| matches!(kind, NodeKind::Rule(RuleName::Expr)))?;

    let binder =
        lower_binder_group_from_parts(ident, type_spec, surface::Plicity::Explicit, source)?;

    Ok(spanned(
        surface::ExprKind::Lambda {
            binders: vec![binder],
            body: Box::new(lower_expression(body, source)?),
        },
        node,
    ))
}

fn lower_binder_group_from_parts(
    ident: &Node,
    type_spec: &Node,
    plicity: surface::Plicity,
    source: &str,
) -> Result<BinderGroup, Diagnostic> {
    let expression = type_spec
        .children
        .iter()
        .find(|child| matches!(&child.kind, NodeKind::Rule(RuleName::Expr)))
        .ok_or_else(|| Diagnostic::unexpected_node(type_spec))?;

    Ok(BinderGroup {
        names: vec![surface::Ident::new(
            ident.span.text(source).to_owned(),
            ident.span.clone(),
        )],
        ty: Some(lower_expression(expression, source)?),
        plicity,
    })
}

fn lower_forall(node: &Node, source: &str) -> Result<surface::SurfaceExpr, Diagnostic> {
    let ident = find_child(node, |kind| {
        matches!(kind, NodeKind::Token(TokenKind::Ident))
    })?;

    let type_spec = find_child(node, |kind| {
        matches!(kind, NodeKind::Rule(RuleName::TypeSpec))
    })?;

    let body = find_child(node, |kind| matches!(kind, NodeKind::Rule(RuleName::Expr)))?;

    let binder =
        lower_binder_group_from_parts(ident, type_spec, surface::Plicity::Explicit, source)?;

    Ok(spanned(
        surface::ExprKind::Pi {
            binders: vec![binder],
            body: Box::new(lower_expression(body, source)?),
        },
        node,
    ))
}

fn lower_type_spec(node: &Node, source: &str) -> Result<surface::SurfaceExpr, Diagnostic> {
    lower_expression(
        find_child(node, |child| {
            matches!(child, NodeKind::Rule(RuleName::Expr))
        })?,
        source,
    )
}

fn lower_qualified_ident(node: &Node, source: &str) -> Result<surface::SurfaceExpr, Diagnostic> {
    let name = node.span.text(source).to_owned();

    Ok(spanned(
        surface::ExprKind::Identifier(surface::Ident::new(name, node.span.clone())),
        node,
    ))
}

fn lower_declaration(
    node: &Node,
    source: &str,
) -> Result<Spanned<surface::Declaration>, Diagnostic> {
    Ok(spanned(
        match node.kind {
            NodeKind::Rule(RuleName::Const) => {
                surface::Declaration::Definition(lower_definition(node, source)?)
            }
            NodeKind::Rule(RuleName::Ind) => {
                surface::Declaration::Inductive(lower_inductive(node, source)?)
            }
            _ => return Err(Diagnostic::unexpected_node(node)),
        },
        node,
    ))
}

fn lower_definition(node: &Node, source: &str) -> Result<surface::Definition, Diagnostic> {
    let name = find_child(node, |kind| {
        matches!(kind, NodeKind::Token(TokenKind::Ident))
    })?;

    let params = find_child(node, |kind| {
        matches!(kind, NodeKind::Rule(RuleName::Params))
    })?;

    let type_spec = find_child(node, |kind| {
        matches!(kind, NodeKind::Rule(RuleName::TypeSpec))
    })?;

    let value = find_child(node, |kind| matches!(kind, NodeKind::Rule(RuleName::Expr)))?;

    Ok(surface::Definition {
        name: lower_ident(name, source)?,
        parameters: lower_parameters(params, source)?,
        ty: Some(lower_type_spec(type_spec, source)?),
        value: lower_expression(value, source)?,
    })
}

fn lower_inductive(node: &Node, source: &str) -> Result<surface::Inductive, Diagnostic> {
    let name = find_child(node, |kind| {
        matches!(kind, NodeKind::Token(TokenKind::Ident))
    })?;

    let params = find_child(node, |kind| {
        matches!(kind, NodeKind::Rule(RuleName::Params))
    })?;

    let type_spec = find_child(node, |kind| {
        matches!(kind, NodeKind::Rule(RuleName::TypeSpec))
    })?;

    let constructors = node
        .children
        .iter()
        .filter(|child| matches!(&child.kind, NodeKind::Rule(RuleName::Constructor)))
        .map(|child| lower_constructor(child, source))
        .collect::<Result<_, _>>()?;

    let recursor_name = node
        .children
        .iter()
        .rev()
        .find(|&child| matches!(&child.kind, NodeKind::Token(TokenKind::Ident)))
        .ok_or_else(|| Diagnostic::unexpected_node(node))?;

    Ok(surface::Inductive {
        name: lower_ident(name, source)?,
        parameters: lower_parameters(params, source)?,
        ty: Some(lower_type_spec(type_spec, source)?),
        cons: constructors,
        recursor_name: Some(lower_ident(recursor_name, source)?),
    })
}

fn lower_constructor(
    node: &Node,
    source: &str,
) -> Result<Spanned<surface::Constructor>, Diagnostic> {
    let name = find_child(node, |kind| {
        matches!(kind, NodeKind::Token(TokenKind::Ident))
    })?;

    let params = find_child(node, |kind| {
        matches!(kind, NodeKind::Rule(RuleName::Params))
    })?;

    let type_spec = find_child(node, |kind| {
        matches!(kind, NodeKind::Rule(RuleName::TypeSpec))
    })?;

    Ok(spanned(
        surface::Constructor {
            name: lower_ident(name, source)?,
            parameters: lower_parameters(params, source)?,
            ty: lower_type_spec(type_spec, source)?,
        },
        node,
    ))
}
