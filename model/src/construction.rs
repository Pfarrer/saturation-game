use std::default::Default;

use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ConstructionKind {
    Base,
    Collector,
    Extractor,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstructionStatus {
    Hovering,
    Operating,
}

#[derive(Component, Debug, Clone)]
pub struct Construction {
    pub location: Vec2,
    pub kind: ConstructionKind,
    pub status: ConstructionStatus,
    pub influence_radius: f32,
}

#[derive(Component)]
pub struct UnderConstructionMarker;

impl Default for Construction {
    fn default() -> Self {
        Self {
            location: Default::default(),
            kind: ConstructionKind::Base,
            status: ConstructionStatus::Operating,
            influence_radius: Default::default(),
        }
    }
}
