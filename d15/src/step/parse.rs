use libaoc::impl_from_str_for_obj_with_lifetimes_from_nom_parser;
use nom::{
    branch::alt,
    character::complete::{alpha1, char, u32},
    combinator::map,
    sequence::{preceded, tuple},
    IResult,
};

use super::{Operation, Step};

fn step(i: &str) -> IResult<&str, Step> {
    map(tuple((alpha1, operation)), |(label, operation)| Step {
        label,
        operation,
    })(i)
}

fn operation(i: &str) -> IResult<&str, Operation> {
    alt((
        map(char('-'), |_| Operation::Remove),
        map(preceded(char('='), u32), Operation::Insert),
    ))(i)
}

impl_from_str_for_obj_with_lifetimes_from_nom_parser!(step, Step);
