use libaoc::{impl_from_str_for_obj_with_lifetimes_from_nom_parser, impl_from_str_from_nom_parser};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, anychar, char, u32},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

use crate::{Category, Cmp, Destination, Part, Rule, Workflow, WorkflowInner};

impl TryFrom<char> for Category {
    type Error = anyhow::Error;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        let res = match c {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            c => anyhow::bail!("invalid category: {c}"),
        };
        Ok(res)
    }
}

fn category(i: &str) -> IResult<&str, Category> {
    map_res(anychar, Category::try_from)(i)
}

impl TryFrom<char> for Cmp {
    type Error = anyhow::Error;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        let res = match c {
            '<' => Self::Less,
            '>' => Self::Greater,
            c => anyhow::bail!("invalid comparison symbol: {c}"),
        };
        Ok(res)
    }
}

fn cmp(i: &str) -> IResult<&str, Cmp> {
    map_res(anychar, Cmp::try_from)(i)
}

fn value(i: &str) -> IResult<&str, u32> {
    u32(i)
}

fn rule(i: &str) -> IResult<&str, Rule> {
    map(
        separated_pair(tuple((category, cmp, value)), char(':'), destination),
        |((category, cmp, value), dest)| Rule {
            category,
            cmp,
            value,
            dest,
        },
    )(i)
}

fn last_rule(i: &str) -> IResult<&str, Destination> {
    destination(i)
}

fn workflow_name(i: &str) -> IResult<&str, &str> {
    alpha1(i)
}

fn destination(i: &str) -> IResult<&str, Destination> {
    alt((
        map(char('A'), |_| Destination::Accept),
        map(char('R'), |_| Destination::Reject),
        map(workflow_name, Destination::Workflow),
    ))(i)
}

fn rules(i: &str) -> IResult<&str, Vec<Rule>> {
    separated_list0(char(','), rule)(i)
}

fn workflow_inner(i: &str) -> IResult<&str, WorkflowInner> {
    map(
        delimited(
            char('{'),
            separated_pair(rules, char(','), last_rule),
            char('}'),
        ),
        |(rules, last_rule)| WorkflowInner { rules, last_rule },
    )(i)
}

fn workflow(i: &str) -> IResult<&str, Workflow> {
    map(tuple((workflow_name, workflow_inner)), |(name, inner)| {
        Workflow { name, inner }
    })(i)
}

impl_from_str_for_obj_with_lifetimes_from_nom_parser!(workflow, Workflow);

fn part(i: &str) -> IResult<&str, Part> {
    map(
        delimited(
            char('{'),
            tuple((
                preceded(tag("x="), u32),
                preceded(tag(",m="), u32),
                preceded(tag(",a="), u32),
                preceded(tag(",s="), u32),
            )),
            char('}'),
        ),
        |(x, m, a, s)| Part { x, m, a, s },
    )(i)
}

impl_from_str_from_nom_parser!(part, Part);
