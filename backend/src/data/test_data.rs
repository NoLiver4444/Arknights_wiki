use crate::models::operator::{Operator, Class};

pub fn test_data() -> Vec<Operator> {
    vec![
        Operator { 
            id: "char_002_amiya".to_string(), 
            name: "Amiya".to_string(), 
            rarity: 5, 
            class: Class::Caster, 
            avatar: "https://example.com/avatars/amiya.png".to_string(),
        },
        Operator {
            id: "char_003_kalts".to_string(),
            name: "Kal'tsit".to_string(),
            rarity: 6,
            class: Class::Medic,
            avatar: "https://example.com/avatars/kaltsit.png".to_string(),
        }
    ]
}