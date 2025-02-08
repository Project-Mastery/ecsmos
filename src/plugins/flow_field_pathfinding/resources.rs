use bevy::state::state::States;


#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShowGridState{
    HideGrid,
    ShowGrid
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PathFindingOverlayState{
    ShowNone,
    ShowTargets,
    ShowObstacles,
    ShowProimity,
    ShowVectorField
}