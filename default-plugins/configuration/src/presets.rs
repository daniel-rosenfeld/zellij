pub fn unlock_first_keybinds(primary_modifier: String, secondary_modifier: String) -> String {
    format!("")
}

pub fn default_keybinds(primary_modifier: String, secondary_modifier: String) -> String {
    if primary_modifier.is_empty() && secondary_modifier.is_empty() {
        return default_keybinds_no_modifiers();
    } else if primary_modifier == secondary_modifier {
        return non_colliding_default_keybinds(primary_modifier, secondary_modifier);
    } else if primary_modifier.is_empty() {
        return default_keybinds_no_primary_modifier(secondary_modifier);
    } else if secondary_modifier.is_empty() {
        return default_keybinds_no_secondary_modifier(primary_modifier);
    }
    format!("")
}

pub fn default_keybinds_no_primary_modifier(secondary_modifier: String) -> String {
    format!("")
}

pub fn default_keybinds_no_secondary_modifier(primary_modifier: String) -> String {
    format!("")
}

pub fn default_keybinds_no_modifiers() -> String {
    format!("")
}

pub fn non_colliding_default_keybinds(
    primary_modifier: String,
    secondary_modifier: String,
) -> String {
    format!("")
}
