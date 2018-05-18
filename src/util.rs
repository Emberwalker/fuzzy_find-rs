//! Crate-wide utility functions.

pub(crate) fn round_score_decimal(val: f32) -> f32 {
    (val * 100_000f32).round() / 100_000f32
}
