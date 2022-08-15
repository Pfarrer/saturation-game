use bevy::{prelude::*, utils::HashSet};

#[derive(Debug)]
pub struct CollisionEvent {
    pub status: CollisionStatus,
    pub entities: (Entity, Entity),
}

#[derive(Debug, PartialEq)]
pub enum CollisionStatus {
    Started,
    Ended,
}

#[derive(Component, Debug)]
pub struct CollisionAware;

#[derive(Component, Debug)]
pub struct Collisions(pub HashSet<Entity>);
