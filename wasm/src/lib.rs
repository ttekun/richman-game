mod constants;
mod decisions;
mod engine;
mod events;
mod format;
mod rng;
mod state;
mod tax;

use wasm_bindgen::prelude::*;

use rng::{JsRng, Rng, SeededRng};
use state::GameState;

#[wasm_bindgen]
pub struct GameEngine {
    state: GameState,
    rng: Box<dyn Rng>,
}

#[wasm_bindgen]
impl GameEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(capital: f64, duration: u32, strategy: &str) -> GameEngine {
        GameEngine {
            state: GameState::new(capital, duration, strategy),
            rng: Box::new(JsRng),
        }
    }

    pub fn total_assets(&self) -> f64 {
        let re_value: f64 = self.state.real_estate.iter().map(|p| p.price - p.loan).sum();
        let stocks_value = self.state.stocks.qqq + self.state.stocks.crypto;
        let business_value = if self.state.business.active {
            self.state.business.value * self.state.business.stake + self.state.business.cash.max(0.0)
        } else {
            0.0
        };
        let proptech_value = if self.state.proptech.active {
            self.state.proptech.value
        } else {
            0.0
        };
        self.state.cash + re_value + stocks_value + business_value + proptech_value
    }

    pub fn process_year(&mut self, decision_id: &str) -> String {
        let result = engine::process_year(&mut self.state, decision_id, self.rng.as_ref());
        serde_json::to_string(&result).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn get_decisions(&self) -> String {
        let decisions = decisions::get_decisions(&self.state, self.rng.as_ref());
        serde_json::to_string(&decisions).unwrap_or_else(|_| "[]".to_string())
    }

    pub fn process_exit(&self) -> String {
        let result = tax::process_exit(&self.state);
        serde_json::to_string(&result).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn get_state_json(&self) -> String {
        serde_json::to_string(&self.state).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn load_state_json(&mut self, json: &str) {
        if let Ok(state) = serde_json::from_str::<GameState>(json) {
            self.state = state;
        }
    }

    pub fn get_glossary(&self) -> String {
        let glossary = constants::glossary();
        serde_json::to_string(&glossary).unwrap_or_else(|_| "[]".to_string())
    }

    pub fn get_rank(&self, total: f64) -> String {
        let rank = tax::get_rank(total);
        serde_json::to_string(&rank).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn generate_share_text(&self, final_total: f64) -> String {
        tax::generate_share_text(&self.state, final_total)
    }

    pub fn set_seed(&mut self, seed: u64) {
        self.rng = Box::new(SeededRng::new(seed));
    }
}