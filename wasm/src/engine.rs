use crate::decisions;
use crate::events;
use crate::format;
use crate::rng::{Rng, random_range, roll_d20, roll_outcome};
use crate::state::GameState;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MacroResult {
    pub bonus: f64,
    pub market_bonus: f64,
    pub roll: u32,
    pub outcome_tier: String,
    pub outcome_label: String,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameEvent {
    pub id: String,
    pub category: String,
    pub flavor: String,
    pub logs: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct YearResult {
    pub year: u32,
    pub total_assets: f64,
    pub cash: f64,
    pub real_estate_value: f64,
    pub stocks_value: f64,
    pub business_value: f64,
    pub proptech_value: f64,
    pub macro_result: MacroResult,
    pub events: Vec<GameEvent>,
    pub log: Vec<String>,
    pub game_over: bool,
}

pub fn calc_macro(state: &GameState, rng: &dyn Rng) -> MacroResult {
    let roll = roll_d20(rng);
    let bonus = if roll >= 17 { 3.0 }
        else if roll >= 13 { 1.0 }
        else if roll >= 8 { 0.0 }
        else if roll >= 4 { -1.0 }
        else { -3.0 };
    
    let outcome = roll_outcome(rng, roll, 0.0);
    
    // Extra d20 for RE bubble
    let market_roll = roll_d20(rng);
    let market_bonus = if market_roll >= 18 { 2.0 } else { 0.0 };
    
    MacroResult {
        bonus,
        market_bonus,
        roll,
        outcome_tier: outcome.tier,
        outcome_label: outcome.label,
    }
}

pub fn calc_real_estate(state: &mut GameState, rng: &dyn Rng, macro_result: &MacroResult) {
    for prop in &mut state.real_estate {
        // Market change based on macro
        let market_change = random_range(rng, -0.05, 0.08) + macro_result.market_bonus * 0.05;
        prop.price = (prop.price * (1.0 + market_change)).round();
        
        // Occupied months: start at 12, decrease with events, recover over time
        if prop.occupied_months < 12 {
            prop.occupied_months = (prop.occupied_months + 2).min(12);
        }
        
        // Annual CF: rent * occupied_months/12 - loan_payment
        let annual_rent = prop.rent * prop.occupied_months as f64 / 12.0 * 12.0;
        let annual_loan_payment = prop.loan * 0.05;
        prop.cf = annual_rent - annual_loan_payment;
        state.cash += prop.cf;
    }
}

pub fn calc_stocks(state: &mut GameState, rng: &dyn Rng, macro_result: &MacroResult) {
    if state.stocks.qqq > 0.0 {
        let qqq_change = random_range(rng, -0.20, 0.35);
        state.stocks.qqq = (state.stocks.qqq * (1.0 + qqq_change)).round();
    }
    if state.stocks.crypto > 0.0 {
        let crypto_change = random_range(rng, -0.60, 2.00) + macro_result.market_bonus * 0.1;
        state.stocks.crypto = (state.stocks.crypto * (1.0 + crypto_change)).round();
    }
}

pub fn calc_business(state: &mut GameState, rng: &dyn Rng, macro_result: &MacroResult) -> Vec<GameEvent> {
    let mut game_events = Vec::new();
    
    if !state.business.active {
        return game_events;
    }
    
    // User growth
    let base_growth = random_range(rng, 0.05, 0.30);
    let growth_penalty = state.business.growth_penalty;
    let marketing_mult = if state.business.marketing_boost { 1.5 } else { 1.0 };
    let dev_mult = if state.business.dev_boost { 1.2 } else { 1.0 };
    
    let actual_growth = base_growth * growth_penalty * marketing_mult * dev_mult;
    let new_users = state.business.users * actual_growth;
    state.business.users += new_users;
    
    // MRR update: new users * avg_mrr + existing_mrr * retention
    let avg_mrr_per_user = if state.business.users > 0.0 {
        state.business.mrr / (state.business.users - new_users).max(1.0)
    } else {
        0.0
    };
    let new_mrr = state.business.mrr + new_users * avg_mrr_per_user;
    state.business.mrr = new_mrr.round();
    
    // Valuation: MRR * 12 * multiplier
    let multiplier = if state.business.pivot_done { 5.0 }
        else if state.business.competitor { 2.0 }
        else { 3.0 };
    state.business.value = (state.business.mrr * 12.0 * multiplier).round();
    
    // Business cash: MRR * 12 - expenses
    let expenses = state.business.employees as f64 * 200.0; // 200万 per employee per year
    state.business.cash += state.business.mrr * 12.0 - expenses;
    
    // 25% chance of business event
    if rng.random() < 0.25 {
        if let Some((result, event)) = events::pick_business_event(state, rng) {
            let logs = event.apply(state, rng);
            game_events.push(GameEvent {
                id: result.id,
                category: result.category,
                flavor: result.flavor,
                logs,
            });
        }
    }
    
    game_events
}

pub fn calc_proptech(state: &mut GameState, rng: &dyn Rng) {
    if !state.proptech.active {
        return;
    }
    
    // User growth
    let base_growth = random_range(rng, 0.05, 0.25);
    let marketing_mult = if state.proptech.marketing_boost { 1.5 } else { 1.0 };
    let actual_growth = base_growth * marketing_mult;
    let new_users = state.proptech.users * actual_growth;
    state.proptech.users += new_users;
    
    // MRR update
    let avg_mrr_per_user = if state.proptech.users > 0.0 {
        state.proptech.mrr / (state.proptech.users - new_users).max(1.0)
    } else {
        0.0
    };
    state.proptech.mrr = (state.proptech.mrr + new_users * avg_mrr_per_user).round();
    
    // Value = MRR * 12 * 3
    state.proptech.value = (state.proptech.mrr * 12.0 * 3.0).round();
    
    // Property boost affects RE
    if state.proptech.property_boost > 0.0 {
        for prop in &mut state.real_estate {
            let boost = state.proptech.property_boost;
            prop.rent = (prop.rent * (1.0 + boost)).round();
        }
    }
}

pub fn process_year(state: &mut GameState, decision_id: &str, rng: &dyn Rng) -> YearResult {
    state.year += 1;
    
    // Apply decision
    decisions::apply_decision(state, decision_id, rng);
    
    // Calculate macro
    let macro_result = calc_macro(state, rng);
    
    // Calculate all sectors
    calc_real_estate(state, rng, &macro_result);
    calc_stocks(state, rng, &macro_result);
    let biz_events = calc_business(state, rng, &macro_result);
    calc_proptech(state, rng);
    
    // 30% chance of life event
    let mut all_events = biz_events;
    if rng.random() < 0.3 {
        if let Some((result, event)) = events::pick_random_event(state, rng) {
            let logs = event.apply(state, rng);
            all_events.push(GameEvent {
                id: result.id,
                category: result.category,
                flavor: result.flavor,
                logs,
            });
        }
    }
    
    // Calculate total assets
    let re_value: f64 = state.real_estate.iter().map(|p| p.price - p.loan).sum();
    let stocks_value = state.stocks.qqq + state.stocks.crypto;
    let business_value = state.business.value * state.business.stake + state.business.cash.max(0.0);
    let proptech_value = state.proptech.value;
    let total = state.cash + re_value + stocks_value + business_value + proptech_value;
    
    // Record history
    state.history.push(crate::state::YearHistory {
        year: state.year,
        total_assets: total,
    });
    
    // Check early end
    if total >= state.goal {
        state.game_over = true;
        state.log.push(format!("log.goal_achieved:{}", format::format_yen(state.goal)));
    }
    
    // Check duration end
    if state.year >= state.duration {
        state.game_over = true;
    }
    
    YearResult {
        year: state.year,
        total_assets: total,
        cash: state.cash,
        real_estate_value: re_value,
        stocks_value,
        business_value,
        proptech_value,
        macro_result,
        events: all_events,
        log: state.log.clone(),
        game_over: state.game_over,
    }
}