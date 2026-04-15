//! Field schema trait for domain-specific calibration (P-04, P-16)
//!
//! Each scientific field (e.g., clinical medicine, physics) has distinct
//! epistemic characteristics that require calibration of the weight function:
//! - Time-decay half-life (how quickly claims become obsolete)
//! - Score normalization (baseline USI calibration per field)

/// Field-calibrated schema for weight function parameters (P-04, P-16)
///
/// Different scientific fields have distinct temporal dynamics:
/// - Clinical medicine: 5-year half-life (fast-moving evidence)
/// - Physics: 20-year half-life (stable foundations)
/// - Climate science: 10-year half-life (evolving models)
pub trait FieldSchema: Send + Sync {
    /// Returns the field identifier (e.g., "clinical-medicine")
    fn field_id(&self) -> &'static str;

    /// Normalizes a raw score using field-specific USI calibration (P-16)
    ///
    /// Clinical medicine is the reference field (USI = 1.0). Other fields
    /// have different coefficients based on their epistemic volatility.
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

    /// Compute time-decay factor R(t) for a given elapsed time
    ///
    /// # Arguments
    /// * `days_elapsed` - Days since claim registration
    ///
    /// # Returns
    /// Decay factor in range [0.0, 1.0] where 1.0 = no decay, 0.5 = one half-life
    fn compute_decay(&self, days_elapsed: u32) -> f64 {
        0.5f64.powf(days_elapsed as f64 / self.decay_half_life() as f64)
    }
}

/// Clinical medicine field schema with 5-year half-life
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClinicalMedicine;

impl FieldSchema for ClinicalMedicine {
    fn field_id(&self) -> &'static str {
        "clinical-medicine"
    }

    fn normalize_score(&self, raw_score: f64) -> f64 {
        // Clinical medicine is the reference field (USI = 1.0)
        // Other fields multiply by different coefficients
        raw_score
    }

    fn decay_half_life(&self) -> u32 {
        // 5 years = 1825 days
        1825
    }
}

impl ClinicalMedicine {
    /// Create a new clinical medicine field schema
    #[must_use]
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
        // Reference field: identity transform
        assert_eq!(field.normalize_score(100.0), 100.0);
        assert_eq!(field.normalize_score(0.5), 0.5);
        // Negative values pass through (domain-specific handling)
        assert_eq!(field.normalize_score(-50.0), -50.0);
    }

    #[test]
    fn field_schema_trait_object() {
        fn get_half_life(schema: &(dyn FieldSchema + Send + Sync)) -> u32 {
            schema.decay_half_life()
        }
        let field = ClinicalMedicine::new();
        assert_eq!(get_half_life(&field), 1825);
    }

    #[test]
    fn compute_decay_at_zero() {
        let field = ClinicalMedicine::new();
        // No time elapsed = no decay
        assert!((field.compute_decay(0) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn compute_decay_at_half_life() {
        let field = ClinicalMedicine::new();
        // One half-life elapsed = 0.5 decay factor
        assert!((field.compute_decay(1825) - 0.5).abs() < 0.001);
    }

    #[test]
    fn compute_decay_at_two_half_lives() {
        let field = ClinicalMedicine::new();
        // Two half-lives = 0.25 decay factor
        assert!((field.compute_decay(3650) - 0.25).abs() < 0.001);
    }

    #[test]
    fn clinical_medicine_is_copy() {
        let field = ClinicalMedicine::new();
        let field2 = field; // move
        let _field3 = field; // should work because Copy
        assert_eq!(field2.decay_half_life(), 1825);
    }
}
