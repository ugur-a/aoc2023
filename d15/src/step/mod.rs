mod parse;

pub(crate) enum Operation {
    Remove,
    Insert(u32),
}

pub(crate) struct Step<'a> {
    pub(crate) label: &'a str,
    pub(crate) operation: Operation,
}
