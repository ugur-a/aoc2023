use std::collections::HashMap;

use anyhow::bail;
use libaoc::impl_from_str_for_obj_with_lifetimes_from_nom_parser;
use nom::{
    bytes::complete::{tag, take},
    character::complete::{anychar, char, newline},
    combinator::{map, map_res},
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Debug, Clone, Copy)]
pub(crate) enum MoveDirection {
    Left,
    Right,
}

impl TryFrom<char> for MoveDirection {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let res = match value {
            'L' => Self::Left,
            'R' => Self::Right,
            c => bail!("invalid direction: {c}"),
        };
        Ok(res)
    }
}

#[derive(Debug)]
pub(crate) struct Moves(pub(crate) Vec<MoveDirection>);

type NodeName<'a> = &'a str;

#[derive(Debug)]
struct Node<'a> {
    name: NodeName<'a>,
    next: NodeNext<'a>,
}

#[derive(Debug)]
pub(crate) struct NodeNext<'a> {
    pub(crate) left: NodeName<'a>,
    pub(crate) right: NodeName<'a>,
}

#[derive(Debug)]
pub(crate) struct Nodes<'a>(pub(crate) HashMap<NodeName<'a>, NodeNext<'a>>);

#[derive(Debug)]
pub(crate) struct Map<'a> {
    pub(crate) moves: Moves,
    pub(crate) nodes: Nodes<'a>,
}

fn moove(i: &str) -> IResult<&str, MoveDirection> {
    map_res(anychar, MoveDirection::try_from)(i)
}

fn moves(i: &str) -> IResult<&str, Moves> {
    map(many1(moove), Moves)(i)
}

fn node_name(i: &str) -> IResult<&str, NodeName> {
    take(3u8)(i)
}

fn node_next(i: &str) -> IResult<&str, NodeNext> {
    map(
        delimited(
            char('('),
            separated_pair(node_name, tag(", "), node_name),
            char(')'),
        ),
        |(left, right)| NodeNext { left, right },
    )(i)
}

fn node(i: &str) -> IResult<&str, Node> {
    map(
        separated_pair(node_name, tag(" = "), node_next),
        |(name, next)| Node { name, next },
    )(i)
}

fn nodes(i: &str) -> IResult<&str, Nodes> {
    map(separated_list1(newline, node), |nodes| {
        let res = nodes
            .into_iter()
            .map(|Node { name, next }| (name, next))
            .collect();
        Nodes(res)
    })(i)
}

fn parse_map(i: &str) -> IResult<&str, Map> {
    map(
        separated_pair(moves, tag("\n\n"), nodes),
        |(moves, nodes)| Map { moves, nodes },
    )(i)
}

impl_from_str_for_obj_with_lifetimes_from_nom_parser!(parse_map, Map);
