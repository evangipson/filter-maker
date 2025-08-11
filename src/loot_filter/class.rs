use std::fmt::Display;

#[derive(PartialEq)]
pub struct Class {
    pub name: &'static str,
}

impl Class {
    pub const CURRENCY: Class = Class::new("Currency");
    pub const MAPS: Class = Class::new("Maps");
    pub const LIFE_FLASKS: Class = Class::new("Life Flasks");
    pub const MANA_FLASKS: Class = Class::new("Mana Flasks");
    pub const HYBRID_FLASKS: Class = Class::new("Hybrid Flasks");
    pub const JEWELS: Class = Class::new("Jewels");
    pub const SKILL_GEMS: Class = Class::new("Skill Gem");
    pub const SUPPORT_GEMS: Class = Class::new("Support Gem");
    pub const BREACHSTONES: Class = Class::new("Breachstones");
    pub const MAP_FRAGMENTS: Class = Class::new("Map Fragments");
    pub const DIVINATION_CARDS: Class = Class::new("Divination Cards");
    pub const TINCTURES: Class = Class::new("Tinctures");
    pub const CONTRACTS: Class = Class::new("Contracts");
    pub const AMULETS: Class = Class::new("Amulets");
    pub const BELTS: Class = Class::new("Belts");
    pub const BODY_ARMORS: Class = Class::new("Body Armours");
    pub const BOOTS: Class = Class::new("Boots");
    pub const BOWS: Class = Class::new("Bows");
    pub const CLAWS: Class = Class::new("Claws");
    pub const DAGGERS: Class = Class::new("Daggers");
    pub const GLOVES: Class = Class::new("Gloves");
    pub const HELMETS: Class = Class::new("Helmets");
    pub const ONE_HAND_AXES: Class = Class::new("One Hand Axes");
    pub const ONE_HAND_SWORDS: Class = Class::new("One Hand Swords");
    pub const ONE_HAND_MACES: Class = Class::new("One Hand Maces");
    pub const QUIVERS: Class = Class::new("Quivers");
    pub const RINGS: Class = Class::new("Rings");
    pub const RUNE_DAGGERS: Class = Class::new("Rune Daggers");
    pub const SCEPTRES: Class = Class::new("Sceptres");
    pub const SHIELDS: Class = Class::new("Shields");
    pub const STAVES: Class = Class::new("Staves");
    pub const THRUSTING_ONE_HAND_SWORDS: Class = Class::new("Thrusting One Hand Swords");
    pub const TWO_HAND_AXES: Class = Class::new("Two Hand Axes");
    pub const TWO_HAND_MACES: Class = Class::new("Two Hand Maces");
    pub const TWO_HAND_SWORDS: Class = Class::new("Two Hand Swords");
    pub const UTILITY_FLASKS: Class = Class::new("Utility Flasks");
    pub const WANDS: Class = Class::new("Wands");
    pub const WARSTAVES: Class = Class::new("Warstaves");
    pub const ALL_CLASSES: [Class; 38] = [
        Class::CURRENCY,
        Class::MAPS,
        Class::LIFE_FLASKS,
        Class::MANA_FLASKS,
        Class::HYBRID_FLASKS,
        Class::SKILL_GEMS,
        Class::SUPPORT_GEMS,
        Class::JEWELS,
        Class::BREACHSTONES,
        Class::MAP_FRAGMENTS,
        Class::DIVINATION_CARDS,
        Class::TINCTURES,
        Class::CONTRACTS,
        Class::AMULETS,
        Class::BELTS,
        Class::BODY_ARMORS,
        Class::BOOTS,
        Class::BOWS,
        Class::CLAWS,
        Class::DAGGERS,
        Class::GLOVES,
        Class::HELMETS,
        Class::ONE_HAND_AXES,
        Class::ONE_HAND_SWORDS,
        Class::ONE_HAND_MACES,
        Class::QUIVERS,
        Class::RINGS,
        Class::RUNE_DAGGERS,
        Class::SCEPTRES,
        Class::SHIELDS,
        Class::STAVES,
        Class::THRUSTING_ONE_HAND_SWORDS,
        Class::TWO_HAND_AXES,
        Class::TWO_HAND_MACES,
        Class::TWO_HAND_SWORDS,
        Class::UTILITY_FLASKS,
        Class::WANDS,
        Class::WARSTAVES,
    ];

    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
