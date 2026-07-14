use std::sync::atomic::{AtomicU64, Ordering};
use std::cell::Cell;

pub trait Rng: Send + Sync {
    fn random(&self) -> f64;
}

pub struct JsRng;

impl Rng for JsRng {
    fn random(&self) -> f64 {
        js_sys::Math::random()
    }
}

pub struct SeededRng {
    state: AtomicU64,
}

impl SeededRng {
    pub fn new(seed: u64) -> Self {
        let s = if seed == 0 { 0xDEADBEEFCAFEBABE } else { seed };
        SeededRng { state: AtomicU64::new(s) }
    }
}

impl Rng for SeededRng {
    fn random(&self) -> f64 {
        let mut x = self.state.load(Ordering::Relaxed);
        // xorshift64
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state.store(x, Ordering::Relaxed);
        // Convert to f64 in [0, 1)
        (x >> 11) as f64 / (1u64 << 53) as f64
    }
}

#[derive(Clone, Debug)]
pub struct RollOutcome {
    pub tier: String,
    pub label: String,
    pub multiplier: f64,
    pub effective_roll: u32,
}

pub fn roll_d20(rng: &dyn Rng) -> u32 {
    (rng.random() * 20.0).floor() as u32 + 1
}

pub fn random_range(rng: &dyn Rng, min: f64, max: f64) -> f64 {
    rng.random() * (max - min) + min
}

pub fn roll_outcome(rng: &dyn Rng, roll: u32, bonus: f64) -> RollOutcome {
    let effective = ((roll as f64 + bonus).max(1.0).min(20.0)) as u32;
    let (tier, label, multiplier) = if effective >= 17 {
        ("great", "macro.great", random_range(rng, 2.0, 2.5))
    } else if effective >= 13 {
        ("good", "macro.good", random_range(rng, 1.3, 1.5))
    } else if effective >= 8 {
        ("moderate", "macro.moderate", random_range(rng, 0.9, 1.1))
    } else if effective >= 4 {
        ("poor", "macro.poor", random_range(rng, 0.5, 0.7))
    } else {
        ("critical", "macro.critical", random_range(rng, 0.1, 0.3))
    };
    RollOutcome {
        tier: tier.to_string(),
        label: label.to_string(),
        multiplier,
        effective_roll: effective,
    }
}