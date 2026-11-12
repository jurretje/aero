use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
};

use serde::{Deserialize, Serialize};

use crate::{
    grammar::RuleName,
    graph::{GrammarGraph, StateId, Transition},
    node::{Node, NodeKind},
    source::Span,
    token::{Token, TokenKind},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Config {
    pub state: StateId,
    pub index: usize,
    pub stack: StackId,
}

impl Config {
    pub fn from(graph: &GrammarGraph) -> Self {
        Self {
            state: graph.start,
            index: 0,
            stack: 0,
        }
    }
}

type StackId = usize;

#[derive(Clone, Copy)]
struct StackNode {
    parent: StackId,
    frame: ReturnFrame,
}

struct StackArena {
    nodes: Vec<StackNode>,
    intern: HashMap<(StackId, ReturnFrame), StackId>,
}

impl StackArena {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            intern: HashMap::new(),
        }
    }

    fn push(&mut self, parent: StackId, frame: ReturnFrame) -> StackId {
        let key = (parent, frame);

        if let Some(&id) = self.intern.get(&key) {
            return id;
        }

        let id = self.nodes.len() + 1;
        self.nodes.push(StackNode { parent, frame });
        self.intern.insert(key, id);
        id
    }

    fn top(&self, stack: StackId) -> Option<StackNode> {
        if stack == 0 {
            None
        } else {
            self.nodes.get(stack - 1).copied()
        }
    }

    fn pop(&self, stack: StackId) -> Option<(StackId, ReturnFrame)> {
        let node = self.top(stack)?;
        Some((node.parent, node.frame))
    }

    fn is_empty(&self, stack: StackId) -> bool {
        stack == 0
    }
}

#[derive(Clone, Debug)]
pub enum Action {
    Match(TokenKind, Span),
    EnterRule(RuleName),
    ExitRule(RuleName),
    Epsilon,
    Insert(TokenKind, usize),
    Delete(usize),
    Replace(TokenKind, Token),
}

impl Action {
    pub fn is_edit(&self) -> bool {
        match self {
            Action::Match(..) => false,
            Action::Insert(..) => true,
            Action::Delete(..) => true,
            Action::Replace(..) => true,
            Action::EnterRule(..) => false,
            Action::ExitRule(..) => false,
            Action::Epsilon => false,
        }
    }
}

fn for_each_neighbor(
    graph: &GrammarGraph,
    config: Config,
    tokens: &[Token],
    costs: ParseCosts,
    stacks: &mut StackArena,
    mut visit: impl FnMut(Config, usize, Action),
) {
    for &transition in &graph.states[config.state].transitions {
        match transition {
            Transition::Call {
                name,
                entry,
                return_state,
            } => {
                let stack = stacks.push(
                    config.stack,
                    ReturnFrame {
                        rule: name,
                        return_state,
                    },
                );

                visit(
                    Config {
                        state: entry,
                        stack,
                        ..config
                    },
                    0,
                    Action::EnterRule(name),
                );
            }

            Transition::Return { name } => {
                if let Some((parent, frame)) = stacks.pop(config.stack) {
                    if frame.rule == name {
                        visit(
                            Config {
                                state: frame.return_state,
                                stack: parent,
                                ..config
                            },
                            0,
                            Action::ExitRule(name),
                        );
                    }
                }
            }

            Transition::Token { kind, target } => {
                if let Some(token) = tokens.get(config.index) {
                    let next = Config {
                        state: target,
                        index: config.index + 1,
                        ..config
                    };

                    if token.kind == kind {
                        visit(next, 0, Action::Match(kind, token.span.clone()));
                    } else {
                        visit(next, costs.replace, Action::Replace(kind, token.clone()));
                    }
                }

                visit(
                    Config {
                        state: target,
                        ..config
                    },
                    costs.insert,
                    Action::Insert(kind, config.index),
                );
            }

            Transition::Epsilon { target } => {
                visit(
                    Config {
                        state: target,
                        ..config
                    },
                    0,
                    Action::Epsilon,
                );
            }
        }
    }

    if config.index < tokens.len() {
        visit(
            Config {
                index: config.index + 1,
                ..config
            },
            costs.delete,
            Action::Delete(config.index),
        );
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct QueueEntry {
    cost: usize,
    sequence: usize,
    node_id: usize,
}

impl Ord for QueueEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.sequence.cmp(&self.sequence))
    }
}

impl PartialOrd for QueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct SearchNode {
    config: Config,
    previous: Option<(usize, Action)>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
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

#[derive(Debug)]
pub struct ParserOutput {
    pub actions: Vec<Action>,
    pub cost: Option<usize>,
    pub stats: ParseStats,
}

#[derive(Debug, Default)]
pub struct ParseStats {
    pub popped: usize,
    pub expanded: usize,
    pub generated: usize,
    pub relaxed: usize,
    pub peak_queue: usize,
    pub visited_configurations: usize,
    pub search_nodes: usize,
}

pub fn parse(graph: &GrammarGraph, tokens: &[Token], costs: ParseCosts) -> ParserOutput {
    let start = Config::from(graph);
    let mut stacks = StackArena::new();
    let mut stats = ParseStats::default();

    let mut nodes = vec![SearchNode {
        config: start,
        previous: None,
    }];

    let mut best: HashMap<Config, (usize, usize)> = HashMap::new();
    best.insert(start, (0, 0));

    let mut queue = BinaryHeap::new();
    let mut sequence = 0;

    queue.push(QueueEntry {
        cost: 0,
        sequence,
        node_id: 0,
    });

    while let Some(QueueEntry { cost, node_id, .. }) = queue.pop() {
        stats.popped += 1;

        let config = nodes[node_id].config;

        if best.get(&config) != Some(&(cost, node_id)) {
            continue;
        }

        if graph.accepts(&config.state)
            && config.index == tokens.len()
            && stacks.is_empty(config.stack)
        {
            stats.visited_configurations = best.len();
            stats.search_nodes = nodes.len();
            return ParserOutput {
                actions: reconstruct(&nodes, node_id),
                cost: Some(cost),
                stats,
            };
        }

        stats.expanded += 1;

        for_each_neighbor(
            graph,
            config,
            tokens,
            costs,
            &mut stacks,
            |next, edge_cost, action| {
                stats.generated += 1;

                let Some(next_cost) = cost.checked_add(edge_cost) else {
                    return;
                };

                if best
                    .get(&next)
                    .is_some_and(|&(old_cost, _)| next_cost >= old_cost)
                {
                    return;
                }

                stats.relaxed += 1;

                let next_id = nodes.len();

                nodes.push(SearchNode {
                    config: next,
                    previous: Some((node_id, action)),
                });

                best.insert(next, (next_cost, next_id));

                sequence = sequence.wrapping_add(1);
                queue.push(QueueEntry {
                    cost: next_cost,
                    sequence,
                    node_id: next_id,
                });
                stats.peak_queue = stats.peak_queue.max(queue.len())
            },
        );
    }

    ParserOutput {
        actions: vec![],
        cost: None,
        stats,
    }
}

fn reconstruct(nodes: &[SearchNode], mut current: usize) -> Vec<Action> {
    let mut actions = Vec::new();

    while let Some((previous, action)) = &nodes[current].previous {
        actions.push(action.clone());
        current = *previous
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
