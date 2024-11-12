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

impl Part {
    fn category_value(&self, category: Category) -> u32 {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
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

impl Cmp {
    fn into_fn(self) -> impl Fn(&u32, &u32) -> bool {
        match self {
            Self::Less => u32::lt,
            Self::Greater => u32::gt,
        }
    }
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

impl Rule<'_> {
    fn apply_at(&self, part: &Part) -> Option<Destination> {
        let Rule {
            category,
            cmp,
            value,
            dest,
        } = self;

        let category_value = part.category_value(*category);
        if cmp.into_fn()(&category_value, value) {
            Some(*dest)
        } else {
            None
        }
    }
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

    fn is_accepted(&self, part: &Part) -> bool {
        matches!(self.consider(part), Destination::Accept)
    }

    fn consider(&self, part: &Part) -> Destination {
        self.consider_inner(part, "in")
    }

    fn consider_inner(&self, part: &Part, workflow_name: &str) -> Destination {
        let WorkflowInner {
            rules,
            last_rule: last,
        } = &self.inner[workflow_name];

        for rule in rules {
            match rule.apply_at(part) {
                None => {
                    // rule doesn't apply
                    continue;
                }
                Some(Destination::Workflow(wf_name)) => {
                    // send to another workflow
                    return self.consider_inner(part, wf_name);
                }
                Some(dest) => return dest,
            }
        }

        match *last {
            Destination::Workflow(wf_name) => {
                // send to another workflow
                self.consider_inner(part, wf_name)
            }
            dest => dest,
        }
    }
}
