use crate::general::rules::RuleRegistry;

pub fn bootstrap() -> Option<RuleRegistry> {
    let mut reg = RuleRegistry::default();

    define_lexical(&mut reg)?;
    define_tokens(&mut reg)?;

    // MODULE_PATH := IDENT ("::" IDENT)*
    let module_path = reg.declare_builtin("module_path");
    reg.define_builtin(module_path, |r| {
        Some(r.seq([
            r.ref_builtin("token_ident")?,
            r.star(r.seq([
                r.ref_builtin("delimiter_square")?,
                r.ref_builtin("token_ident")?,
            ])),
        ]))
    })?;

    // MODULE := "mod" MODULE_PATH
    let module = reg.declare_builtin("mod");
    reg.define_builtin(module, |r| {
        Some(r.seq([r.string("mod"), r.ref_rule(module_path)]))
    })?;

    // QUALIFIED_IDENT := MODULE_PATH "." IDENT
    let qualified_ident = reg.declare_builtin("qualified_ident");
    reg.define_builtin(qualified_ident, |r| {
        Some(r.seq([
            r.ref_rule(module_path),
            r.ref_builtin("delimiter_dot")?,
            r.ref_builtin("token_ident")?,
        ]))
    })?;

    let expr = reg.declare_builtin("expr");

    // PAREN_EXPR := "(" EXPR ")"
    let paren_expr = reg.declare_builtin("paren_expr");
    reg.define_builtin(paren_expr, |r| {
        Some(r.seq([
            r.ref_builtin("delimiter_open_paren")?,
            r.ref_rule(expr),
            r.ref_builtin("delimiter_close_paren")?,
        ]))
    })?;

    // TYPE_SPEC := ":" EXPR
    let type_spec = reg.declare_builtin("type_spec");
    reg.define_builtin(type_spec, |r| {
        Some(r.seq([r.ref_builtin("delimiter_colon")?, r.ref_rule(expr)]))
    })?;

    // BINDER_GROUP := IDENT+ TYPE_SPEC
    let binder_group = reg.declare_builtin("binder_group");
    reg.define_builtin(binder_group, |r| {
        Some(r.seq([r.plus(r.ref_builtin("token_ident")?), r.ref_rule(type_spec)]))
    })?;

    // EXPLICIT_SECTION := "(" BINDER_GROUP ")"
    let explicit_section = reg.declare_builtin("explicit_section");
    reg.define_builtin(explicit_section, |r| {
        Some(r.seq([
            r.ref_builtin("delimiter_open_paren")?,
            r.ref_rule(binder_group),
            r.ref_builtin("delimiter_close_paren")?,
        ]))
    })?;

    // IMPLICIT_SECTION := "{" BINDER_GROUP "}"
    let implicit_section = reg.declare_builtin("implicit_section");
    reg.define_builtin(implicit_section, |r| {
        Some(r.seq([
            r.ref_builtin("delimiter_open_brace")?,
            r.ref_rule(binder_group),
            r.ref_builtin("delimiter_close_brace")?,
        ]))
    })?;

    // PARAMS := (EXPLICIT_SECTION | IMPLICIT_SECTION)*
    let params = reg.declare_builtin("params");
    reg.define_builtin(params, |r| {
        Some(r.star(r.alt([r.ref_rule(explicit_section), r.ref_rule(implicit_section)])))
    })?;

    // LAMBDA := "fn" PARAMS TYPE_SPEC? "=>" EXPR
    let lambda = reg.declare_builtin("lambda");
    reg.define_builtin(lambda, |r| {
        Some(r.seq([
            r.ref_builtin("keyword_fn")?,
            r.ref_rule(params),
            r.opt(r.ref_rule(type_spec)),
            r.ref_builtin("delimiter_fat_arrow")?,
            r.ref_rule(expr),
        ]))
    })?;

    // FORALL := "forall" PARAMS TYPE_SPEC? "," EXPR
    let forall = reg.declare_builtin("forall");
    reg.define_builtin(forall, |r| {
        Some(r.seq([
            r.ref_builtin("keyword_forall")?,
            r.ref_rule(params),
            r.opt(r.ref_rule(type_spec)),
            r.ref_builtin("delimiter_comma")?,
            r.ref_rule(expr),
        ]))
    })?;

    let universe_level = reg.declare_builtin("universe_level");

    // PAREN_LEVEL := "(" UNIVERSE_LEVEL ")"
    let paren_level = reg.declare_builtin("paren_level");
    reg.define_builtin(paren_level, |r| {
        Some(r.seq([
            r.ref_builtin("delimiter_open_paren")?,
            r.ref_rule(universe_level),
            r.ref_builtin("delimiter_close_paren")?,
        ]))
    })?;

    // UNIVERSE_LEVEL := IDENT | "zero" | "succ" UNIVERSE_LEVEL | "max" UNIVERSE_LEVEL UNIVERSE_LEVEL | "(" UNIVERSE_LEVEL ")"
    reg.define_builtin(universe_level, |r| {
        Some(r.alt([
            r.ref_builtin("token_ident")?,
            r.ref_builtin("keyword_zero")?,
            r.seq([r.ref_builtin("keyword_succ")?, r.ref_rule(universe_level)]),
            r.seq([
                r.ref_builtin("keyword_max")?,
                r.ref_rule(universe_level),
                r.ref_rule(universe_level),
            ]),
            r.ref_rule(paren_level),
        ]))
    })?;

    // SORT_EXPR := "Sort" UNIVERSE_LEVEL?
    let sort_expr = reg.declare_builtin("sort_expr");
    reg.define_builtin(sort_expr, |r| {
        Some(r.seq([
            r.ref_builtin("keyword_sort")?,
            r.opt(r.ref_rule(universe_level)),
        ]))
    })?;

    // ATOMIC_EXPR := IDENT | SORT_EXPR | PAREN_EXPR | LAMBDA | FORALL
    let atomic_expr = reg.declare_builtin("atomic_expr");
    reg.define_builtin(atomic_expr, |r| {
        Some(r.alt([
            r.ref_builtin("token_ident")?,
            r.ref_rule(sort_expr),
            r.ref_rule(paren_expr),
            r.ref_rule(lambda),
            r.ref_rule(forall),
        ]))
    })?;

    // APP_EXPR := ATOMIC_EXPR ATOMIC_EXPR*
    let app_expr = reg.declare_builtin("app_expr");
    reg.define_builtin(app_expr, |r| {
        Some(r.seq([r.ref_rule(atomic_expr), r.star(r.ref_rule(atomic_expr))]))
    })?;

    // EXPR := APP_EXPR ("->" EXPR)?
    reg.define_builtin(expr, |r| {
        Some(r.seq([
            r.ref_rule(app_expr),
            r.opt(r.seq([r.ref_builtin("delimiter_arrow")?, r.ref_rule(expr)])),
        ]))
    })?;

    // CONST := "def" QUALIFIED_IDENT PARAMS TYPE_SPEC ":=" EXPR
    let constant = reg.declare_builtin("constant");
    reg.define_builtin(constant, |r| {
        Some(r.seq([
            r.ref_builtin("keyword_def")?,
            r.ref_rule(qualified_ident),
            r.ref_rule(params),
            r.ref_rule(type_spec),
            r.ref_builtin("delimiter_assign")?,
            r.ref_rule(expr),
        ]))
    })?;

    // CONSTRUCTOR := "|" IDENT PARAMS TYPE_SPEC
    let constructor = reg.declare_builtin("constructor");
    reg.define_builtin(constructor, |r| {
        Some(r.seq([
            r.ref_builtin("delimiter_pipe")?,
            r.ref_builtin("token_ident")?,
            r.ref_rule(params),
            r.ref_rule(type_spec),
        ]))
    })?;

    // IND := "ind" QUALIFIED_IDENT PARAMS TYPE_SPEC ":=" CONSTRUCTOR* "elim" IDENT
    let ind = reg.declare_builtin("ind");
    reg.define_builtin(ind, |r| {
        Some(r.seq([
            r.ref_builtin("keyword_ind")?,
            r.ref_rule(qualified_ident),
            r.ref_rule(params),
            r.ref_rule(type_spec),
            r.ref_builtin("delimiter_assign")?,
            r.star(r.ref_rule(constructor)),
            r.ref_builtin("keyword_elim")?,
            r.ref_builtin("token_ident")?,
        ]))
    })?;

    // ENV := (MODULE | IND | CONST)*
    let env = reg.declare_builtin("env");
    reg.define_builtin(env, |r| {
        Some(r.alt([r.ref_rule(module), r.ref_rule(ind), r.ref_rule(constant)]))
    })?;

    Some(reg)
}

fn define_lexical(reg: &mut RuleRegistry) -> Option<()> {
    let alphabetic = reg.declare_builtin("alphabetic");

    reg.define_builtin(alphabetic, |r| {
        Some(r.alt(('a'..='z').map(|c| r.literal(c))))
    })?;

    let identifier = reg.declare_builtin("token_ident");
    reg.define_builtin(identifier, |r| {
        Some(r.plus(r.alt([r.ref_rule(alphabetic), r.literal('_')])))
    })?;

    Some(())
}

fn define_tokens(reg: &mut RuleRegistry) -> Option<()> {
    const TOKENS: [(&str, &str); 25] = [
        ("delimiter_pipe", "|"),
        ("delimiter_colon", ":"),
        ("delimiter_comma", ","),
        ("delimiter_dot", "."),
        ("delimiter_placeholder", "_"),
        ("delimiter_open_paren", "("),
        ("delimiter_close_paren", ")"),
        ("delimiter_open_brace", "{"),
        ("delimiter_close_brace", "}"),
        ("delimiter_assign", ":="),
        ("delimiter_arrow", "->"),
        ("delimiter_fat_arrow", "=>"),
        ("delimiter_square", "::"),
        ("keyword_fn", "fn"),
        ("keyword_forall", "forall"),
        ("keyword_mod", "mod"),
        ("keyword_ind", "ind"),
        ("keyword_def", "def"),
        ("keyword_elim", "elim"),
        ("keyword_self", "Self"),
        ("keyword_prop", "Prop"),
        ("keyword_sort", "Sort"),
        ("keyword_max", "max"),
        ("keyword_succ", "succ"),
        ("keyword_zero", "zero"),
    ];

    for (name, literal) in TOKENS {
        let rule = reg.declare_builtin(name);
        reg.define_builtin(rule, |r| Some(r.string(literal)))?;
    }
    Some(())
}
