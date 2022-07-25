use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ConstructionKind {
    Base,
    Collector,
    Extractor,
}

#[derive(Component, Debug, Clone)]
pub struct Construction {
    pub location: Vec2,
    pub kind: ConstructionKind,
    pub influence_radius: f32,
}
