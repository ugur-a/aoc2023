use core::ops::{Index, IndexMut, RangeInclusive};

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
        // let min = core::cmp::min(category_value_range.start, *value);
        // let max = core::cmp::max(category_value_range.end, *value);

        let (changed_range, unchanged_range) = match cmp {
            #[allow(clippy::range_minus_one)]
            Cmp::Less => {
                if *value <= *category_value_range.start() {
                    (None, Some(category_value_range.clone()))
                } else if *value < *category_value_range.end() {
                    (
                        Some(*category_value_range.start()..=*value - 1),
                        Some(*value..=*category_value_range.end()),
                    )
                } else {
                    (Some(category_value_range.clone()), None)
                }
            }
            Cmp::Greater => {
                if *value > *category_value_range.end() {
                    (None, Some(category_value_range.clone()))
                } else if *value >= *category_value_range.start() {
                    (
                        Some(*value + 1..=*category_value_range.end()),
                        Some(*category_value_range.start()..=*value),
                    )
                } else {
                    (Some(category_value_range.clone()), None)
                }
            }
        };

        let mut copy_unchanged = part_range.clone();
        if let Some(unchanged_range) = unchanged_range {
            copy_unchanged[category] = unchanged_range;
        }

        let mut copy_changed = part_range.clone();
        if let Some(changed_range) = changed_range {
            copy_changed[category] = changed_range;
        }

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
                // unreachable!("no workflow-as-last-rule after optimisation 1")
                for rule in rules {
                    let category = match rule.category {
                        Category::X => 'x',
                        Category::M => 'm',
                        Category::A => 'a',
                        Category::S => 's',
                    };
                    let cmp = match rule.cmp {
                        Cmp::Less => '<',
                        Cmp::Greater => '>',
                    };
                    let value = rule.value;
                    let dest = match rule.dest {
                        Destination::Accept => "A",
                        Destination::Reject => "R",
                        Destination::Workflow(s) => s,
                    };
                    eprint!("{category}{cmp}{value}:{dest},");
                }
                eprintln!("last={last:?}");
            }
            dest => {
                res.push((part_range.clone(), dest));
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
        x: 1..4000 + 1,
        m: 1..4000 + 1,
        a: 1..4000 + 1,
        s: 1..4000 + 1,
    };

    let destinations = workflows.consider_range(part_range);

    let res: usize = destinations
        .into_iter()
        // .inspect(|(pr, dest)| {
        //     let dest = match dest {
        //         Destination::Accept => "accepted",
        //         Destination::Reject => "rejected",
        //         Destination::Workflow(_) => unreachable!(),
        //     };
        //     eprintln!("{pr:?}- {dest}");
        // })
        .filter(|(_, dest)| matches!(dest, Destination::Accept))
        // .inspect(|(pr, _)| eprintln!("{pr:?}"))
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

    // PartRange { x:    1..1416, m:    1..4001, a:    1..2006, s:    1..1351 }- accepted
    // PartRange { x: 2662..4001, m:    1..4001, a:    1..2006, s:    1..1351 }- accepted
    // PartRange { x: 1416..2662, m:    1..4001, a:    1..2006, s:    1..1351 }- rejected
    // PartRange { x:    1..4001, m: 2090..4001, a: 2006..4001, s:    1..1351 }- accepted
    // PartRange { x:    1..4001, m:    1..2090, a: 2006..4001, s:    1.. 537 }- rejected
    // PartRange { x: 2440..4001, m:    1..2090, a: 2006..4001, s:  537..1351 }- rejected
    // PartRange { x:    1..2440, m:    1..2090, a: 2006..4001, s:  537..1351 }- accepted
    // PartRange { x:    1..4001, m:    1..4001, a:    1..4001, s: 2770..4001 }- accepted
    // PartRange { x:    1..4001, m:  838..1801, a:    1..4001, s: 1351..2770 }- accepted
    // PartRange { x:    1..4001, m:    1.. 838, a: 1716..4001, s: 1351..2770 }- rejected
    // PartRange { x:    1..4001, m:    1.. 838, a:    1..1716, s: 1351..2770 }- accepted
    // PartRange { x:    1..4001, m: 1801..4001, a:    1..4001, s: 1351..2770 }- rejected
    #[test_case(EXAMPLE => 167_409_079_868_000)]
    #[test_case(REAL => 0)] // 92_712_585_759_730 < x
    fn p2(inp: &str) -> usize {
        super::p2(inp).unwrap()
    }
}
