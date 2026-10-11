use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
};

use crate::{
    grammar::RuleName,
    graph::{GrammarGraph, StateId, Transition},
    node::{Node, NodeKind},
    source::Span,
    token::{Token, TokenKind},
};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Config {
    pub state: StateId,
    pub index: usize,
    pub stack: Vec<ReturnFrame>,
}

impl Config {
    pub fn from(graph: &GrammarGraph) -> Self {
        Self {
            state: graph.start,
            index: 0,
            stack: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub enum Action {
    Match(TokenKind, Span),
    Insert(TokenKind, usize),
    Delete(usize),
    Replace(TokenKind, Token),
    EnterRule(RuleName),
    ExitRule(RuleName),
    Epsilon,
}

#[derive(Clone, PartialEq, Eq)]
struct QueueEntry {
    cost: usize,
    config: Config,
}

impl Ord for QueueEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for QueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ParseCosts {
    pub insert: usize,
    pub delete: usize,
    pub replace: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ReturnFrame {
    pub rule: RuleName,
    pub return_state: StateId,
}

type Neighbor = (Config, usize, Action);

fn call_neighbor(
    config: &Config,
    name: RuleName,
    entry: StateId,
    return_state: StateId,
) -> Neighbor {
    let mut next = config.clone();
    next.state = entry;

    next.stack.push(ReturnFrame {
        rule: name,
        return_state,
    });

    (next, 0, Action::EnterRule(name))
}

fn return_neighbor(config: &Config, name: RuleName) -> Option<Neighbor> {
    let mut next = config.clone();

    let frame = next.stack.pop()?;

    if frame.rule != name {
        return None;
    }

    next.state = frame.return_state;

    Some((next, 0, Action::ExitRule(name)))
}

fn token_neighbors(
    config: &Config,
    kind: TokenKind,
    target: StateId,
    tokens: &[Token],
    costs: ParseCosts,
) -> Vec<Neighbor> {
    let mut result = Vec::with_capacity(2);

    if let Some(token) = tokens.get(config.index) {
        let mut next = config.clone();
        next.state = target;
        next.index += 1;

        let neighbor = if token.kind == kind {
            (next, 0, Action::Match(kind, token.span.clone()))
        } else {
            (next, costs.replace, Action::Replace(kind, token.clone()))
        };

        result.push(neighbor);
    }

    let mut inserted = config.clone();
    inserted.state = target;
    result.push((inserted, costs.insert, Action::Insert(kind, config.index)));

    result
}

fn neighbors(
    graph: &GrammarGraph,
    config: &Config,
    tokens: &[Token],
    costs: ParseCosts,
) -> Vec<(Config, usize, Action)> {
    let mut result = Vec::new();

    for &transition in &graph.states[config.state].transitions {
        match transition {
            Transition::Call {
                name,
                entry,
                return_state,
            } => {
                result.push(call_neighbor(config, name, entry, return_state));
            }

            Transition::Return { name } => {
                if let Some(neighbor) = return_neighbor(config, name) {
                    result.push(neighbor);
                }
            }

            Transition::Token { kind, target } => {
                result.extend(token_neighbors(config, kind, target, tokens, costs));
            }
        }
    }

    if config.index < tokens.len() {
        let mut next = config.clone();
        next.index += 1;

        result.push((next, costs.delete, Action::Delete(config.index)));
    }

    result
}

pub fn parse(
    graph: &GrammarGraph,
    tokens: &[Token],
    costs: ParseCosts,
) -> Option<Vec<Action>> {
    let start = Config::from(graph);

    let mut queue = BinaryHeap::new();
    queue.push(QueueEntry {
        cost: 0,
        config: start.clone(),
    });

    let mut distance = HashMap::new();
    distance.insert(start, 0);

    let mut previous = HashMap::new();

    while let Some(QueueEntry { cost, config }) = queue.pop() {
        dbg!(&config);
        let ending_condition =
            graph.accepts(&config.state) && config.index == tokens.len() && config.stack.is_empty();

        if ending_condition {
            return Some(reconstruct(previous, config));
        }

        if distance.get(&config).is_some_and(|best| cost > *best) {
            continue;
        }

        for (next, edge_cost, action) in neighbors(graph, &config, tokens, costs) {
            let next_cost = cost + edge_cost;

            if next_cost < distance.get(&next).copied().unwrap_or(usize::MAX) {
                distance.insert(next.clone(), next_cost);
                previous.insert(next.clone(), (config.clone(), action));

                queue.push(QueueEntry {
                    cost: next_cost,
                    config: next,
                });
            }
        }
    }

    None
}

fn reconstruct(
    mut previous: HashMap<Config, (Config, Action)>,
    mut current: Config,
) -> Vec<Action> {
    let mut actions = Vec::new();

    while let Some((from, action)) = previous.remove(&current) {
        actions.push(action);
        current = from;
    }

    actions.reverse();
    actions
}

pub struct BuildFrame {
    children: Vec<Node>,
    start: Option<usize>,
}

fn combine_spans(children: &[Node]) -> Option<Span> {
    let first = children.first()?;
    let last = children.last()?;

    Some(Span {
        start: first.span.start,
        end: last.span.end,
    })
}

pub fn build_ast(actions: &[Action], tokens: &[Token]) -> Option<Node> {
    let mut stack = Vec::new();
    let mut roots = Vec::new();
    let mut index = 0;

    for action in actions {
        match action {
            Action::EnterRule(_) => {
                stack.push(BuildFrame {
                    children: Vec::new(),
                    start: Some(index),
                });
            }

            Action::Match(kind, ..) => {
                let token = tokens.get(index)?;

                let node = Node {
                    kind: NodeKind::Token(*kind),
                    span: token.span.clone(),
                    children: Vec::new(),
                };

                index += 1;

                if let Some(frame) = stack.last_mut() {
                    frame.children.push(node);
                }
            }

            Action::Replace(_, _) | Action::Delete(_) => {
                index += 1;
            }

            Action::ExitRule(rule) => {
                let frame = stack.pop()?;

                let span = combine_spans(&frame.children).or_else(|| {
                    frame.start.and_then(|i| {
                        let pos = tokens.get(i)?.span.start;

                        Some(Span {
                            start: pos,
                            end: pos,
                        })
                    })
                })?;

                let node = Node {
                    kind: NodeKind::Rule(*rule),
                    span,
                    children: frame.children,
                };

                if let Some(parent) = stack.last_mut() {
                    parent.children.push(node);
                } else {
                    roots.push(node);
                }
            }

            Action::Insert(..) | Action::Epsilon => {}
        }
    }

    roots.into_iter().next()
}
