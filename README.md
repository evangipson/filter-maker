# Filter Maker
An item filter generator for Path of Exile 1 & Path of Exile 2.

## Getting started
- Run `./update.ps1` to generate a filter for Path of Exile 1 (saved as `config/filter.poe1.toml`)
- Run `./update.ps1 -Poe2` to generate a filter for Path of Exile 2 (saved as `config/filter.poe2.toml`)
- Run `./update.ps1 -Latest` to replace your local filter with the latest example (this will **not** replace your `configs.destination`)
- Run `./update.ps1 -Poe2 -Latest` to replace your local Path of Exile 2 filter with the latest example

## Customizing the filter
### Adding a color
To add a color, you can add an RGBA value to the `config.palette` section of `filter.poe1.toml` or `filter.poe2.toml`. Note these colors can be references in any of the base styles or filters by `name`:
```toml
palette = [
    # ... existing colors
    { name = "bright_red", color = {r = 255, g = 0, b = 0, a = 255 }},
]
```

### Adding a base style
To add a base style, you can add to the `config.styles` section of `filter.poe1.toml` or `filter.poe2.toml`:
```toml
styles = [
    # ... existing styles
    { name = "Staves (base style)", classes = [ "Staves", "Warstaves" ], font = "white", background = "bright_red", size = 32 },
]
```

### Adding a filter
To add a filter, you can add to the `[[filters]]` section of `filter.poe1.toml` or `filter.poe2.toml`. Note you don't have to redefine styles:
```toml
[[filters]]
name = "Level 83 Staves"
classes = [ "Staves", "Warstaves" ]
size = 42
item_level = 83
```

You can also add a filter for specific items using the `items` rule:
```toml
[[filters]]
name = "Corrupted Karui Choppers"
items = [ "Karui Chopper" ]
size = 42
theme = { font = "bright_red", background = "faded_black", outline = "bright_red" }
corrupted_mods = 1
```

## Keywords
### Colors
|Keyword|Meaning|
|-|-|
|`name`|The name of the color, which can be used in any base style or filter.|
|`r`|The red value of the color, from 0 - 255.|
|`g`|The green value of the color, from 0 - 255.|
|`b`|The blue value of the color, from 0 - 255.|
|`a`|The alpha (transparency) value of the color, from 0 - 255.|

### Base Styles
|Keyword|Meaning|
|-|-|
|`name`|The name of the base style, which will show up as a comment in the base style.|
|`items`|The specific items to include in the base style.|
|`classes`|The specific item classes to include in the base style.|
|`rarity`|The rarity to include in the base style.|
|`font`|The color of the text.|
|`background`|The color of the background.|
|`outline`|The color of the outline.|
|`size`|The size of the text.|
|`is_synthesised`|When set to `true`, incidates a base style is for an item that is synthesised, defaults to `false`.|
|`is_fractured`|When set to `true`, indicates a base style is for an item that has a fractured mod, defaults to `false`.|
|`is_influenced`|When set to `true`, indicates a base style is for an item that is influenced, defaults to `false`.|
|`is_enchanted`|When set to `true`, indicates a base style is for an item that is enchanted, defaults to `false`.|
|`is_veiled`|When set to `true`, indicates a base style is for an item that has at least one veiled mod, defaults to `false`.|
|`is_replica`|When set to `true`, indicates a base style is for a replica item, defaults to `false`.|
|`good_mods`|How many good mods an item needs to be included in a base style, defaults to `0`.|
|`corrupted_mods`|How many corrupted mods and item needs to be included in a base style, defaults to `0`.|
|`item_level`|The minimum item level needed for an item to be included in a base style.|
|`item_tier`|The minimum unintendified tier needed for an item to be included in a base style, defaults to `0`.|
|`strict`|When set to `false`, any `items` or `classes` do not need to be an exact match, defaults to `true`.|

### Filter
|Keyword|Meaning|
|-|-|
|`name`|The name of the filter, which will show up as a comment in the filter rule.|
|`items`|The specific items to include in the filter.|
|`classes`|The specific item classes to include in the filter.|
|`rarity`|The rarity to include in the filter.|
|`theme`|The `font`, `background`, and `outline` of the filter.|
|`size`|The size of the text.|
|`sound`|The `sound_type` and `volume` of the filter.|
|`icon`|The `icon_type`, `color`, and `size` of a minimap icon for the filter.|
|`beam`|When defined, will display a beam of pre-defined color for the filter.|
|`is_synthesised`|When set to `true`, incidates a filter is for an item that is synthesised, defaults to `false`.|
|`is_fractured`|When set to `true`, indicates a filter is for an item that has a fractured mod, defaults to `false`.|
|`is_influenced`|When set to `true`, indicates a filter is for an item that is influenced, defaults to `false`.|
|`is_enchanted`|When set to `true`, indicates a filter is for an item that is enchanted, defaults to `false`.|
|`is_veiled`|When set to `true`, indicates a filter is for an item that has at least one veiled mod, defaults to `false`.|
|`is_replica`|When set to `true`, indicates a filter is for a replica item, defaults to `false`.|
|`good_mods`|How many good mods an item needs to be included in a filter, defaults to `0`.|
|`corrupted_mods`|How many corrupted mods and item needs to be included in a filter, defaults to `0`.|
|`quality`|The minimum item quality for an item to be included in a filter.|
|`map_tier`|The minimum map tier for a map to be included in a filter.|
|`links`|The minimum number of linked sockets for an item to be included in a filter, defaults to `0`.|
|`item_level`|The minimum item level needed for an item to be included in a filter, defaults to `0`.|
|`item_tier`|The minimum unintendified tier needed for an item to be included in a base style.|
|`stack_size`|The minimum amount of an item in one stack for an item to be included in a filter.|
|`strict`|When set to `false`, any `items` or `classes` do not need to be an exact match, defaults to `true`.|
|`hide`|When set to `true`, will produce a filter rule to hide instead of show, defaults to `false`.|