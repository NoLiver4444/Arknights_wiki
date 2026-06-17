use crate::models::operator::{Operator, Rarity, Class};

pub fn test_data() -> Vec<Operator> {
    vec![
        Operator{ 
            id: "char_002_amiya".to_string(), 
            name: "Amiya".to_string(), 
            rarity: Rarity::FiveStars, 
            class: Class::Caster, 
            avatar: "https://example.com/avatars/amiya.png".to_string() }
    ]
}