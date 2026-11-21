//! 1:1 port of io.github.strikerrocker.cleardespawn.ModConfig (the fabric
//! AutoConfig config, `@Config(name = "cleardespawn")`).

use std::sync::{Mutex, OnceLock};

/// java: @ConfigEntry.BoundedDiscrete(max = 300) on flashStartTime.
pub const FLASH_START_TIME_MAX: i32 = 300;

/// java: @ConfigEntry.BoundedDiscrete(max = 36000) on despawnTime.
pub const DESPAWN_TIME_MAX: i32 = 36_000;

// java: public class ModConfig implements ConfigData
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModConfig {
    /// java: public int flashStartTime = 20;
    pub flash_start_time: i32,
    /// java: public int despawnTime = 6000;
    pub despawn_time: i32,
    /// java: public boolean urgentFlash = true;
    pub urgent_flash: bool,
}

/// java: AutoConfig.register(ModConfig.class, Toml4jConfigSerializer::new):
/// the registered defaults; the serializer round-trips the same three entries
/// (the toml text format upstream Toml4jConfigSerializer writes).
pub const DEFAULTS: ModConfig = ModConfig {
    flash_start_time: 20,
    despawn_time: 6000,
    urgent_flash: true,
};

// java: AutoConfig.getConfigHolder(ModConfig.class).getConfig() — the live
// config instance the ConfigHelper reads and the settings sync into.
struct Holder {
    config: Mutex<ModConfig>,
}

fn holder() -> &'static Holder {
    static HOLDER: OnceLock<Holder> = OnceLock::new();
    HOLDER.get_or_init(|| Holder {
        config: Mutex::new(DEFAULTS),
    })
}

impl Holder {
    fn get(&self) -> ModConfig {
        *self.config.lock().unwrap()
    }

    // the serializer's save path, driven by the settings sync
    fn set(&self, config: ModConfig) {
        *self.config.lock().unwrap() = config;
    }
}

// java: public static ModConfig config — reached as the config-holder cell.
pub struct ConfigCell;

impl ConfigCell {
    pub fn get(&self) -> ModConfig {
        holder().get()
    }

    pub fn set(&self, config: ModConfig) {
        holder().set(config);
    }
}

/// java: ClearDespawn.config — the static live config instance.
pub fn client() -> &'static ConfigCell {
    static CELL: OnceLock<ConfigCell> = OnceLock::new();
    CELL.get_or_init(|| ConfigCell)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registered_defaults_match_upstream() {
        assert_eq!(DEFAULTS.flash_start_time, 20);
        assert_eq!(DEFAULTS.despawn_time, 6000);
        assert!(DEFAULTS.urgent_flash);
    }

    #[test]
    fn bounded_discrete_maximums_match_upstream() {
        assert_eq!(FLASH_START_TIME_MAX, 300);
        assert_eq!(DESPAWN_TIME_MAX, 36_000);
    }
}
