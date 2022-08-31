mod connections;
mod constructions;

fn main() {
//   connections::disallow_crossing_connections();
    constructions::prevent_building_if_influence_areas_overlap();
}
