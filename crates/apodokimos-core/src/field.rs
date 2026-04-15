//! Field schema trait for domain-specific calibration (P-04, P-16)
//!
//! Each scientific field (e.g., clinical medicine, physics) has distinct
//! epistemic characteristics that require calibration of the weight function:
//! - Time-decay half-life (how quickly claims become obsolete)
//! - Score normalization (baseline USI calibration per field)

use alloc::string::String;

/// Field-calibrated schema for weight function parameters (P-04, P-16)
///
/// Different scientific fields have distinct temporal dynamics:
/// - Clinical medicine: 5-year half-life (fast-moving evidence)
/// - Physics: 20-year half-life (stable foundations)
/// - Climate science: 10-year half-life (evolving models)
pub trait FieldSchema {
    /// Returns the field identifier (e.g., "clinical-medicine")
    fn field_id(&self) -> String;

    /// Normalizes a raw score using field-specific USI calibration (P-16)
    ///
    /// # Arguments
    /// * `raw_score` - The unnormalized score from W(claim) computation
    ///
    /// # Returns
    /// Normalized score adjusted for field baseline
    fn normalize_score(&self, raw_score: f64) -> f64;

    /// Returns the field-calibrated time-decay half-life in days (P-04)
    ///
    /// This determines the R(t) decay rate for the field:
    /// R(t) = 0.5^(t / half_life_days)
    ///
    /// # Examples
    /// - Clinical medicine: 1825 days (~5 years)
    /// - Physics: 7300 days (~20 years)
    fn decay_half_life(&self) -> u32;
}

/// Clinical medicine field schema with 5-year half-life
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClinicalMedicine;

impl FieldSchema for ClinicalMedicine {
    fn field_id(&self) -> String {
        String::from("clinical-medicine")
    }

    fn normalize_score(&self, raw_score: f64) -> f64 {
        // Baseline USI calibration: clinical medicine uses 1.0 as reference
        raw_score * 1.0
    }

    fn decay_half_life(&self) -> u32 {
        // 5 years = 1825 days
        1825
    }
}

impl ClinicalMedicine {
    /// Create a new clinical medicine field schema
    pub const fn new() -> Self {
        Self
    }
}

impl Default for ClinicalMedicine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clinical_medicine_field_id() {
        let field = ClinicalMedicine::new();
        assert_eq!(field.field_id(), "clinical-medicine");
    }

    #[test]
    fn clinical_medicine_half_life() {
        let field = ClinicalMedicine::new();
        assert_eq!(field.decay_half_life(), 1825);
    }

    #[test]
    fn clinical_medicine_normalize() {
        let field = ClinicalMedicine::new();
        // Baseline calibration: 1.0 multiplier
        assert_eq!(field.normalize_score(100.0), 100.0);
        assert_eq!(field.normalize_score(0.5), 0.5);
    }

    #[test]
    fn field_schema_trait_object() {
        fn get_half_life(schema: &dyn FieldSchema) -> u32 {
            schema.decay_half_life()
        }
        let field = ClinicalMedicine::new();
        assert_eq!(get_half_life(&field), 1825);
    }
}
