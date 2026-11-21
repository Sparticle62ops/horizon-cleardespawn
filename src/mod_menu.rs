//! 1:1 port of io.github.strikerrocker.cleardespawn.ModMenuIntegration.

// java: getModConfigScreenFactory() returns AutoConfig.getConfigScreen
use crate::config_helper::IConfigHelper;
// (ModConfig.class, screen) — the config screen over ModConfig. In this port
// the mod API's settings page hosts the same three entries (registered in
// lib.rs with upstream's own lang keys), so the factory resolves to that
// settings page; no separate screen ships.
pub const CONFIG_SCREEN: &str = "horizon mod settings: cleardespawn";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_config_screen_is_the_settings_page() {
        assert_eq!(CONFIG_SCREEN, "horizon mod settings: cleardespawn");
    }
}
