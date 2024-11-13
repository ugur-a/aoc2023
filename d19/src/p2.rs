use core::{
    cmp,
    ops::{Index, IndexMut, RangeInclusive},
};

use anyhow::Context;
use itertools::Itertools;

use crate::{Category, Cmp, Destination, Rule, Workflow, WorkflowInner, WorkflowName, Workflows};

#[derive(Debug, Clone)]
struct PartRange {
    x: RangeInclusive<u32>,
    m: RangeInclusive<u32>,
    a: RangeInclusive<u32>,
    s: RangeInclusive<u32>,
}

impl Index<&Category> for PartRange {
    type Output = RangeInclusive<u32>;
    fn index(&self, category: &Category) -> &Self::Output {
        match *category {
            Category::X => &self.x,
            Category::M => &self.m,
            Category::A => &self.a,
            Category::S => &self.s,
        }
    }
}

impl IndexMut<&Category> for PartRange {
    fn index_mut(&mut self, category: &Category) -> &mut Self::Output {
        match *category {
            Category::X => &mut self.x,
            Category::M => &mut self.m,
            Category::A => &mut self.a,
            Category::S => &mut self.s,
        }
    }
}

impl Rule<'_> {
    fn apply_at_range(&self, part_range: &PartRange) -> (PartRange, (PartRange, Destination<'_>)) {
        let Rule {
            category,
            cmp,
            value,
            dest,
        } = self;

        let category_value_range = &part_range[category];
        let (&start, &end) = (category_value_range.start(), category_value_range.end());

        let (changed_range, unchanged_range) = match cmp {
            Cmp::Less => {
                let max_changed = cmp::min(end, *value - 1);
                (start..=max_changed, *value..=end)
            }
            Cmp::Greater => {
                let min_changed = cmp::max(start, *value + 1);
                (min_changed..=end, start..=*value)
            }
        };

        let mut copy_unchanged = part_range.clone();
        copy_unchanged[category] = unchanged_range;

        let mut copy_changed = part_range.clone();
        copy_changed[category] = changed_range;

        (copy_unchanged, (copy_changed, *dest))
    }
}

impl Workflows<'_> {
    fn consider_range(&self, part_range: PartRange) -> Vec<(PartRange, Destination)> {
        self.consider_range_inner(part_range, "in")
    }

    fn consider_range_inner(
        &self,
        mut part_range: PartRange,
        workflow_name: WorkflowName,
    ) -> Vec<(PartRange, Destination)> {
        let WorkflowInner {
            rules,
            last_rule: last,
        } = &self.inner[workflow_name];

        let mut res = vec![];

        for rule in rules {
            let (unchanged, (changed, destination)) = rule.apply_at_range(&part_range);
            part_range = unchanged;
            match destination {
                Destination::Workflow(wf_name) => {
                    // send to another workflow
                    res.extend(self.consider_range_inner(changed, wf_name));
                }
                dest => {
                    res.push((changed, dest));
                }
            }
        }

        match *last {
            Destination::Workflow(_) => {
                // res.extend(self.consider_range_inner(part_range, wf_name));
                unreachable!()
            }
            dest => {
                res.push((part_range, dest));
            }
        }
        res
    }
}

pub fn p2(file: &str) -> anyhow::Result<usize> {
    let (workflows, _) = file
        .split_once("\n\n")
        .context("no empty line between workflows and parts")?;

    let workflows = workflows.lines().map(Workflow::try_from).try_collect()?;
    let workflows = Workflows::new(workflows);

    let part_range = PartRange {
        x: 1..=4000,
        m: 1..=4000,
        a: 1..=4000,
        s: 1..=4000,
    };

    let destinations = workflows.consider_range(part_range);

    let res: usize = destinations
        .into_iter()
        .filter(|(_, dest)| matches!(dest, Destination::Accept))
        .map(|(pr, _)| {
            let PartRange { x, m, a, s } = pr;
            x.count() * m.count() * a.count() * s.count()
        })
        .sum();

    Ok(res)
}

#[cfg(test)]
mod test {
    use test_case::test_case;
    const EXAMPLE: &str = include_str!("../inputs/example.txt");
    const REAL: &str = include_str!("../inputs/real.txt");

    #[test_case(EXAMPLE => 167_409_079_868_000)]
    #[test_case(REAL => 110_807_725_108_076)]
    fn p2(inp: &str) -> usize {
        super::p2(inp).unwrap()
    }
}
