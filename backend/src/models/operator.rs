use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operator {
    pub id: String,
    pub name: String,
    pub rarity: u8,
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

#[derive(Debug, Deserialize)]
pub struct HellaOperator {
    pub _id: String,
    pub canon: String,
    pub value: HellaValue,
}

#[derive(Debug, Deserialize)]
pub struct HellaValue {
    pub id: String,
    pub data: HellaData,
}

#[derive(Debug, Deserialize)]
pub struct HellaData {
    pub name: String,
    pub rarity: String,
    pub profession: String,
}

fn parse_rarity(rarity_str: &str) -> u8 {
    match rarity_str {
        "TIER_1" => 1,
        "TIER_2" => 2,
        "TIER_3" => 3,
        "TIER_4" => 4,
        "TIER_5" => 5,
        "TIER_6" => 6,
        _ => 1,
    }
}

impl Class {
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "CASTER" => Class::Caster,
            "GUARD" => Class::Guard,
            "SNIPER" => Class::Sniper,
            "VANGUARD" => Class::Vanguard,
            "DEFENDER" => Class::Defender,
            "MEDIC" => Class::Medic,
            "SUPPORTER" => Class::Supporter,
            "SPECIALIST" => Class::Specialist,
            _ => Class::Guard,
        }
    }
}

impl From<HellaOperator> for Operator {
    fn from(hella: HellaOperator) -> Self {
        let data = hella.value.data;
        
        let rarity = parse_rarity(&data.rarity);
        let class = Class::from_str(&data.profession);
        
        let avatar = format!(
            "https://raw.githubusercontent.com/kiraio-moe/Arknights-Base/main/Spine%202D/{}/{}.png",
            hella.value.id,
            hella.value.id
        );
        
        Operator {
            id: hella.value.id,
            name: data.name,
            rarity,
            class,
            avatar,
        }
    }
}