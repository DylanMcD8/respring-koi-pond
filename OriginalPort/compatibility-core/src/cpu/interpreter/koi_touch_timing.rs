//! Normalize repeated touch-move disturbances to the original 24 Hz budget.
//! Touch positions still update at full speed; taps and non-touch impulses keep
//! their original strengths. Sparse events never get amplified.
#[derive(Default)]
pub(super) struct TouchDisturbanceTiming {
    last: [Option<f64>; 4],
}
impl TouchDisturbanceTiming {
    pub(super) fn strength(&mut self, slot: usize, strength: f32, now: f64, unlocked: bool) -> f32 {
        if !unlocked || slot >= self.last.len() || !now.is_finite() {
            return strength;
        }
        // Original OnTouchDown queues 2.0; OnTouchMove queues 1.0. Slots
        // 0..4 are touches; 4..8 contain rain, fish and food disturbances.
        if strength == 2.0 {
            self.last[slot] = Some(now);
            return strength;
        }
        if strength != 1.0 { return strength; }
        let previous = self.last[slot].replace(now);
        let gain = previous.map(|then| ((now - then) * 24.0).clamp(0.0, 1.0)).unwrap_or(1.0);
        strength * gain as f32
    }
}
