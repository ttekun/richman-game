use crate::format;
use crate::state::GameState;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExitBreakdown {
    pub category: String,
    pub gross: f64,
    pub tax: f64,
    pub net: f64,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExitResult {
    pub breakdowns: Vec<ExitBreakdown>,
    pub total_gross: f64,
    pub total_tax: f64,
    pub total_net: f64,
    pub rank: Rank,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Rank {
    pub tier: String,
    pub label: String,
    pub description: String,
}

pub fn process_exit(state: &GameState) -> ExitResult {
    let mut breakdowns = Vec::new();
    let mut total_gross = 0.0;
    let mut total_tax = 0.0;
    let mut total_net = 0.0;

    // Cash (no tax)
    {
        let gross = state.cash.max(0.0);
        let tax = 0.0;
        let net = gross - tax;
        total_gross += gross;
        total_tax += tax;
        total_net += net;
        breakdowns.push(ExitBreakdown {
            category: "exit.category.cash".into(),
            gross,
            tax,
            net,
        });
    }

    // Real estate (15% on gain)
    for prop in &state.real_estate {
        let gross = prop.price - prop.loan;
        let gain = prop.price - prop.purchase_price;
        let tax = (gain.max(0.0) * 0.15).round();
        let net = gross - tax;
        total_gross += gross;
        total_tax += tax;
        total_net += net;
        breakdowns.push(ExitBreakdown {
            category: format!("exit.category.realestate:{}", prop.area),
            gross,
            tax,
            net,
        });
    }

    // Stocks (20.315% on gain)
    {
        let qqq_gross = state.stocks.qqq;
        let qqq_gain = state.stocks.qqq - state.stocks.qqq_cost;
        let qqq_tax = (qqq_gain.max(0.0) * 0.20315).round();
        
        let crypto_gross = state.stocks.crypto;
        let crypto_gain = state.stocks.crypto - state.stocks.crypto_cost;
        let crypto_tax = (crypto_gain.max(0.0) * 0.20315).round();
        
        let gross = qqq_gross + crypto_gross;
        let tax = qqq_tax + crypto_tax;
        let net = gross - tax;
        total_gross += gross;
        total_tax += tax;
        total_net += net;
        breakdowns.push(ExitBreakdown {
            category: "exit.category.stocks".into(),
            gross,
            tax,
            net,
        });
    }

    // Business (20% tax on value, 20% dividend tax on cash)
    if state.business.active {
        let biz_value = state.business.value * state.business.stake;
        let biz_tax = (biz_value * 0.20).round();
        let biz_cash = state.business.cash.max(0.0);
        let biz_cash_tax = (biz_cash * 0.20).round(); // dividend tax
        let gross = biz_value + biz_cash;
        let tax = biz_tax + biz_cash_tax;
        let net = gross - tax;
        total_gross += gross;
        total_tax += tax;
        total_net += net;
        breakdowns.push(ExitBreakdown {
            category: "exit.category.business".into(),
            gross,
            tax,
            net,
        });
    }

    // PropTech (20% tax)
    if state.proptech.active {
        let gross = state.proptech.value;
        let tax = (gross * 0.20).round();
        let net = gross - tax;
        total_gross += gross;
        total_tax += tax;
        total_net += net;
        breakdowns.push(ExitBreakdown {
            category: "exit.category.propTech".into(),
            gross,
            tax,
            net,
        });
    }

    let rank = get_rank(total_net);

    ExitResult {
        breakdowns,
        total_gross,
        total_tax,
        total_net,
        rank,
    }
}

pub fn get_rank(total: f64) -> Rank {
    if total >= 200_000.0 {
        Rank {
            tier: "gold".into(),
            label: "rank.gold.label".into(),
            description: "rank.gold.desc".into(),
        }
    } else if total >= 100_000.0 {
        Rank {
            tier: "silver".into(),
            label: "rank.silver.label".into(),
            description: "rank.silver.desc".into(),
        }
    } else if total >= 50_000.0 {
        Rank {
            tier: "bronze".into(),
            label: "rank.bronze.label".into(),
            description: "rank.bronze.desc".into(),
        }
    } else if total >= 10_000.0 {
        Rank {
            tier: "iron".into(),
            label: "rank.iron.label".into(),
            description: "rank.iron.desc".into(),
        }
    } else {
        Rank {
            tier: "participation".into(),
            label: "rank.participation.label".into(),
            description: "rank.participation.desc".into(),
        }
    }
}

pub fn generate_share_text(state: &GameState, final_total: f64) -> String {
    let rank = get_rank(final_total);
    format!(
        "share.template:{}:{}:{}:{}:{}",
        format::format_yen(final_total),
        rank.label,
        state.strategy,
        state.year,
        state.duration
    )
}