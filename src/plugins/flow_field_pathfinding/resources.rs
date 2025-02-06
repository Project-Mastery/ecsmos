use core::fmt;


pub trait CellStatus : Default + Send + Sync + Copy + PartialEq{
    fn get_non_default_value() -> Self;
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum BlockedStatus {
    #[default]
    Empty,
    Blocked
}

impl BlockedStatus {
    pub fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl CellStatus for BlockedStatus {
    fn get_non_default_value() -> Self {
        Self::Blocked
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum TargetStatus {
    #[default]
    NotTarget,
    IsTarget,
}

impl TargetStatus {
    pub fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl CellStatus for TargetStatus {
    fn get_non_default_value() -> Self {
        Self::IsTarget
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TargetProximity {
    Unreachable,
    NotComputed,
    Computed(f32)
}

impl From<f32> for TargetProximity {
    fn from(value: f32) -> Self {
        TargetProximity::Computed(value)
    }
}
