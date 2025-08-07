use filter_maker::loot_filter::{filter::Filter, rule::Rule};

fn main() {
    let filter_rules = vec![Rule::SHOW_ALL_SUPER_VALUABLES, Rule::SHOW_ALL_MAGIC_SWORDS];

    let item_filter = Filter::new(filter_rules);

    print!("{item_filter}");
}
