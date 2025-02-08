pub trait CellStatus : Default + Send + Sync + Copy + PartialEq{
    fn get_non_default_value() -> Self;
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum BlockedStatus {
    #[default]
    Empty,
    Blocked
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

