use std::ops::Range;

#[derive(Debug)]
pub struct LexError<'a> {
    pub(crate) source: &'a str,
    pub(crate) span: Range<usize>,
}
