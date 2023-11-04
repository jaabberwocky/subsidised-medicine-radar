use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Drug {
    ingredient: String,
    dosage: String,
    strength: String,
    subsidy_class: String,
    clinical_indication: String,
}

impl Drug {
    pub fn new(
        ingredient: String,
        dosage: String,
        strength: String,
        subsidy_class: String,
        clinical_indication: String,
    ) -> Self {
        Drug {
            ingredient,
            dosage,
            strength,
            subsidy_class,
            clinical_indication,
        }
    }
}
