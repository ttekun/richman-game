use serde::Serialize;

pub const GOAL: f64 = 200_000.0;

#[derive(Clone, Debug, Serialize)]
pub struct TokyoArea {
    pub name: &'static str,
    pub base_price: [f64; 2],
    pub rent_rate: [f64; 2],
}

pub static TOKYO_AREAS: &[TokyoArea] = &[
    TokyoArea { name: "Meguro", base_price: [1700.0, 2200.0], rent_rate: [0.045, 0.060] },
    TokyoArea { name: "Shinagawa", base_price: [1800.0, 2400.0], rent_rate: [0.045, 0.058] },
    TokyoArea { name: "Ebisu", base_price: [1900.0, 2500.0], rent_rate: [0.048, 0.062] },
    TokyoArea { name: "Shibuya", base_price: [2000.0, 2800.0], rent_rate: [0.050, 0.065] },
    TokyoArea { name: "Setagaya", base_price: [1500.0, 2100.0], rent_rate: [0.042, 0.055] },
    TokyoArea { name: "Nakano", base_price: [1400.0, 1900.0], rent_rate: [0.045, 0.058] },
    TokyoArea { name: "Koto", base_price: [1600.0, 2100.0], rent_rate: [0.046, 0.056] },
    TokyoArea { name: "Adachi", base_price: [1200.0, 1700.0], rent_rate: [0.050, 0.065] },
];

#[derive(Clone, Debug, Serialize)]
pub struct GlossaryEntry {
    pub key: String,
    pub term: String,
    pub short: String,
    pub desc: String,
    pub example: String,
}

pub fn glossary() -> Vec<GlossaryEntry> {
    vec![
        GlossaryEntry {
            key: "MRR".into(),
            term: "glossary.MRR.term".into(),
            short: "glossary.MRR.short".into(),
            desc: "glossary.MRR.desc".into(),
            example: "glossary.MRR.example".into(),
        },
        GlossaryEntry {
            key: "ARR".into(),
            term: "glossary.ARR.term".into(),
            short: "glossary.ARR.short".into(),
            desc: "glossary.ARR.desc".into(),
            example: "glossary.ARR.example".into(),
        },
        GlossaryEntry {
            key: "LTV".into(),
            term: "glossary.LTV.term".into(),
            short: "glossary.LTV.short".into(),
            desc: "glossary.LTV.desc".into(),
            example: "glossary.LTV.example".into(),
        },
        GlossaryEntry {
            key: "CF".into(),
            term: "glossary.CF.term".into(),
            short: "glossary.CF.short".into(),
            desc: "glossary.CF.desc".into(),
            example: "glossary.CF.example".into(),
        },
        GlossaryEntry {
            key: "SeriesA".into(),
            term: "glossary.SeriesA.term".into(),
            short: "glossary.SeriesA.short".into(),
            desc: "glossary.SeriesA.desc".into(),
            example: "glossary.SeriesA.example".into(),
        },
        GlossaryEntry {
            key: "Exit".into(),
            term: "glossary.Exit.term".into(),
            short: "glossary.Exit.short".into(),
            desc: "glossary.Exit.desc".into(),
            example: "glossary.Exit.example".into(),
        },
        GlossaryEntry {
            key: "Refinance".into(),
            term: "glossary.Refinance.term".into(),
            short: "glossary.Refinance.short".into(),
            desc: "glossary.Refinance.desc".into(),
            example: "glossary.Refinance.example".into(),
        },
        GlossaryEntry {
            key: "Equity".into(),
            term: "glossary.Equity.term".into(),
            short: "glossary.Equity.short".into(),
            desc: "glossary.Equity.desc".into(),
            example: "glossary.Equity.example".into(),
        },
        GlossaryEntry {
            key: "PropTech".into(),
            term: "glossary.PropTech.term".into(),
            short: "glossary.PropTech.short".into(),
            desc: "glossary.PropTech.desc".into(),
            example: "glossary.PropTech.example".into(),
        },
        GlossaryEntry {
            key: "CAC".into(),
            term: "glossary.CAC.term".into(),
            short: "glossary.CAC.short".into(),
            desc: "glossary.CAC.desc".into(),
            example: "glossary.CAC.example".into(),
        },
        GlossaryEntry {
            key: "Bootstrap".into(),
            term: "glossary.Bootstrap.term".into(),
            short: "glossary.Bootstrap.short".into(),
            desc: "glossary.Bootstrap.desc".into(),
            example: "glossary.Bootstrap.example".into(),
        },
        GlossaryEntry {
            key: "VC".into(),
            term: "glossary.VC.term".into(),
            short: "glossary.VC.short".into(),
            desc: "glossary.VC.desc".into(),
            example: "glossary.VC.example".into(),
        },
        GlossaryEntry {
            key: "IPO".into(),
            term: "glossary.IPO.term".into(),
            short: "glossary.IPO.short".into(),
            desc: "glossary.IPO.desc".into(),
            example: "glossary.IPO.example".into(),
        },
        GlossaryEntry {
            key: "MA".into(),
            term: "glossary.MA.term".into(),
            short: "glossary.MA.short".into(),
            desc: "glossary.MA.desc".into(),
            example: "glossary.MA.example".into(),
        },
        GlossaryEntry {
            key: "Multiplier".into(),
            term: "glossary.Multiplier.term".into(),
            short: "glossary.Multiplier.short".into(),
            desc: "glossary.Multiplier.desc".into(),
            example: "glossary.Multiplier.example".into(),
        },
        GlossaryEntry {
            key: "PoC".into(),
            term: "glossary.PoC.term".into(),
            short: "glossary.PoC.short".into(),
            desc: "glossary.PoC.desc".into(),
            example: "glossary.PoC.example".into(),
        },
        GlossaryEntry {
            key: "Pivot".into(),
            term: "glossary.Pivot.term".into(),
            short: "glossary.Pivot.short".into(),
            desc: "glossary.Pivot.desc".into(),
            example: "glossary.Pivot.example".into(),
        },
    ]
}