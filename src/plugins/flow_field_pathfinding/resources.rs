use core::fmt;


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellContents {
    Empty,
    Blocked
}

impl CellContents {
    pub fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
