use crate::general::rules::RuleRegistry;

pub fn bootstrap() -> RuleRegistry {
    let mut g = RuleRegistry::default();

    let env = g.declare_rule();
    let module = g.declare_rule();
    let rule_declaration = g.declare_rule();
    let expr = g.declare_rule();

    g
}
