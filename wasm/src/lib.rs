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

    /// Apply LLM-generated effects to game state.
    /// JSON format: {"cashDelta":0,"stocksDelta":0,"cryptoDelta":0,"businessValueDelta":0,"businessCashDelta":0,"proptechDelta":0,"rePriceDelta":0,"log":"","gameOver":false}
    pub fn apply_effects(&mut self, effects_json: &str) -> String {
        if let Ok(effects) = serde_json::from_str::<serde_json::Value>(effects_json) {
            if let Some(cash) = effects["cashDelta"].as_f64() {
                self.state.cash += cash;
            }
            if let Some(stocks) = effects["stocksDelta"].as_f64() {
                self.state.stocks.qqq += stocks;
            }
            if let Some(crypto) = effects["cryptoDelta"].as_f64() {
                self.state.stocks.crypto += crypto;
            }
            if let Some(bv) = effects["businessValueDelta"].as_f64() {
                self.state.business.value += bv;
            }
            if let Some(bc) = effects["businessCashDelta"].as_f64() {
                self.state.business.cash += bc;
            }
            if let Some(pt) = effects["proptechDelta"].as_f64() {
                self.state.proptech.value += pt;
            }
            if let Some(re_pct) = effects["rePriceDelta"].as_f64() {
                for prop in &mut self.state.real_estate {
                    prop.price = (prop.price * (1.0 + re_pct)).round();
                }
            }
            if let Some(log) = effects["log"].as_str() {
                if !log.is_empty() {
                    self.state.log.push(log.to_string());
                }
            }
            if let Some(game_over) = effects["gameOver"].as_bool() {
                if game_over {
                    self.state.game_over = true;
                }
            }
        }

        // Return updated state summary as JSON
        let total = self.total_assets();
        serde_json::json!({
            "totalAssets": total,
            "cash": self.state.cash,
            "gameOver": self.state.game_over,
        }).to_string()
    }

    /// Get a compact game context string for LLM prompt
    pub fn get_game_context(&self) -> String {
        let state = &self.state;
        serde_json::json!({
            "year": state.year,
            "duration": state.duration,
            "strategy": state.strategy,
            "cash": state.cash,
            "totalAssets": self.total_assets(),
            "goal": state.goal,
            "realEstate": state.real_estate.iter().map(|p| {
                serde_json::json!({"area": p.area, "price": p.price, "loan": p.loan, "rent": p.rent, "cf": p.cf})
            }).collect::<Vec<_>>(),
            "stocks": {"qqq": state.stocks.qqq, "crypto": state.stocks.crypto},
            "business": {"active": state.business.active, "mrr": state.business.mrr, "users": state.business.users, "value": state.business.value, "cash": state.business.cash, "stake": state.business.stake, "competitor": state.business.competitor},
            "proptech": {"active": state.proptech.active, "mrr": state.proptech.mrr, "value": state.proptech.value},
            "history": state.history,
        }).to_string()
    }
}