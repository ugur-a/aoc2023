use core::ops::Index;
use std::collections::HashMap;

pub mod p1;
pub mod p2;
mod parse;

#[derive(Clone, Copy)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Index<&Category> for Part {
    type Output = u32;
    fn index(&self, category: &Category) -> &Self::Output {
        match *category {
            Category::X => &self.x,
            Category::M => &self.m,
            Category::A => &self.a,
            Category::S => &self.s,
        }
    }
}

type WorkflowName<'a> = &'a str;

#[derive(Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Clone, Copy)]
enum Cmp {
    Less,
    Greater,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Destination<'a> {
    Accept,
    Reject,
    Workflow(WorkflowName<'a>),
}

#[derive(Clone, Copy)]
struct Rule<'a> {
    category: Category,
    cmp: Cmp,
    value: u32,
    dest: Destination<'a>,
}

struct Workflow<'a> {
    name: WorkflowName<'a>,
    inner: WorkflowInner<'a>,
}

#[derive(Clone)]
struct WorkflowInner<'a> {
    rules: Vec<Rule<'a>>,
    last_rule: Destination<'a>,
}

struct Workflows<'a> {
    inner: HashMap<&'a str, WorkflowInner<'a>>,
}

impl<'a> Workflows<'a> {
    fn new(workflows: Vec<Workflow<'a>>) -> Self {
        let mut workflows_map: HashMap<_, _> = workflows
            .into_iter()
            .map(|Workflow { name, inner }| (name, inner))
            .collect();

        // optimize: simplify rule chains of forms:
        // - `X:Y,?:A,?:A,...,A` - to `X:Y,A`
        // - `X:Y,?:R,?:R,...,R` - to `X:Y,R`
        for w_inner in workflows_map.values_mut() {
            let to_truncate = w_inner
                .rules
                .iter()
                .rev()
                .take_while(|rule| rule.dest == w_inner.last_rule)
                .count();
            w_inner.rules.truncate(w_inner.rules.len() - to_truncate);
        }

        Self {
            inner: workflows_map,
        }
    }
}
