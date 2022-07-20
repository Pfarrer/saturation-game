use bevy::prelude::*;

#[derive(Debug)]
pub enum ConstructionKind {
    Base,
    Collector,
}

#[derive(Component, Debug)]
pub struct Construction {
    pub location: Vec2,
    pub kind: ConstructionKind,
    pub influence_radius: f32,
}
