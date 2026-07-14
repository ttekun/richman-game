use crate::constants::TOKYO_AREAS;
use crate::rng::{Rng, random_range, roll_d20, roll_outcome};
use crate::state::{GameState, Property};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Decision {
    pub id: String,
    pub label: String,
    pub description: String,
    pub category: String,
    pub priority: u32,
}

pub fn generate_property(rng: &dyn Rng) -> Property {
    let area_idx = (rng.random() * TOKYO_AREAS.len() as f64).floor() as usize;
    let area = &TOKYO_AREAS[area_idx];
    let price = random_range(rng, area.base_price[0], area.base_price[1]).round();
    let loan_rate = random_range(rng, 0.6, 0.8);
    let loan = (price * loan_rate).round();
    let rent_rate = random_range(rng, area.rent_rate[0], area.rent_rate[1]);
    let rent = (price * rent_rate / 12.0).round();
    let annual_loan_payment = loan * 0.05; // 5% annual payment
    let cf = rent * 12.0 - annual_loan_payment;
    Property {
        area: area.name.to_string(),
        price,
        purchase_price: price,
        loan,
        rent,
        cf,
        occupied_months: 12,
    }
}

pub fn apply_decision(state: &mut GameState, decision_id: &str, rng: &dyn Rng) {
    match decision_id {
        "start_business" => {
            state.business.active = true;
            state.business.mrr = 20.0;
            state.business.users = 10.0;
            state.business.value = 0.0;
            state.business.cash = 0.0;
            state.business.employees = 0;
            state.log.push("log.start_business".into());
        }
        "start_proptech" => {
            state.proptech.active = true;
            state.proptech.mrr = 30.0;
            state.proptech.users = 5.0;
            state.proptech.value = 0.0;
            state.log.push("log.start_proptech".into());
        }
        "buy_stocks" => {
            if state.cash >= 100.0 {
                let amount = state.cash * 0.5;
                let split = rng.random();
                let qqq_amount = amount * split;
                let crypto_amount = amount * (1.0 - split);
                state.stocks.qqq += qqq_amount;
                state.stocks.crypto += crypto_amount;
                state.stocks.qqq_cost += qqq_amount;
                state.stocks.crypto_cost += crypto_amount;
                state.cash -= amount;
                state.log.push(format!("log.buy_stocks:{}:{}", crate::format::format_yen(qqq_amount), crate::format::format_yen(crypto_amount)));
            }
        }
        "save_cash" => {
            // Just keep cash, no action needed
            state.log.push("log.save_cash".into());
        }
        "buy_re_1" => {
            if state.cash >= 500.0 {
                let prop = generate_property(rng);
                if state.cash >= prop.price * 0.3 {
                    state.cash -= prop.price * 0.3; // down payment
                    state.real_estate.push(prop);
                    state.log.push("log.buy_re_1".into());
                }
            }
        }
        "buy_re_3" => {
            if state.cash >= 1500.0 {
                for _ in 0..3 {
                    let prop = generate_property(rng);
                    if state.cash >= prop.price * 0.3 {
                        state.cash -= prop.price * 0.3;
                        state.real_estate.push(prop);
                    }
                }
                state.log.push("log.buy_re_3".into());
            }
        }
        "refinance" => {
            for prop in &mut state.real_estate {
                if prop.loan > 0.0 {
                    prop.loan = (prop.loan * 0.85).round();
                    let annual_loan_payment = prop.loan * 0.05;
                    prop.cf = prop.rent * 12.0 - annual_loan_payment;
                }
            }
            state.log.push("log.refinance".into());
        }
        "hire_marketer" => {
            state.business.employees += 1;
            state.business.marketing_boost = true;
            state.cash -= 100.0;
            state.log.push("log.hire_marketer".into());
        }
        "hire_engineer" => {
            state.business.employees += 1;
            state.business.dev_boost = true;
            state.cash -= 100.0;
            state.log.push("log.hire_engineer".into());
        }
        "pivot" => {
            state.business.pivot_done = true;
            state.business.growth_penalty = 1.0;
            state.business.competitor = false;
            state.cash -= 200.0;
            state.log.push("log.pivot".into());
        }
        "series_a" => {
            state.series_a = true;
            state.business.cash += 2000.0;
            state.business.stake = 0.7;
            state.log.push("log.series_a".into());
        }
        "bootstrap" => {
            state.series_a = true; // mark as decided (won't show again)
            state.log.push("log.bootstrap".into());
        }
        "expand_korea" => {
            state.expanded_to = Some("korea".into());
            state.business.cash -= 500.0;
            state.log.push("log.expand_korea".into());
        }
        "expand_taiwan" => {
            state.expanded_to = Some("taiwan".into());
            state.business.cash -= 500.0;
            state.log.push("log.expand_taiwan".into());
        }
        "hold_expansion" => {
            state.log.push("log.hold_expansion".into());
        }
        "proptech_marketing" => {
            state.proptech.marketing_boost = true;
            state.cash -= 100.0;
            state.log.push("log.proptech_marketing".into());
        }
        "proptech_apply_all" => {
            state.proptech.property_boost = 0.05;
            for prop in &mut state.real_estate {
                prop.rent = (prop.rent * 1.05).round();
                prop.cf = prop.rent * 12.0 - prop.loan * 0.05;
            }
            state.log.push("log.proptech_apply_all".into());
        }
        "exit_sell_all" => {
            state.exited = true;
            state.game_over = true;
            state.log.push("log.exit_sell_all".into());
        }
        "exit_sell_biz" => {
            state.exited = true;
            state.game_over = true;
            state.log.push("log.exit_sell_biz".into());
        }
        "hold_to_end" => {
            state.log.push("log.hold_to_end".into());
        }
        _ => {}
    }
}

pub fn get_decisions(state: &GameState, rng: &dyn Rng) -> Vec<Decision> {
    let mut decisions = Vec::new();

    // Year 1: start business
    if state.year == 1 && !state.business.active {
        decisions.push(Decision {
            id: "start_business".into(),
            label: "decision.start_business".into(),
            description: "decision.start_business.desc".into(),
            category: "business".into(),
            priority: 100,
        });
    }

    // Start proptech (if business active and has RE)
    if state.business.active && !state.proptech.active && !state.real_estate.is_empty() {
        decisions.push(Decision {
            id: "start_proptech".into(),
            label: "decision.start_proptech".into(),
            description: "decision.start_proptech.desc".into(),
            category: "proptech".into(),
            priority: 90,
        });
    }

    // Cash allocation
    if state.cash >= 100.0 {
        decisions.push(Decision {
            id: "buy_stocks".into(),
            label: "decision.buy_stocks".into(),
            description: "decision.buy_stocks.desc".into(),
            category: "stocks".into(),
            priority: 50,
        });
    }
    if state.cash >= 200.0 {
        decisions.push(Decision {
            id: "save_cash".into(),
            label: "decision.save_cash".into(),
            description: "decision.save_cash.desc".into(),
            category: "cash".into(),
            priority: 30,
        });
    }

    // Business decisions
    if state.business.active {
        if state.cash >= 100.0 {
            decisions.push(Decision {
                id: "hire_marketer".into(),
                label: "decision.hire_marketer".into(),
                description: "decision.hire_marketer.desc".into(),
                category: "business".into(),
                priority: 60,
            });
            decisions.push(Decision {
                id: "hire_engineer".into(),
                label: "decision.hire_engineer".into(),
                description: "decision.hire_engineer.desc".into(),
                category: "business".into(),
                priority: 60,
            });
        }
        if state.business.competitor && !state.business.pivot_done && state.cash >= 200.0 {
            decisions.push(Decision {
                id: "pivot".into(),
                label: "decision.pivot".into(),
                description: "decision.pivot.desc".into(),
                category: "business".into(),
                priority: 70,
            });
        }
    }

    // Real estate decisions
    if state.cash >= 500.0 {
        decisions.push(Decision {
            id: "buy_re_1".into(),
            label: "decision.buy_re_1".into(),
            description: "decision.buy_re_1.desc".into(),
            category: "realestate".into(),
            priority: 55,
        });
    }
    if state.cash >= 1500.0 {
        decisions.push(Decision {
            id: "buy_re_3".into(),
            label: "decision.buy_re_3".into(),
            description: "decision.buy_re_3.desc".into(),
            category: "realestate".into(),
            priority: 65,
        });
    }
    if !state.real_estate.is_empty() {
        decisions.push(Decision {
            id: "refinance".into(),
            label: "decision.refinance".into(),
            description: "decision.refinance.desc".into(),
            category: "realestate".into(),
            priority: 40,
        });
    }

    // VC options
    if state.business.active && !state.series_a {
        decisions.push(Decision {
            id: "series_a".into(),
            label: "decision.series_a".into(),
            description: "decision.series_a.desc".into(),
            category: "vc".into(),
            priority: 75,
        });
        decisions.push(Decision {
            id: "bootstrap".into(),
            label: "decision.bootstrap".into(),
            description: "decision.bootstrap.desc".into(),
            category: "vc".into(),
            priority: 70,
        });
    }

    // Expansion options (Series B)
    if state.series_a && !state.expanded_to.is_some() && state.business.active {
        decisions.push(Decision {
            id: "expand_korea".into(),
            label: "decision.expand_korea".into(),
            description: "decision.expand_korea.desc".into(),
            category: "expansion".into(),
            priority: 65,
        });
        decisions.push(Decision {
            id: "expand_taiwan".into(),
            label: "decision.expand_taiwan".into(),
            description: "decision.expand_taiwan.desc".into(),
            category: "expansion".into(),
            priority: 65,
        });
        decisions.push(Decision {
            id: "hold_expansion".into(),
            label: "decision.hold_expansion".into(),
            description: "decision.hold_expansion.desc".into(),
            category: "expansion".into(),
            priority: 35,
        });
    }

    // PropTech decisions
    if state.proptech.active {
        if state.cash >= 100.0 {
            decisions.push(Decision {
                id: "proptech_marketing".into(),
                label: "decision.proptech_marketing".into(),
                description: "decision.proptech_marketing.desc".into(),
                category: "proptech".into(),
                priority: 55,
            });
        }
        if !state.real_estate.is_empty() {
            decisions.push(Decision {
                id: "proptech_apply_all".into(),
                label: "decision.proptech_apply_all".into(),
                description: "decision.proptech_apply_all.desc".into(),
                category: "proptech".into(),
                priority: 60,
            });
        }
    }

    // Exit options (later years)
    if state.year >= 5 {
        decisions.push(Decision {
            id: "exit_sell_all".into(),
            label: "decision.exit_sell_all".into(),
            description: "decision.exit_sell_all.desc".into(),
            category: "exit".into(),
            priority: 80,
        });
        if state.business.active {
            decisions.push(Decision {
                id: "exit_sell_biz".into(),
                label: "decision.exit_sell_biz".into(),
                description: "decision.exit_sell_biz.desc".into(),
                category: "exit".into(),
                priority: 75,
            });
        }
        decisions.push(Decision {
            id: "hold_to_end".into(),
            label: "decision.hold_to_end".into(),
            description: "decision.hold_to_end.desc".into(),
            category: "exit".into(),
            priority: 20,
        });
    }

    // Strategy priority boost (years 1-3)
    let strat = state.strategy.as_str();
    if state.year <= 3 && !strat.is_empty() {
        let boost_map: &[(&str, &str, u32)] = &[
            ("realestate", "realestate", 15),
            ("stocks", "stocks", 15),
            ("business", "business", 15),
            ("proptech", "proptech", 15),
        ];
        for (s, cat, boost) in boost_map {
            if strat == *s {
                for d in &mut decisions {
                    if d.category == *cat {
                        d.priority += boost;
                    }
                }
            }
        }
    }

    // 30% chance life event
    if rng.random() < 0.3 {
        decisions.push(Decision {
            id: "life_event".into(),
            label: "decision.life_event".into(),
            description: "decision.life_event.desc".into(),
            category: "life".into(),
            priority: 10,
        });
    }

    // Fallback: if no decisions available, add a "hold" option
    if decisions.is_empty() {
        decisions.push(Decision {
            id: "hold_to_end".into(),
            label: "decision.hold_to_end".into(),
            description: "decision.hold_to_end.desc".into(),
            category: "wait".into(),
            priority: 10,
        });
    }

    // Sort by priority desc, take top 4
    decisions.sort_by(|a, b| b.priority.cmp(&a.priority));
    decisions.truncate(4);
    decisions
}