use core::str::FromStr;

use anyhow::Context;
use itertools::Itertools;

use crate::{Cmp, Destination, Part, Rule, Workflow, WorkflowInner, WorkflowName, Workflows};

impl Rule<'_> {
    fn apply_at(&self, part: &Part) -> Option<Destination> {
        let Rule {
            category,
            cmp,
            value,
            dest,
        } = self;

        let category_value = &part[category];

        let rule_applies = match cmp {
            Cmp::Less => category_value < value,
            Cmp::Greater => category_value > value,
        };

        if rule_applies {
            Some(*dest)
        } else {
            None
        }
    }
}

impl Workflows<'_> {
    fn is_accepted(&self, part: &Part) -> bool {
        matches!(self.consider(part), Destination::Accept)
    }

    fn consider(&self, part: &Part) -> Destination {
        self.consider_inner(part, "in")
    }

    fn consider_inner(&self, part: &Part, workflow_name: WorkflowName) -> Destination {
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

pub fn p1(file: &str) -> anyhow::Result<u32> {
    let (workflows, parts) = file
        .split_once("\n\n")
        .context("no empty line between workflows and parts")?;

    let workflows = workflows.lines().map(Workflow::try_from).try_collect()?;
    let workflows = Workflows::new(workflows);

    let parts: Vec<_> = parts.lines().map(Part::from_str).try_collect()?;

    let res = parts
        .into_iter()
        .filter(|p| workflows.is_accepted(p))
        .map(|Part { x, m, a, s }| x + m + a + s)
        .sum();
    Ok(res)
}

#[cfg(test)]
mod test {
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 19114)]
    #[test_case(REAL => 319_295)]
    fn p1(inp: &str) -> u32 {
        super::p1(inp).unwrap()
    }
}
