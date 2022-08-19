use crate::construction::ConstructionKind;

pub struct GameConfiguration;

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
            ConstructionKind::Base => 10.,
            ConstructionKind::Collector => 5.,
            ConstructionKind::Extractor => 0.,
        }
    }
    pub fn energy_input(&self, kind: &ConstructionKind) -> f64 {
        match kind {
            ConstructionKind::Base => 0.,
            ConstructionKind::Collector => 0.,
            ConstructionKind::Extractor => 4.,
        }
    }

    pub fn influence_radius(&self, kind: &ConstructionKind) -> f32 {
        match kind {
            ConstructionKind::Base => 70.,
            ConstructionKind::Collector => 30.,
            ConstructionKind::Extractor => 20.,
        }
    }

    
}
