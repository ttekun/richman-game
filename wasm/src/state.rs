use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub area: String,
    pub price: f64,
    pub purchase_price: f64,
    pub loan: f64,
    pub rent: f64,
    pub cf: f64,
    pub occupied_months: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stocks {
    pub qqq: f64,
    pub crypto: f64,
    pub qqq_cost: f64,
    pub crypto_cost: f64,
}

impl Default for Stocks {
    fn default() -> Self {
        Stocks { qqq: 0.0, crypto: 0.0, qqq_cost: 0.0, crypto_cost: 0.0 }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Business {
    pub mrr: f64,
    pub users: f64,
    pub value: f64,
    pub cash: f64,
    pub stake: f64,
    pub employees: u32,
    pub active: bool,
    pub competitor: bool,
    pub growth_penalty: f64,
    pub marketing_boost: bool,
    pub dev_boost: bool,
    pub pivot_done: bool,
}

impl Default for Business {
    fn default() -> Self {
        Business {
            mrr: 0.0,
            users: 0.0,
            value: 0.0,
            cash: 0.0,
            stake: 1.0,
            employees: 0,
            active: false,
            competitor: false,
            growth_penalty: 1.0,
            marketing_boost: false,
            dev_boost: false,
            pivot_done: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PropTech {
    pub mrr: f64,
    pub users: f64,
    pub value: f64,
    pub active: bool,
    pub property_boost: f64,
    pub marketing_boost: bool,
}

impl Default for PropTech {
    fn default() -> Self {
        PropTech {
            mrr: 0.0,
            users: 0.0,
            value: 0.0,
            active: false,
            property_boost: 0.0,
            marketing_boost: false,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YearHistory {
    pub year: u32,
    pub total_assets: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    pub start_capital: f64,
    pub goal: f64,
    pub duration: u32,
    pub strategy: String,
    pub year: u32,
    pub cash: f64,
    pub real_estate: Vec<Property>,
    pub stocks: Stocks,
    pub business: Business,
    pub proptech: PropTech,
    pub emergency: f64,
    pub history: Vec<YearHistory>,
    pub log: Vec<String>,
    pub game_over: bool,
    pub series_a: bool,
    pub series_b: bool,
    pub expanded_to: Option<String>,
    pub exited: bool,
    pub pending_events: Vec<String>,
    pub save_version: u32,
}

impl GameState {
    pub fn new(capital: f64, duration: u32, strategy: &str) -> Self {
        GameState {
            start_capital: capital,
            goal: 200_000.0,
            duration,
            strategy: strategy.to_string(),
            year: 0,
            cash: capital,
            real_estate: Vec::new(),
            stocks: Stocks::default(),
            business: Business::default(),
            proptech: PropTech::default(),
            emergency: 0.0,
            history: Vec::new(),
            log: Vec::new(),
            game_over: false,
            series_a: false,
            series_b: false,
            expanded_to: None,
            exited: false,
            pending_events: Vec::new(),
            save_version: 1,
        }
    }
}