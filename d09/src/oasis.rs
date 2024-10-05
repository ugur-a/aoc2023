use libaoc::impl_from_str_from_nom_parser;
use nom::{
    character::complete::{char, i32, newline},
    combinator::map,
    multi::separated_list1,
    IResult,
};

pub(crate) struct ValueHistory(Vec<i32>);

impl ValueHistory {
    fn derivatives(self) -> Vec<Vec<i32>> {
        let mut curr_derivative = self.0;
        let mut derivatives = vec![];
        while !curr_derivative.iter().all(|n| n == &0) {
            // TODO: use `array_windows`
            let next_derivative = curr_derivative
                .windows(2)
                .map(|pair| pair[1] - pair[0])
                .collect();
            derivatives.push(std::mem::replace(&mut curr_derivative, next_derivative));
        }
        derivatives
    }

    pub(crate) fn extrapolate(self) -> i32 {
        self.derivatives()
            .into_iter()
            .map(|v| *v.last().expect("enough data points to extrapolate"))
            .sum()
    }

    pub(crate) fn extrapolate_back(self) -> i32 {
        self.derivatives()
            .into_iter()
            .map(|v| *v.first().expect("enough data points to extrapolate"))
            .rfold(0, |acc, n| n - acc)
    }
}

pub(crate) struct OasisReport(pub(crate) Vec<ValueHistory>);

fn value_history(i: &str) -> IResult<&str, ValueHistory> {
    map(separated_list1(char(' '), i32), ValueHistory)(i)
}

fn oasis_report(i: &str) -> IResult<&str, OasisReport> {
    map(separated_list1(newline, value_history), OasisReport)(i)
}

impl_from_str_from_nom_parser!(oasis_report, OasisReport);
