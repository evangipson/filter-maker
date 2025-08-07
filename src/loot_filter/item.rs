pub struct Item {
    pub base_type: &'static str,
}

impl Item {
    pub const MIRROR: Item = Item::new("Mirror");
    pub const DIVINE: Item = Item::new("Divine");
    pub const RELIQUARY_KEY: Item = Item::new("Reliquary Key");
    pub const ALBINO_RHOA_FEATHER: Item = Item::new("Albino Rhoa Feather");
    pub const AWAKENERS_ORB: Item = Item::new("Awakener's Orb");
    pub const BLESSING_OF: Item = Item::new("Blessing of");
    pub const CRUSADERS_EXALTED: Item = Item::new("Crusader's Exalted");
    pub const ELDERS_EXALTED: Item = Item::new("Elder's Exalted");
    pub const ETERNAL_ORB: Item = Item::new("Eternal Orb");
    pub const EXCEPTIONAL_ELDRITCH: Item = Item::new("Exceptional Eldritch");
    pub const FRACTURING_ORB: Item = Item::new("Fracturing Orb");
    pub const FRACTURING_SHARD: Item = Item::new("Fracturing Shard");
    pub const SUPER_VALUABLES: [Item; 11] = [
        Item::MIRROR,
        Item::DIVINE,
        Item::RELIQUARY_KEY,
        Item::ALBINO_RHOA_FEATHER,
        Item::AWAKENERS_ORB,
        Item::BLESSING_OF,
        Item::CRUSADERS_EXALTED,
        Item::ETERNAL_ORB,
        Item::EXCEPTIONAL_ELDRITCH,
        Item::FRACTURING_ORB,
        Item::FRACTURING_SHARD,
    ];

    pub const CLAW: Item = Item::new("Claw");
    pub const CONVOKING_WAND: Item = Item::new("Convoking Wand");
    pub const DAGGER: Item = Item::new("Dagger");
    pub const ONE_HAND_AXE: Item = Item::new("One Hand Axe");
    pub const ONE_HAND_MACE: Item = Item::new("One Hand Mace");
    pub const ONE_HAND_SWORD: Item = Item::new("One Hand Sword");
    pub const RUNE_DAGGER: Item = Item::new("Rune Dagger");
    pub const SCEPTRE: Item = Item::new("Sceptre");
    pub const THRUSTING_ONE_HAND_SWORD: Item = Item::new("Thrusting One hand Sword");
    pub const WAND: Item = Item::new("Wand");
    pub const ONE_HANDED_WEAPONS: [Item; 10] = [
        Item::CLAW,
        Item::CONVOKING_WAND,
        Item::DAGGER,
        Item::ONE_HAND_AXE,
        Item::ONE_HAND_MACE,
        Item::ONE_HAND_SWORD,
        Item::RUNE_DAGGER,
        Item::SCEPTRE,
        Item::THRUSTING_ONE_HAND_SWORD,
        Item::WAND,
    ];

    pub const BOW: Item = Item::new("Bow");
    pub const STAFF: Item = Item::new("Staff");
    pub const TWO_HAND_AXE: Item = Item::new("Two Hand Axe");
    pub const TWO_HAND_MACE: Item = Item::new("Two Hand Mace");
    pub const TWO_HAND_SWORD: Item = Item::new("Two Hand Sword");
    pub const WARSTAFF: Item = Item::new("Warstaff");
    pub const TWO_HANDED_WEAPONS: [Item; 6] = [
        Item::BOW,
        Item::STAFF,
        Item::TWO_HAND_AXE,
        Item::TWO_HAND_MACE,
        Item::TWO_HAND_SWORD,
        Item::WARSTAFF,
    ];

    const fn new(base_type: &'static str) -> Self {
        Self { base_type }
    }
}
