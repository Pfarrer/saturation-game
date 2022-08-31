use std::collections::HashMap;
use std::ops::Index;

use bevy::prelude::*;
use model::connection::Connection;
use model::construction::Construction;
use petgraph::algo::bellman_ford;
use petgraph::prelude::*;

pub fn shortest_paths_from(
    constructions: &Vec<(Entity, &Construction)>,
    connections: &Vec<&Connection>,
    from_entity: Entity,
    to_fn: &dyn Fn(&Construction) -> bool,
) -> Vec<Vec<Entity>> {
    let mut graph = Graph::<Entity, f32, Undirected>::new_undirected();

    let entity_to_node_map: HashMap<Entity, NodeIndex> = constructions
        .iter()
        .map(|(entity, _)| (*entity, graph.add_node(*entity)))
        .collect();
    connections.iter().copied().for_each(|connection| {
        let node0_opt = entity_to_node_map.get(&connection.between().0);
        let node1_opt = entity_to_node_map.get(&connection.between().1);

        if let (Some(node0), Some(node1)) = (node0_opt, node1_opt) {
            graph.add_edge(*node0, *node1, 1.);
        } else {
            warn!("Connection {:?} links between two constructions, but at least one was not found: {:?}. Will ignore connection (and missing construction) in energy flow assigment.", connection, (node0_opt, node1_opt));
        }
    });

    let predecessors = bellman_ford(&graph, entity_to_node_map[&from_entity])
        .unwrap()
        .predecessors;

    let mut paths = Vec::new();

    for (target_index, (target_entity, _)) in constructions
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, (_, construction))| to_fn(construction))
    {
        if let Some(predecessor_index) = predecessors[target_index] {
            // Construction is reachable by the current producing node
            let predecessor_entity = graph.index(predecessor_index);
            let mut path = vec![target_entity, *predecessor_entity];

            let mut previous_index = predecessor_index;
            while let Some(predecessor_index) = predecessors[previous_index.index()] {
                let predecessor_entity = graph.index(predecessor_index);
                path.push(*predecessor_entity);

                previous_index = predecessor_index;
            }

            paths.push(path);
        }
    }

    paths
}

#[cfg(test)]
mod tests {
    use crate::default;

    use model::{construction::ConstructionKind, game_configuration::GameConfiguration};

    use super::*;

    const GAME_CONFIGURATION: GameConfiguration = GameConfiguration {
        energy_output_base: 0.,
        energy_output_collector: 1.,
        energy_output_extractor: 0.,

        energy_input_base: 0.,
        energy_input_collector: 0.,
        energy_input_extractor: 1.,
    };

    /// C -- E
    #[test]
    fn test_shortest_paths_from_c_e() {
        let collector1 = Construction {
            kind: ConstructionKind::Collector,
            ..default()
        };
        let extractor1 = Construction {
            kind: ConstructionKind::Extractor,
            ..default()
        };
        let constructions = vec![(entity(1), &collector1), (entity(2), &extractor1)];

        let connection1 = Connection::new_between(entity(1), entity(2));
        let connections = vec![&connection1];

        let paths = shortest_paths_from(
            &constructions,
            &connections,
            entity(1),
            &|construction: &Construction| GAME_CONFIGURATION.energy_input(&construction.kind) > 0.,
        );

        assert_eq!(1, paths.len());
        assert!(paths.contains(&vec![entity(2), entity(1)]));
    }

    ///         / E
    /// C -- E +
    ///         \ E
    #[test]
    fn test_shortest_paths_from_c_e_e2() {
        let collector1 = Construction {
            kind: ConstructionKind::Collector,
            ..default()
        };
        let extractor1 = Construction {
            kind: ConstructionKind::Extractor,
            ..default()
        };
        let extractor21 = Construction {
            kind: ConstructionKind::Extractor,
            ..default()
        };
        let extractor22 = Construction {
            kind: ConstructionKind::Extractor,
            ..default()
        };
        let constructions = vec![
            (entity(1), &collector1),
            (entity(2), &extractor1),
            (entity(3), &extractor21),
            (entity(4), &extractor22),
        ];

        let connection1 = Connection::new_between(entity(1), entity(2));
        let connection21 = Connection::new_between(entity(2), entity(3));
        let connection22 = Connection::new_between(entity(2), entity(4));
        let connections = vec![&connection1, &connection21, &connection22];

        let paths = shortest_paths_from(
            &constructions,
            &connections,
            entity(1),
            &|construction: &Construction| GAME_CONFIGURATION.energy_input(&construction.kind) > 0.,
        );
        assert_eq!(3, paths.len(), "{:?}", paths);
        assert!(paths.contains(&vec![entity(2), entity(1)]));
        assert!(
            paths.contains(&vec![entity(3), entity(2), entity(1)]),
            "{:#?}",
            paths
        );
        assert!(
            paths.contains(&vec![entity(4), entity(2), entity(1)]),
            "{:#?}",
            paths
        );
    }

    /// E -- B -- C -- E -- C
    #[test]
    fn test_shortest_paths_from_e_b_c_e_c() {
        let collector1 = Construction {
            kind: ConstructionKind::Collector,
            ..default()
        };
        let collector2 = Construction {
            kind: ConstructionKind::Collector,
            ..default()
        };
        let extractor1 = Construction {
            kind: ConstructionKind::Extractor,
            ..default()
        };
        let extractor2 = Construction {
            kind: ConstructionKind::Extractor,
            ..default()
        };
        let base = Construction {
            kind: ConstructionKind::Base,
            ..default()
        };
        let constructions = vec![
            (entity(1), &extractor1),
            (entity(2), &base),
            (entity(3), &collector1),
            (entity(4), &extractor2),
            (entity(5), &collector2),
        ];

        let connection1 = Connection::new_between(entity(1), entity(2));
        let connection2 = Connection::new_between(entity(2), entity(3));
        let connection3 = Connection::new_between(entity(3), entity(4));
        let connection4 = Connection::new_between(entity(4), entity(5));

        let connections = vec![&connection1, &connection2, &connection3, &connection4];

        {
            let paths_from_3 = shortest_paths_from(
                &constructions,
                &connections,
                entity(3),
                &|construction: &Construction| {
                    GAME_CONFIGURATION.energy_input(&construction.kind) > 0.
                },
            );

            assert_eq!(2, paths_from_3.len());
            assert!(paths_from_3.contains(&vec![entity(1), entity(2), entity(3)]));
            assert!(
                paths_from_3.contains(&vec![entity(4), entity(3)]),
                "{:#?}",
                paths_from_3
            );
        }

        {
            let paths_from_5 = shortest_paths_from(
                &constructions,
                &connections,
                entity(5),
                &|construction: &Construction| {
                    GAME_CONFIGURATION.energy_input(&construction.kind) > 0.
                },
            );

            assert_eq!(2, paths_from_5.len());
            assert!(paths_from_5.contains(&vec![
                entity(1),
                entity(2),
                entity(3),
                entity(4),
                entity(5)
            ]));
            assert!(paths_from_5.contains(&vec![entity(4), entity(5)]));
        }
    }

    fn entity(idx: u64) -> Entity {
        Entity::from_bits(idx)
    }
}
