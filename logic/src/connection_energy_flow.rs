use bevy::prelude::*;
use model::connection::Connection;
use model::construction::Construction;
use model::game_configuration::GameConfiguration;
use std::collections::HashMap;

pub fn assign_energy_flows(
    game_configuration: &GameConfiguration,
    constructions: &Vec<(Entity, &Construction)>,
    connections: &Vec<&Connection>,
) -> HashMap<(Entity, Entity), f64> {
    let to_fn =
        |construction: &Construction| game_configuration.energy_input(&construction.kind) > 0.;

    let (total_energy_availability, total_energy_demand) =
        constructions
            .iter()
            .copied()
            .fold((0., 0.), |acc, (_, construction)| {
                (
                    acc.0 + game_configuration.energy_output(&construction.kind),
                    acc.1 + game_configuration.energy_input(&construction.kind),
                )
            });
    let energy_factor = {
        let factor = total_energy_demand / total_energy_availability;
        if factor > 1. {
            1.
        } else {
            factor
        }
    };

    let entity_construction_map: HashMap<Entity, &Construction> =
        constructions.iter().copied().collect();
    let mut connection_flows: HashMap<(Entity, Entity), f64> = HashMap::new();

    let energy_producing_constructions = constructions
        .iter()
        .copied()
        .filter(|(_, c)| game_configuration.energy_output(&c.kind) > 0.);
    for (producing_entity, producing_construction) in energy_producing_constructions {
        let paths = crate::utils::routing::shortest_paths_from(
            constructions,
            connections,
            producing_entity,
            &to_fn,
        );

        for path in paths {
            let consuming_entity = path[0];
            let consuming_construction = entity_construction_map.get(&consuming_entity).unwrap();

            // Calculate the energy flowing from `producing_construction` to `consuming_construction`
            let energy_production = game_configuration.energy_output(&producing_construction.kind);
            let energy_demand = game_configuration.energy_input(&consuming_construction.kind);
            let energy_flow: f64 =
                energy_factor * energy_production * energy_demand / total_energy_demand;

            for entities in path.windows(2) {
                *connection_flows
                    .entry((entities[1], entities[0]))
                    .or_insert(0.) += energy_flow;
            }
        }
    }

    normalize_energy_flows(connection_flows)
}

fn normalize_energy_flows(
    connection_flows: HashMap<(Entity, Entity), f64>,
) -> HashMap<(Entity, Entity), f64> {
    connection_flows
        .iter()
        .fold(
            HashMap::new(),
            |mut normalized_map, (entities, energy_flow)| {
                let connection = Connection::new_between(entities.0, entities.1);
                let entity_order_flipped = entities.0 != connection.between().0;
                let energy_flow = if entity_order_flipped {
                    energy_flow * -1.
                } else {
                    *energy_flow
                };

                *normalized_map.entry(*connection.between()).or_insert(0.) += energy_flow;
                normalized_map
            },
        )
        .iter()
        .filter_map(|(entities, energy_flow)| {
            let rounded_energy_flow = (energy_flow * 1000.).round() / 1000.;
            if rounded_energy_flow == 0. {
                None
            } else {
                Some((*entities, rounded_energy_flow))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::default;
    use bevy::prelude::Entity;
    use model::construction::ConstructionKind;

    const GAME_CONFIGURATION: GameConfiguration = GameConfiguration {
        energy_output_base: 0.,
        energy_output_collector: 1.,
        energy_output_extractor: 0.,

        energy_input_base: 0.,
        energy_input_collector: 0.,
        energy_input_extractor: 1.,
    };

    /// World: C
    #[test]
    fn test_assign_energy_flows_c() {
        let construction1 = Construction {
            kind: ConstructionKind::Collector,
            ..default()
        };
        let constructions = vec![(entity(1), &construction1)];
        let connections = vec![];
        let connection_flows =
            assign_energy_flows(&GAME_CONFIGURATION, &constructions, &connections);
        assert_eq!(0, connection_flows.len());
    }

    /// World: C -- E and C -- C -- E
    #[test]
    fn test_assign_energy_flows_c_e() {
        let collector1 = Construction {
            kind: ConstructionKind::Collector,
            ..default()
        };
        let extractor = Construction {
            kind: ConstructionKind::Extractor,
            ..default()
        };
        let constructions = vec![(entity(1), &collector1), (entity(2), &extractor)];

        let connection1 = Connection::new_between(entity(1), entity(2));
        let connections = vec![&connection1];

        let connection_flows =
            assign_energy_flows(&GAME_CONFIGURATION, &constructions, &connections);
        assert_eq!(1, connection_flows.len());
        assert_eq!(Some(&1.), connection_flows.get(&(entity(1), entity(2))));

        // Now simluate world: C -- C -- E
        let collector2 = Construction {
            kind: ConstructionKind::Collector,
            ..default()
        };
        let constructions = vec![
            (entity(1), &collector1),
            (entity(2), &collector2),
            (entity(3), &extractor),
        ];
        let connection2 = Connection::new_between(entity(2), entity(3));
        let connections = vec![&connection1, &connection2];

        let connection_flows =
            assign_energy_flows(&GAME_CONFIGURATION, &constructions, &connections);
        assert_eq!(2, connection_flows.len());
        assert_eq!(
            Some(&0.5),
            connection_flows.get(&(entity(1), entity(2))),
            "{:?}",
            connection_flows
        );
        assert_eq!(Some(&1.), connection_flows.get(&(entity(2), entity(3))));
    }

    /// World: C -- E -- E
    #[test]
    fn test_assign_energy_flows_c_e_e() {
        let extractor1 = Construction {
            kind: ConstructionKind::Extractor,
            ..default()
        };
        let extractor2 = Construction {
            kind: ConstructionKind::Extractor,
            ..default()
        };
        let collector1 = Construction {
            kind: ConstructionKind::Collector,
            ..default()
        };
        let constructions = vec![
            (entity(1), &collector1),
            (entity(2), &extractor1),
            (entity(3), &extractor2),
        ];

        let connection1 = Connection::new_between(entity(1), entity(2));
        let connection2 = Connection::new_between(entity(2), entity(3));
        let connections = vec![&connection1, &connection2];

        let connection_flows =
            assign_energy_flows(&GAME_CONFIGURATION, &constructions, &connections);
        assert_eq!(2, connection_flows.len());
        assert_eq!(Some(&1.), connection_flows.get(&(entity(1), entity(2))));
        assert_eq!(Some(&0.5), connection_flows.get(&(entity(2), entity(3))));
    }

    /// World: E -- C -- C -- E
    #[test]
    fn test_assign_energy_flows_e_c_c_e() {
        let extractor1 = Construction {
            kind: ConstructionKind::Extractor,
            ..default()
        };
        let extractor2 = Construction {
            kind: ConstructionKind::Extractor,
            ..default()
        };
        let collector1 = Construction {
            kind: ConstructionKind::Collector,
            ..default()
        };
        let collector2 = Construction {
            kind: ConstructionKind::Collector,
            ..default()
        };
        let constructions = vec![
            (entity(1), &extractor1),
            (entity(2), &collector1),
            (entity(3), &collector2),
            (entity(4), &extractor2),
        ];

        let connection1 = Connection::new_between(entity(1), entity(2));
        let connection2 = Connection::new_between(entity(2), entity(3));
        let connection3 = Connection::new_between(entity(3), entity(4));
        let connections = vec![&connection1, &connection2, &connection3];

        let connection_flows =
            assign_energy_flows(&GAME_CONFIGURATION, &constructions, &connections);
        assert_eq!(2, connection_flows.len(), "{:?}", connection_flows);
        assert_eq!(Some(&-1.), connection_flows.get(&(entity(1), entity(2))));
        assert_eq!(Some(&1.), connection_flows.get(&(entity(3), entity(4))));
    }

    fn entity(idx: u64) -> Entity {
        Entity::from_bits(idx)
    }
}
