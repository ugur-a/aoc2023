#[derive(Debug, Clone, Copy)]
pub(crate) struct Number {
    pub(crate) value: u32,
    pub(crate) start_pos: (usize, usize),
    pub(crate) len: u32,
}

pub(crate) fn parse_numbers(s: &str) -> Vec<Number> {
    let width = s.lines().next().unwrap().len();

    s.lines()
        .enumerate()
        .flat_map(|(y, line)| line.char_indices().map(move |(x, c)| ((x, y), c)))
        .fold((vec![], None), |(mut acc, mut curr_number), ((x, y), c)| {
            match (&mut curr_number, c.to_digit(10)) {
                (None, None) => {
                    // no current number and no new one encountered
                    // => no need to do anything
                }
                (None, Some(n)) => {
                    // a new number encountered!
                    curr_number = Some(Number {
                        value: n,
                        start_pos: (x, y),
                        len: 1,
                    });
                }
                (Some(num), Some(n)) => {
                    // continue building up the number
                    num.value = 10 * num.value + n;
                    num.len += 1;

                    // if this is the last character of the line,
                    // finish up the number immediately
                    if x == width - 1 {
                        acc.push(*num);
                        curr_number = None;
                    }
                }
                (Some(num), None) => {
                    // a non-digit encountered - finish up the current number!
                    acc.push(*num);
                    curr_number = None;
                }
            }
            (acc, curr_number)
        })
        .0
}
