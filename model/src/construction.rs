use bevy::prelude::*;

#[derive(Debug, Clone)]
pub enum ConstructionKind {
    Base,
    Collector,
}

#[derive(Component, Debug, Clone)]
pub struct Construction {
    pub location: Vec2,
    pub kind: ConstructionKind,
    pub influence_radius: f32,
}
