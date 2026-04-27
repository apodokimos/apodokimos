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

    /// Returns the field's reference dependency depth D_ref for D̃ normalization (wp-v0.2 §3.3)
    ///
    /// D_ref represents the "typical" maximum depth for well-formed claims in this field.
    /// Claims with D ≤ D_ref have D̃ ≥ 1.0 (full or boosted weight); deeper claims are
    /// logarithmically penalized.
    ///
    /// # Examples
    /// - Clinical medicine: 3 (typical evidence chain: study → review → guideline)
    /// - Physics: 5 (deeper theoretical derivations)
    /// - Basic science: 2 (direct experimental claims)
    fn reference_depth(&self) -> u32;

    /// Compute time-decay factor R(c, t) per wp-v0.2 §3.2
    ///
    /// Formula: R(c, t) = 2^(−Δt / t_½(c))
    ///
    /// Where:
    /// - Δt = days elapsed since claim registration
    /// - t_½(c) = field-calibrated half-life in days
    ///
    /// At Δt = 0: R = 1.0 (no decay)
    /// At Δt = t_½: R = 0.5 (one half-life)
    /// At Δt = 2·t_½: R = 0.25 (two half-lives)
    ///
    /// # Arguments
    /// * `days_elapsed` - Days since claim registration (Δt)
    ///
    /// # Returns
    /// Decay factor in range [0.0, 1.0]
    fn compute_decay(&self, days_elapsed: u32) -> f64 {
        // 2^(-Δt/t_½) = 0.5^(Δt/t_½)
        2.0f64.powf(-(days_elapsed as f64) / self.decay_half_life() as f64)
    }

    /// Returns the oracle bonus coefficient γ (gamma) per wp-v0.2 §3.5
    ///
    /// The oracle factor enters the weight formula as a bonus:
    ///   W = R × D̃ × S × (1 + γ·O) × δ
    ///
    /// Where O ∈ [0, 1] is the base oracle credibility from `OFactorSource::factor_value()`.
    ///
    /// Typical values:
    /// - 0.5 (moderate bonus): clinical medicine, applied sciences
    /// - 1.0 (high bonus): physics, mathematics (peer review very reliable)
    /// - 0.2 (low bonus): emerging fields, preprint-heavy domains
    fn oracle_gamma(&self) -> f64;
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

    fn reference_depth(&self) -> u32 {
        // Typical evidence chain depth: primary study → systematic review → clinical guideline
        3
    }

    fn oracle_gamma(&self) -> f64 {
        // Moderate bonus for clinical medicine (peer review is important but not infallible)
        0.5
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
    fn clinical_medicine_reference_depth() {
        let field = ClinicalMedicine::new();
        // D_ref = 3: study → review → guideline
        assert_eq!(field.reference_depth(), 3);
    }

    #[test]
    fn clinical_medicine_oracle_gamma() {
        let field = ClinicalMedicine::new();
        // γ = 0.5: moderate bonus for clinical medicine
        // Oracle bonus = (1 + γ·O) ranges from 1.0 to 1.5
        assert!((field.oracle_gamma() - 0.5).abs() < f64::EPSILON);
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
        // One half-life: R = 2^(-1) = 0.5
        let r = field.compute_decay(1825);
        assert!((r - 0.5).abs() < 0.001, "R at one half-life should be 0.5, got {}", r);
    }

    #[test]
    fn compute_decay_at_two_half_lives() {
        let field = ClinicalMedicine::new();
        // Two half-lives: R = 2^(-2) = 0.25
        let r = field.compute_decay(3650);
        assert!((r - 0.25).abs() < 0.001, "R at two half-lives should be 0.25, got {}", r);
    }

    #[test]
    fn compute_decay_formula_matches_wp_v0_2() {
        let field = ClinicalMedicine::new();
        // wp-v0.2 §3.2: R(c, t) = 2^(-Δt / t_½)
        // Verify at several points
        let test_cases = [
            (0, 1.0),      // t=0: 2^0 = 1.0
            (912, 0.707),  // t=0.5*t_½: 2^(-0.5) ≈ 0.707
            (1825, 0.5),   // t=t_½: 2^(-1) = 0.5
            (3650, 0.25),  // t=2*t_½: 2^(-2) = 0.25
        ];
        for (days, expected) in test_cases {
            let r = field.compute_decay(days);
            assert!(
                (r - expected).abs() < 0.01,
                "R({}) should be ~{}, got {}",
                days,
                expected,
                r
            );
        }
    }

    #[test]
    fn clinical_medicine_is_copy() {
        let field = ClinicalMedicine::new();
        let field2 = field; // move
        let _field3 = field; // should work because Copy
        assert_eq!(field2.decay_half_life(), 1825);
    }
}
