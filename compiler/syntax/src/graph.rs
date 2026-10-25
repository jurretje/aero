use std::collections::{HashMap, HashSet};

use serde::Serialize;

use crate::{
    grammar::{Grammar, RuleKind, RuleName},
    token::TokenKind,
};

pub type StateId = usize;

#[derive(Serialize, Debug)]
pub struct GrammarGraph {
    pub states: Vec<State>,
    pub start: StateId,
    accepting: HashSet<StateId>,
}

impl GrammarGraph {
    pub fn accepts(&self, state: &StateId) -> bool {
        self.accepting.contains(state)
    }
}

#[derive(Default, Serialize, Debug)]
pub struct State {
    pub transitions: Vec<Transition>,
}

#[derive(Clone, Copy, Serialize, Debug)]
pub enum Transition {
    Token {
        kind: TokenKind,
        target: StateId,
    },
    Call {
        name: RuleName,
        entry: StateId,
        return_state: StateId,
    },
    Return {
        name: RuleName,
    },
    Epsilon {
        target: StateId,
    },
}

pub struct GrammarGraphBuilder<'g> {
    grammar: &'g Grammar,
    graph: GrammarGraph,
    entries: HashMap<RuleName, StateId>,
}

impl<'g> GrammarGraphBuilder<'g> {
    pub fn new(grammar: &'g Grammar) -> Self {
        Self {
            grammar,
            graph: GrammarGraph {
                states: vec![],
                start: 0,
                accepting: HashSet::new(),
            },
            entries: HashMap::new(),
        }
    }

    fn new_state(&mut self) -> StateId {
        let id = self.graph.states.len();
        self.graph.states.push(State::default());
        id
    }

    fn add_transition(&mut self, from: StateId, transition: Transition) {
        self.graph.states[from].transitions.push(transition);
    }

    fn compile_rule(&mut self, name: RuleName) -> StateId {
        if let Some(&existing) = self.entries.get(&name) {
            return existing;
        }

        let entry = self.new_state();
        self.entries.insert(name, entry);
        let exit = self.new_state();

        self.compile_kind(&self.grammar.get(name).kind, entry, exit);
        self.add_transition(exit, Transition::Return { name });

        entry
    }

    fn compile_kind(
        &mut self,
        kind: &RuleKind,
        entry: StateId,
        exit: StateId,
    ) -> (StateId, StateId) {
        match kind {
            RuleKind::Token(token) => {
                self.add_transition(
                    entry,
                    Transition::Token {
                        kind: *token,
                        target: exit,
                    },
                );
                (entry, exit)
            }
            RuleKind::Ref(name) => {
                let target = self.compile_rule(*name);
                self.add_transition(
                    entry,
                    Transition::Call {
                        name: *name,
                        entry: target,
                        return_state: exit,
                    },
                );
                (entry, exit)
            }
            RuleKind::Seq(items) => {
                if items.is_empty() {
                    return (entry, exit);
                }

                let mut current = entry;
                for (i, item) in items.iter().enumerate() {
                    let next = if i + 1 == items.len() {
                        exit
                    } else {
                        self.new_state()
                    };
                    self.compile_kind(item, current, next);
                    current = next;
                }
                (entry, exit)
            }
            RuleKind::Alt(options) => {
                for option in options {
                    let branch_start = self.new_state();
                    let branch_end = self.new_state();
                    self.add_transition(
                        entry,
                        Transition::Epsilon {
                            target: branch_start,
                        },
                    );
                    self.compile_kind(option, branch_start, branch_end);
                    self.add_transition(branch_end, Transition::Epsilon { target: exit });
                }
                (entry, exit)
            }
            RuleKind::Question(inner) => {
                let inner_start = self.new_state();
                let inner_end = self.new_state();

                self.add_transition(entry, Transition::Epsilon { target: exit });

                self.add_transition(entry, Transition::Epsilon { target: inner_start });
                self.compile_kind(inner, inner_start, inner_end);

                self.add_transition(inner_end, Transition::Epsilon { target: exit });

                (entry, exit)
            }
            RuleKind::Plus(inner) => {
                let inner_start = self.new_state();
                let inner_end = self.new_state();

                self.add_transition(entry, Transition::Epsilon { target: inner_start });
                self.compile_kind(inner, inner_start, inner_end);
                self.add_transition(inner_end, Transition::Epsilon { target: exit });
                self.add_transition(inner_end, Transition::Epsilon { target: inner_start });
                (entry, exit)
            }
            RuleKind::Star(inner) => {
                let inner_start = self.new_state();
                let inner_end = self.new_state();

                self.add_transition(entry, Transition::Epsilon { target: exit });
                self.add_transition(entry, Transition::Epsilon { target: inner_start });
                self.compile_kind(inner, inner_start, inner_end);
                self.add_transition(inner_end, Transition::Epsilon { target: exit });
                self.add_transition(inner_end, Transition::Epsilon { target: inner_start });

                (entry, exit)
            }
        }
    }

    pub fn build(mut self) -> GrammarGraph {
        let start = self.new_state();
        let end = self.new_state();

        let entry = self.compile_rule(self.grammar.starting_rule);
        self.add_transition(
            start,
            Transition::Call {
                name: self.grammar.starting_rule,
                entry,
                return_state: end,
            },
        );

        self.graph.start = start;
        self.graph.accepting = HashSet::from_iter(vec![end]);

        self.graph
    }
}
