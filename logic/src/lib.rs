use bevy::{prelude::*};

pub fn window_to_world(
    wnds: &Res<Windows>,
    camera_query: &Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) -> Option<Vec2> {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_query.single();
    let wnd = wnds.get_primary().unwrap();

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();
        Some(world_pos)
    } else {
        None
    }
}

pub mod construction {
    use model::construction::Construction;

    pub fn collides_with(construction: &Construction, other_construction: &Construction) -> bool {
        other_construction
            .location
            .distance(construction.location.clone())
            + 0.1 // account for floating point math errors
            < (other_construction.influence_radius + construction.influence_radius)
    }

//     use crate::{collides_with, find_closest_construction};
//
//     pub fn move_to<'a>(
//         target_location: &Vec2,
//         construction: &mut Construction,
//         other_constructions: Vec<&'a Construction>,
//     ) {
//         construction.location = target_location.clone();
//
//         let colliding_constructions: Vec<_> = other_constructions
//             .iter()
//             .copied()
//             .filter(|other_construction| collides_with(construction, other_construction))
//             .collect();
//         let colliding_construction_opt =
//             find_closest_construction(construction, &colliding_constructions);
//         if let Some(colliding_construction) = colliding_construction_opt {
//             // Blocking constructions do exist on the target_location -> find the next best location
//             // that is not blocked by other constructions
//             let normalized_relative =
//                 (*target_location - colliding_construction.location).normalize();
//             let target_length =
//                 colliding_construction.influence_radius + construction.influence_radius;
//             construction.location =
//                 colliding_construction.location + target_length * normalized_relative;
//
//             let initial_colliding_construction = colliding_construction;
//
//             for _ in 0..9 {
//                 let colliding_constructions: Vec<_> = other_constructions
//                     .iter()
//                     .copied()
//                     .filter(|other_construction| collides_with(construction, other_construction))
//                     .collect();
//                 let colliding_construction_opt =
//                     find_closest_construction(construction, &colliding_constructions);
//                 if let Some(colliding_construction) = colliding_construction_opt {
//                     // Blocking constructions do exist on the target_location -> find the next best location
//                     // that is not blocked by other constructions
//                     let distance_to_colliding_construction_center = crate::distance_line_to_point(
//                         (target_location, &initial_colliding_construction.location),
//                         &colliding_construction.location,
//                     );
//                     let ratio = 1.
//                         - distance_to_colliding_construction_center
//                             / (colliding_construction.influence_radius
//                                 + construction.influence_radius);
//                     let factor = ratio.asin();
//                     construction.location +=
//                         normalized_relative * colliding_construction.influence_radius * 2. * factor;
//                 } else {
//                     // No more colliding constructions -> we are done here
//                     break;
//                 }
//             }
//         } else {
//             // No colliding constructions -> target_location not blocked by anything
//         }
//     }
}

// fn find_closest_construction<'a, 'b>(
//     construction: &'a Construction,
//     other_constructions: &Vec<&'b Construction>,
// ) -> Option<&'b Construction> {
//     let mut colliding_constructions = other_constructions.iter().copied().collect::<Vec<_>>();
//     colliding_constructions.sort_by(|a, b| {
//         let distance_a = a.location.distance(construction.location.clone());
//         let distance_b = b.location.distance(construction.location.clone());
//         distance_a.total_cmp(&distance_b)
//     });
//     colliding_constructions.first().copied()
// }

// fn distance_line_to_point(line: (&Vec2, &Vec2), point: &Vec2) -> f32 {
//     let [x0, y0] = point.to_array();
//     let [x1, y1] = line.0.to_array();
//     let [x2, y2] = line.1.to_array();
//
//     let numerator = ((x2 - x1) * (y1 - y0) - (x1 - x0) * (y2 - y1)).abs();
//     let denominator = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
//
//     numerator / denominator
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn find_closest_construction_test() {
//         let construction1 = Construction {
//             location: Vec2::new(0., 0.),
//             influence_radius: 2.,
//             ..Default::default()
//         };
//         let construction2 = Construction {
//             location: Vec2::new(5., 0.),
//             influence_radius: 3.,
//             ..Default::default()
//         };
//         let construction3 = Construction {
//             location: Vec2::new(0., 3.),
//             influence_radius: 1.,
//             ..Default::default()
//         };
//
//         let closest = find_closest_construction(&construction1, &vec![&construction2]);
//         assert_eq!(construction2.location, closest.unwrap().location);
//
//         let closest =
//             find_closest_construction(&construction1, &vec![&construction2, &construction3]);
//         assert_eq!(construction3.location, closest.unwrap().location);
//
//         let closest =
//             find_closest_construction(&construction1, &vec![&construction3, &construction2]);
//         assert_eq!(construction3.location, closest.unwrap().location);
//     }
//
//     #[test]
//     fn distance_line_to_point_test() {
//         let line_x_axis = (&Vec2::ZERO, &Vec2::new(1.9, 0.));
//         assert_eq!(0., distance_line_to_point(line_x_axis, &Vec2::ZERO));
//         assert_eq!(
//             0.,
//             distance_line_to_point(line_x_axis, &Vec2::new(100., 0.))
//         );
//         assert_eq!(1., distance_line_to_point(line_x_axis, &Vec2::new(0., 1.)));
//         assert_eq!(1., distance_line_to_point(line_x_axis, &Vec2::new(0., -1.)));
//         assert_eq!(
//             3.45,
//             distance_line_to_point(line_x_axis, &Vec2::new(0., -3.45))
//         );
//         assert_eq!(
//             12.3456,
//             distance_line_to_point(line_x_axis, &Vec2::new(3.91, 12.3456))
//         );
//
//         let line_y_axis = (&Vec2::new(0., -13.), &Vec2::ZERO);
//         assert_eq!(0., distance_line_to_point(line_y_axis, &Vec2::ZERO));
//         assert_eq!(
//             3.1,
//             distance_line_to_point(line_y_axis, &Vec2::new(3.1, 0.))
//         );
//     }
// }
