use crate::constants;
use crate::rng::{Rng, random_range, roll_d20, roll_outcome};
use crate::state::GameState;

#[derive(Clone, Debug)]
pub struct EventResult {
    pub id: String,
    pub category: String,
    pub flavor: String,
    pub log: Vec<String>,
}

pub trait EventScenario: Send + Sync {
    fn id(&self) -> &str;
    fn category(&self) -> &str;
    fn weight(&self) -> u32;
    fn flavor_text(&self) -> &str;
    fn apply(&self, state: &mut GameState, rng: &dyn Rng) -> Vec<String>;
}

// re_01: real estate market change
pub struct ReMarketChange;
impl EventScenario for ReMarketChange {
    fn id(&self) -> &str { "re_01" }
    fn category(&self) -> &str { "re" }
    fn weight(&self) -> u32 { 20 }
    fn flavor_text(&self) -> &str { "event.re_01.title" }
    fn apply(&self, state: &mut GameState, rng: &dyn Rng) -> Vec<String> {
        let mut logs = Vec::new();
        for prop in &mut state.real_estate {
            let change = random_range(rng, -0.03, 0.12);
            prop.price = (prop.price * (1.0 + change)).round();
            let pct = change * 100.0;
            let label = if pct >= 0.0 { format!("+{:.0}%", pct) } else { format!("{:.0}%", pct) };
            logs.push(format!("event.re_01.log:{}:{}:{}", prop.area, crate::format::format_yen(prop.price), label));
        }
        logs
    }
}

// re_02: vacancy
pub struct ReVacancy;
impl EventScenario for ReVacancy {
    fn id(&self) -> &str { "re_02" }
    fn category(&self) -> &str { "re" }
    fn weight(&self) -> u32 { 15 }
    fn flavor_text(&self) -> &str { "event.re_02.title" }
    fn apply(&self, state: &mut GameState, rng: &dyn Rng) -> Vec<String> {
        if state.real_estate.is_empty() {
            return vec!["event.re_02.no_re".into()];
        }
        let idx = (rng.random() * state.real_estate.len() as f64).floor() as usize;
        let months = (rng.random() * 3.0).floor() as u32 + 1;
        let prop = &mut state.real_estate[idx];
        let loss = prop.rent * months as f64;
        prop.occupied_months = prop.occupied_months.saturating_sub(months);
        state.cash -= loss;
        vec![format!("event.re_02.log:{}:{}:{}", prop.area, months, crate::format::format_yen(loss))]
    }
}

// re_03: buy opportunity
pub struct ReBuyOpportunity;
impl EventScenario for ReBuyOpportunity {
    fn id(&self) -> &str { "re_03" }
    fn category(&self) -> &str { "re" }
    fn weight(&self) -> u32 { 10 }
    fn flavor_text(&self) -> &str { "event.re_03.title" }
    fn apply(&self, _state: &mut GameState, _rng: &dyn Rng) -> Vec<String> {
        vec!["event.re_03.text".into()]
    }
}

// sc_02: crypto surge/crash
pub struct ScCryptoVolatility;
impl EventScenario for ScCryptoVolatility {
    fn id(&self) -> &str { "sc_02" }
    fn category(&self) -> &str { "sc" }
    fn weight(&self) -> u32 { 20 }
    fn flavor_text(&self) -> &str { "event.sc_02.title" }
    fn apply(&self, state: &mut GameState, rng: &dyn Rng) -> Vec<String> {
        if state.stocks.crypto <= 0.0 {
            return vec!["event.sc_02.no_crypto".into()];
        }
        let change = random_range(rng, -0.20, 1.50);
        state.stocks.crypto = (state.stocks.crypto * (1.0 + change)).round();
        let pct = change * 100.0;
        let label = if pct >= 0.0 { format!("+{:.0}%", pct) } else { format!("{:.0}%", pct) };
        vec![format!("event.sc_02.log:{}:{}", crate::format::format_yen(state.stocks.crypto), label)]
    }
}

// sc_04: altcoin swing
pub struct ScAltcoinSwing;
impl EventScenario for ScAltcoinSwing {
    fn id(&self) -> &str { "sc_04" }
    fn category(&self) -> &str { "sc" }
    fn weight(&self) -> u32 { 12 }
    fn flavor_text(&self) -> &str { "event.sc_04.title" }
    fn apply(&self, state: &mut GameState, rng: &dyn Rng) -> Vec<String> {
        if state.stocks.crypto <= 0.0 {
            return vec!["event.sc_04.no_crypto".into()];
        }
        let change = random_range(rng, -0.50, 3.00);
        state.stocks.crypto = (state.stocks.crypto * (1.0 + change)).round();
        let pct = change * 100.0;
        let label = if pct >= 0.0 { format!("+{:.0}%", pct) } else { format!("{:.0}%", pct) };
        vec![format!("event.sc_04.log:{}:{}", crate::format::format_yen(state.stocks.crypto), label)]
    }
}

// bz_01: customer churn
pub struct BzCustomerChurn;
impl EventScenario for BzCustomerChurn {
    fn id(&self) -> &str { "bz_01" }
    fn category(&self) -> &str { "bz" }
    fn weight(&self) -> u32 { 15 }
    fn flavor_text(&self) -> &str { "event.bz_01.title" }
    fn apply(&self, state: &mut GameState, rng: &dyn Rng) -> Vec<String> {
        if !state.business.active || state.business.users <= 0.0 {
            return vec!["event.no_biz".into()];
        }
        let loss_rate = random_range(rng, 0.05, 0.20);
        let lost = (state.business.users * loss_rate).round();
        state.business.users -= lost;
        let new_mrr = state.business.mrr * (1.0 - loss_rate);
        state.business.mrr = new_mrr.round();
        vec![format!("event.bz_01.log:{:.0}:{:.0}", lost, loss_rate * 100.0)]
    }
}

// bz_02: viral effect
pub struct BzViral;
impl EventScenario for BzViral {
    fn id(&self) -> &str { "bz_02" }
    fn category(&self) -> &str { "bz" }
    fn weight(&self) -> u32 { 15 }
    fn flavor_text(&self) -> &str { "event.bz_02.title" }
    fn apply(&self, state: &mut GameState, rng: &dyn Rng) -> Vec<String> {
        if !state.business.active || state.business.users <= 0.0 {
            return vec!["event.no_biz".into()];
        }
        let growth_rate = random_range(rng, 0.30, 1.00);
        let gained = (state.business.users * growth_rate).round();
        state.business.users += gained;
        let new_mrr = state.business.mrr * (1.0 + growth_rate);
        state.business.mrr = new_mrr.round();
        vec![format!("event.bz_02.log:{:.0}:{:.0}", gained, growth_rate * 100.0)]
    }
}

// bz_03: competitor appears
pub struct BzCompetitor;
impl EventScenario for BzCompetitor {
    fn id(&self) -> &str { "bz_03" }
    fn category(&self) -> &str { "bz" }
    fn weight(&self) -> u32 { 10 }
    fn flavor_text(&self) -> &str { "event.bz_03.title" }
    fn apply(&self, state: &mut GameState, _rng: &dyn Rng) -> Vec<String> {
        if !state.business.active {
            return vec!["event.no_biz".into()];
        }
        state.business.competitor = true;
        state.business.growth_penalty = 0.7;
        vec!["event.bz_03.text".into()]
    }
}

// bz_06: enterprise deal
pub struct BzEnterpriseDeal;
impl EventScenario for BzEnterpriseDeal {
    fn id(&self) -> &str { "bz_06" }
    fn category(&self) -> &str { "bz" }
    fn weight(&self) -> u32 { 10 }
    fn flavor_text(&self) -> &str { "event.bz_06.title" }
    fn apply(&self, state: &mut GameState, rng: &dyn Rng) -> Vec<String> {
        if !state.business.active {
            return vec!["event.no_biz".into()];
        }
        let mrr_add = random_range(rng, 50.0, 200.0);
        let users_add = random_range(rng, 5.0, 20.0).round();
        state.business.mrr += mrr_add;
        state.business.users += users_add;
        vec![format!("event.bz_06.log:{}:{:.0}", crate::format::format_yen(mrr_add), users_add)]
    }
}

// bz_11: gov DX grant
pub struct BzGovGrant;
impl EventScenario for BzGovGrant {
    fn id(&self) -> &str { "bz_11" }
    fn category(&self) -> &str { "bz" }
    fn weight(&self) -> u32 { 8 }
    fn flavor_text(&self) -> &str { "event.bz_11.title" }
    fn apply(&self, state: &mut GameState, rng: &dyn Rng) -> Vec<String> {
        if !state.business.active {
            return vec!["event.no_biz".into()];
        }
        let grant = random_range(rng, 100.0, 500.0);
        state.business.cash += grant;
        vec![format!("event.bz_11.log:{}", crate::format::format_yen(grant))]
    }
}

// lf_01: health issue
pub struct LfHealthIssue;
impl EventScenario for LfHealthIssue {
    fn id(&self) -> &str { "lf_01" }
    fn category(&self) -> &str { "lf" }
    fn weight(&self) -> u32 { 5 }
    fn flavor_text(&self) -> &str { "event.lf_01.title" }
    fn apply(&self, state: &mut GameState, rng: &dyn Rng) -> Vec<String> {
        let cost = random_range(rng, 50.0, 200.0);
        state.cash -= cost;
        vec![format!("event.lf_01.log:{}", crate::format::format_yen(cost))]
    }
}

// lf_03: inheritance
pub struct LfInheritance;
impl EventScenario for LfInheritance {
    fn id(&self) -> &str { "lf_03" }
    fn category(&self) -> &str { "lf" }
    fn weight(&self) -> u32 { 4 }
    fn flavor_text(&self) -> &str { "event.lf_03.title" }
    fn apply(&self, state: &mut GameState, rng: &dyn Rng) -> Vec<String> {
        let amount = random_range(rng, 200.0, 1000.0);
        state.cash += amount;
        vec![format!("event.lf_03.log:{}", crate::format::format_yen(amount))]
    }
}

// lf_05: speaking gig
pub struct LfSpeakingGig;
impl EventScenario for LfSpeakingGig {
    fn id(&self) -> &str { "lf_05" }
    fn category(&self) -> &str { "lf" }
    fn weight(&self) -> u32 { 6 }
    fn flavor_text(&self) -> &str { "event.lf_05.title" }
    fn apply(&self, state: &mut GameState, rng: &dyn Rng) -> Vec<String> {
        let amount = random_range(rng, 50.0, 300.0);
        state.cash += amount;
        vec![format!("event.lf_05.log:{}", crate::format::format_yen(amount))]
    }
}

fn all_events() -> Vec<Box<dyn EventScenario>> {
    vec![
        Box::new(ReMarketChange),
        Box::new(ReVacancy),
        Box::new(ReBuyOpportunity),
        Box::new(ScCryptoVolatility),
        Box::new(ScAltcoinSwing),
        Box::new(BzCustomerChurn),
        Box::new(BzViral),
        Box::new(BzCompetitor),
        Box::new(BzEnterpriseDeal),
        Box::new(BzGovGrant),
        Box::new(LfHealthIssue),
        Box::new(LfInheritance),
        Box::new(LfSpeakingGig),
    ]
}

pub fn pick_random_event(state: &GameState, rng: &dyn Rng) -> Option<(EventResult, Box<dyn EventScenario>)> {
    let events = all_events();
    
    // Filter events based on game state (category-based filtering)
    let mut candidates: Vec<Box<dyn EventScenario>> = events.into_iter().filter(|e| {
        let cat = e.category();
        match cat {
            "re" => !state.real_estate.is_empty() || e.id() == "re_03",
            "sc" => state.stocks.crypto > 0.0 || state.stocks.qqq > 0.0,
            "bz" => state.business.active,
            "pt" => state.proptech.active,
            "lf" => true,
            _ => true,
        }
    }).collect();
    
    // Also always include life events regardless of state
    // (they're already included since "lf" => true)
    
    if candidates.is_empty() {
        // fallback: include all life events
        candidates = all_events().into_iter().filter(|e| e.category() == "lf").collect();
    }
    
    if candidates.is_empty() {
        return None;
    }
    
    let total_weight: u32 = candidates.iter().map(|e| e.weight()).sum();
    if total_weight == 0 {
        return None;
    }
    
    let mut roll = (rng.random() * total_weight as f64).floor() as u32;
    for e in candidates {
        let w = e.weight();
        if roll < w {
            let result = EventResult {
                id: e.id().to_string(),
                category: e.category().to_string(),
                flavor: e.flavor_text().to_string(),
                log: Vec::new(),
            };
            return Some((result, e));
        }
        roll -= w;
    }
    
    // Fallback: return last
    None
}

// Get business event for 25% chance in calc_business
pub fn pick_business_event(state: &GameState, rng: &dyn Rng) -> Option<(EventResult, Box<dyn EventScenario>)> {
    let events = all_events();
    let candidates: Vec<Box<dyn EventScenario>> = events.into_iter().filter(|e| {
        e.category() == "bz" && match e.id() {
            "bz_01" | "bz_02" | "bz_03" | "bz_06" | "bz_11" => state.business.active,
            _ => false,
        }
    }).collect();
    
    if candidates.is_empty() {
        return None;
    }
    
    let total_weight: u32 = candidates.iter().map(|e| e.weight()).sum();
    if total_weight == 0 {
        return None;
    }
    
    let mut roll = (rng.random() * total_weight as f64).floor() as u32;
    for e in candidates {
        let w = e.weight();
        if roll < w {
            let result = EventResult {
                id: e.id().to_string(),
                category: e.category().to_string(),
                flavor: e.flavor_text().to_string(),
                log: Vec::new(),
            };
            return Some((result, e));
        }
        roll -= w;
    }
    
    None
}