use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operator {
    pub id: String,
    pub name: String,
    pub rarity: Rarity,
    pub class: Class,
    pub avatar: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Class {
    Caster,
    Guard,
    Sniper,
    Vanguard,
    Defender,
    Medic,
    Supporter,
    Specialist,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Rarity {
    OneStar = 1,
    TwoStars = 2,
    ThreeStars = 3,
    FourStars = 4,
    FiveStars = 5,
    SixStars = 6,
}