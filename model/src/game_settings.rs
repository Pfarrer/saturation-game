use crate::construction::ConstructionKind;

pub struct GameSettings;

impl GameSettings {
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
}
