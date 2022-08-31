use crate::construction::ConstructionKind;

pub struct GameConfiguration {
    pub energy_output_base: f64,
    pub energy_output_collector: f64,
    pub energy_output_extractor: f64,

    pub energy_input_base: f64,
    pub energy_input_collector: f64,
    pub energy_input_extractor: f64,
}

impl GameConfiguration {
    pub fn material_build_demand(&self, kind: &ConstructionKind) -> f64 {
        match kind {
            ConstructionKind::Base => f64::MAX,
            ConstructionKind::Collector => 30.,
            ConstructionKind::Extractor => 20.,
        }
    }

    pub fn material_output(&self, kind: &ConstructionKind) -> f64 {
        match kind {
            ConstructionKind::Base => 0.,
            ConstructionKind::Collector => 0.,
            ConstructionKind::Extractor => 0.2,
        }
    }
    pub fn energy_output(&self, kind: &ConstructionKind) -> f64 {
        match kind {
            ConstructionKind::Base => self.energy_output_base,
            ConstructionKind::Collector => self.energy_output_collector,
            ConstructionKind::Extractor => self.energy_output_extractor,
        }
    }
    pub fn energy_input(&self, kind: &ConstructionKind) -> f64 {
        match kind {
            ConstructionKind::Base => self.energy_input_base,
            ConstructionKind::Collector => self.energy_input_collector,
            ConstructionKind::Extractor => self.energy_input_extractor,
        }
    }

    pub fn influence_radius(&self, kind: &ConstructionKind) -> f32 {
        match kind {
            ConstructionKind::Base => 70.,
            ConstructionKind::Collector => 30.,
            ConstructionKind::Extractor => 20.,
        }
    }

    pub fn max_connection_distance(&self) -> f32 {
        150.
    }
}
