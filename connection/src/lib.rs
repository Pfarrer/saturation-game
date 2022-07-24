use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::shapes::Line;
use model::{connection::Connection, construction::Construction, RemovalEvent};

pub struct ConnectionShapePlugin;

impl Plugin for ConnectionShapePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_connection_shape_system)
            .add_system(update_connection_shape_system)
            .add_system_to_stage(CoreStage::PostUpdate, remove_connection_shape_system);
    }
}

#[derive(Component)]
struct ConnectionShapeRef {
    connection_shape: Entity,
}

#[derive(Component, Debug)]
struct ConnectionShape {
    connection: Entity,
}

fn spawn_connection_shape_system(
    mut commands: Commands,
    query: Query<(Entity, &Connection), Added<Connection>>,
    constuction_query: Query<&Construction>,
) {
    for (connection_entity, connection) in query.iter() {
        let construction1 = constuction_query.get(connection.between.0).unwrap();
        let construction2 = constuction_query.get(connection.between.1).unwrap();
        let shape_entity = spawn_connection_shape(
            &mut commands,
            construction1.location,
            construction2.location,
            connection_entity,
        );

        debug!(
            "ConnectionShape {:?} spawned for {:?}",
            shape_entity, connection
        );
    }
}

fn update_connection_shape_system(
    mut commands: Commands,
    construction_changed_query: Query<Entity, Changed<Construction>>,
    connection_query: Query<(&Connection, &ConnectionShapeRef)>,
    constuction_query: Query<&Construction>,
) {
    for construction_entity in construction_changed_query.iter() {
        let connections = connection_query
            .iter()
            .filter(|(connection, _)| connection.connects_to(construction_entity));
        for (connection, shape_ref) in connections {
            let construction1 = constuction_query.get(connection.between.0).unwrap();
            let construction2 = constuction_query.get(connection.between.1).unwrap();
            update_connection_shape(
                &mut commands,
                construction1.location,
                construction2.location,
                shape_ref.connection_shape,
            );
        }
    }
}

fn remove_connection_shape_system(
    mut commands: Commands,
    mut connection_removal_events: EventReader<RemovalEvent<Connection>>,
    shape_query: Query<(Entity, &ConnectionShape)>,
) {
    for removal_event in connection_removal_events.iter() {
        let shape_entities = shape_query
            .iter()
            .filter(|(_, connection_shape)| connection_shape.connection == removal_event.entity);
        for (shape_entity, _) in shape_entities {
            // Despawn marker in connection
            commands
                .entity(removal_event.entity)
                .remove::<ConnectionShapeRef>();

            // Despawn shape itself
            commands.entity(shape_entity).despawn();

            debug!(
                "Despawning ConnectionShape of {:?}",
                removal_event.component
            );
        }
    }
}

fn spawn_connection_shape(
    commands: &mut Commands,
    location1: Vec2,
    location2: Vec2,
    connection_entity: Entity,
) -> Entity {
    let line = Line(location1, location2);
    let color = Color::Rgba {
        red: 255.,
        green: 255.,
        blue: 255.,
        alpha: 0.1,
    };

    let bundle = GeometryBuilder::build_as(
        &line,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::CYAN),
            outline_mode: StrokeMode::new(color, 1.),
        },
        Transform::default(),
    );

    let shape_entity = commands
        .spawn_bundle(bundle)
        .insert(ConnectionShape {
            connection: connection_entity,
        })
        .id();
    commands
        .entity(connection_entity)
        .insert(ConnectionShapeRef {
            connection_shape: shape_entity,
        });

    shape_entity
}

fn update_connection_shape(
    commands: &mut Commands,
    location1: Vec2,
    location2: Vec2,
    shape_entity: Entity,
) {
    let line = Line(location1, location2);
    let color = Color::Rgba {
        red: 255.,
        green: 255.,
        blue: 255.,
        alpha: 0.1,
    };

    let bundle = GeometryBuilder::build_as(
        &line,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::CYAN),
            outline_mode: StrokeMode::new(color, 1.),
        },
        Transform::default(),
    );

    commands.entity(shape_entity).insert_bundle(bundle);
}
