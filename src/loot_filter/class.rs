use std::fmt::Display;

/// [`Class`] represents an item class that encapsulates many similar items.
#[derive(PartialEq)]
pub struct Class {
    /// [`Class::name`] represents the name of an item class.
    pub name: &'static str,
}

/// Implement [`Class`].
impl Class {
    /// [`Class::CURRENCY`] is the item class for all currency items.
    pub const CURRENCY: Class = Class::new("Currency");

    /// [`Class::MAPS`] is the item class for all map items.
    pub const MAPS: Class = Class::new("Maps");

    /// [`Class::LIFE_FLASKS`] is the item class for all life flasks.
    pub const LIFE_FLASKS: Class = Class::new("Life Flasks");

    /// [`Class::MANA_FLASKS`] is the item class for all mana flasks.
    pub const MANA_FLASKS: Class = Class::new("Mana Flasks");

    /// [`Class::HYBRID_FLASKS`] is the item class for all hybrid (life and mana) flasks.
    pub const HYBRID_FLASKS: Class = Class::new("Hybrid Flasks");

    /// [`Class::JEWELS`] is the item class for all jewels.
    pub const JEWELS: Class = Class::new("Jewels");

    /// [`Class::ABYSS_JEWELS`] is the item class for all abyss jewels.
    pub const ABYSS_JEWELS: Class = Class::new("Abyss Jewels");

    /// [`Class::SKILL_GEMS`] is the item class for all skill gems.
    pub const SKILL_GEMS: Class = Class::new("Skill Gem");

    /// [`Class::SUPPORT_GEMS`] is the item class for all support gems.
    pub const SUPPORT_GEMS: Class = Class::new("Support Gem");

    /// [`Class::BREACHSTONES`] is the item class for all breachstone items.
    pub const BREACHSTONES: Class = Class::new("Breachstones");

    /// [`Class::MAP_FRAGMENTS`] is the item class for all map fragment items.
    pub const MAP_FRAGMENTS: Class = Class::new("Map Fragments");

    /// [`Class::DIVINATION_CARDS`] is the item class for all divination cards.
    pub const DIVINATION_CARDS: Class = Class::new("Divination Cards");

    /// [`Class::TINCTURES`] is the item class for all tinctures.
    pub const TINCTURES: Class = Class::new("Tinctures");

    /// [`Class::CONTRACTS`] is the item class for all heist contracts.
    pub const CONTRACTS: Class = Class::new("Contracts");

    /// [`Class::AMULETS`] is the item class for all amulets.
    pub const AMULETS: Class = Class::new("Amulets");

    /// [`Class::BELTS`] is the item class for all belts.
    pub const BELTS: Class = Class::new("Belts");

    /// [`Class::BODY_ARMORS`] is the item class for all body armors.
    pub const BODY_ARMORS: Class = Class::new("Body Armours");

    /// [`Class::BOOTS`] is the item class for all boots.
    pub const BOOTS: Class = Class::new("Boots");

    /// [`Class::BOWS`] is the item class for all bows.
    pub const BOWS: Class = Class::new("Bows");

    /// [`Class::CLAWS`] is the item class for all claw weapons.
    pub const CLAWS: Class = Class::new("Claws");

    /// [`Class::DAGGERS`] is the item class for all daggers.
    pub const DAGGERS: Class = Class::new("Daggers");

    /// [`Class::GLOVES`] is the item class for all gloves.
    pub const GLOVES: Class = Class::new("Gloves");

    /// [`Class::HELMETS`] is the item class for all helmets.
    pub const HELMETS: Class = Class::new("Helmets");

    /// [`Class::ONE_HAND_AXES`] is the item class for all one-handed axes.
    pub const ONE_HAND_AXES: Class = Class::new("One Hand Axes");

    /// [`Class::ONE_HAND_SWORDS`] is the item class for all one-handed swords.
    pub const ONE_HAND_SWORDS: Class = Class::new("One Hand Swords");

    /// [`Class::ONE_HAND_MACES`] is the item class for all one-handed maces.
    pub const ONE_HAND_MACES: Class = Class::new("One Hand Maces");

    /// [`Class::QUIVERS`] is the item class for all quivers.
    pub const QUIVERS: Class = Class::new("Quivers");

    /// [`Class::RINGS`] is the item class for all rings.
    pub const RINGS: Class = Class::new("Rings");

    /// [`Class::RUNE_DAGGERS`] is the item class for all rune dagger weapons.
    pub const RUNE_DAGGERS: Class = Class::new("Rune Daggers");

    /// [`Class::SCEPTRES`] is the item class for all sceptres.
    pub const SCEPTRES: Class = Class::new("Sceptres");

    /// [`Class::SHIELDS`] is the item class for all shields.
    pub const SHIELDS: Class = Class::new("Shields");

    /// [`Class::STAVES`] is the item class for all magical two-handed staves.
    pub const STAVES: Class = Class::new("Staves");

    /// [`Class::THRUSTING_ONE_HAND_SWORDS`] is the item class for all thrusting one-handed swords.
    pub const THRUSTING_ONE_HAND_SWORDS: Class = Class::new("Thrusting One Hand Swords");

    /// [`Class::TWO_HAND_AXES`] is the item class for all two-handed axes.
    pub const TWO_HAND_AXES: Class = Class::new("Two Hand Axes");

    /// [`Class::TWO_HAND_MACES`] is the item class for all two-handed maces.
    pub const TWO_HAND_MACES: Class = Class::new("Two Hand Maces");

    /// [`Class::TWO_HAND_SWORDS`] is the item class for all two-handed swords.
    pub const TWO_HAND_SWORDS: Class = Class::new("Two Hand Swords");

    /// [`Class::UTILITY_FLASKS`] is the item class for all utility flasks.
    pub const UTILITY_FLASKS: Class = Class::new("Utility Flasks");

    /// [`Class::WANDS`] is the item class for all one-handed wands.
    pub const WANDS: Class = Class::new("Wands");

    /// [`Class::WARSTAVES`] is the item class for all two-handed quarterstaves.
    pub const WARSTAVES: Class = Class::new("Warstaves");

    /// [`Class::ALL_CLASSES`] is a collection of all item classes.
    pub const ALL_CLASSES: [Class; 39] = [
        Class::CURRENCY,
        Class::MAPS,
        Class::LIFE_FLASKS,
        Class::MANA_FLASKS,
        Class::HYBRID_FLASKS,
        Class::SKILL_GEMS,
        Class::SUPPORT_GEMS,
        Class::JEWELS,
        Class::ABYSS_JEWELS,
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

    /// [`Class::new`] will create a new [`Class`].
    const fn new(name: &'static str) -> Self {
        Self { name }
    }
}

/// Implement [`Display`] for [`Class`].
impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
