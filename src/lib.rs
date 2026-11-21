//! 1:1 Horizon Rust port of ClearDespawn (StrikerRockers-Mods, MIT).
//! Upstream: https://github.com/StrikerRockers-Mods/ClearDespawn
//!
//! Every Java class maps to one module below, same names, same logic, same
//! defaults, same control flow. Host seams (seams.rs) carry the Minecraft
//! client render surfaces the engine binds; where an upstream type is
//! engine-bound, the seam mirrors its Java calls one to one.
//!
//! This root carries the mod API wiring on the pinned SDK (Horizon f00125f61f,
//! mixins e59da909, mod api 0.2):
//! - the original's three config entries as settings with upstream's own
//!   autoconfig lang keys as the labels (`flashStartTime` a 0..300 range,
//!   default 20; `despawnTime` a 0..36000 range, default 6000; `urgentFlash`
//!   a toggle, default on);
//! - `client_setup` syncs the settings into the live config cell the
//!   ConfigHelper reads (the AutoConfig holder load path);
//! - the original's entry (`ClearDespawn.onInitializeClient()` = registering
//!   the extended item entity renderer) runs on client setup through the
//!   registration seam.
//!
//! Gap, tracked: the mod API has no surface for registering a custom renderer
//! for an existing entity type (the extended `ItemEntityRenderer` with the
//! despawn flash); the registration is recorded on the seam until one lands.
//!
//! Upstream licence: MIT. Copyright (c) 2019, 2020, 2021, 2022 StrikerRocker;
//! Copyright (c) 2020 Guntram Blohm. Carried verbatim in LICENSE, and the
//! Rust translation is distributed under the same licence.
//! Source: https://github.com/Sparticle62ops/horizon-cleardespawn

use mod_api::{ClientSetupEvent, ClientHost, RegistrationContext, Registrar, SettingSpec};

pub mod clear_despawn;
pub mod config;
pub mod config_helper;
pub mod constants;
pub mod mod_menu;
pub mod render_item_entity_extended;
pub mod seams;
pub mod services;

/// java: text.autoconfig.cleardespawn.option.* — the entries' own lang keys.
const LABEL_FLASH_START_TIME: &str = "text.autoconfig.cleardespawn.option.flashStartTime";
const LABEL_DESPAWN_TIME: &str = "text.autoconfig.cleardespawn.option.despawnTime";
const LABEL_URGENT_FLASH: &str = "text.autoconfig.cleardespawn.option.urgentFlash";

#[derive(Default)]
struct Mod;

#[mod_api::horizon_mod]
impl mod_api::Hooks for Mod {
    /// 1:1 of the original's three config entries the settings page can hold,
    /// the BoundedDiscrete bounds and field defaults verbatim, upstream's own
    /// autoconfig lang keys as the labels.
    fn register(&self, registrar: &mut dyn Registrar, _context: &RegistrationContext) {
        let _ = registrar.register_setting(SettingSpec::range(
            "flashStartTime",
            LABEL_FLASH_START_TIME,
            0.0,
            config::FLASH_START_TIME_MAX as f64,
            1.0,
            config::DEFAULTS.flash_start_time as f64,
        ));
        let _ = registrar.register_setting(SettingSpec::range(
            "despawnTime",
            LABEL_DESPAWN_TIME,
            0.0,
            config::DESPAWN_TIME_MAX as f64,
            1.0,
            config::DEFAULTS.despawn_time as f64,
        ));
        let _ = registrar.register_setting(SettingSpec::toggle(
            "urgentFlash",
            LABEL_URGENT_FLASH,
            config::DEFAULTS.urgent_flash,
        ));
    }

    /// 1:1 of the original's entry (the AutoConfig holder load plus
    /// ClearDespawn.onInitializeClient): the settings sync into the live
    /// config cell, then the extended item entity renderer registers.
    fn client_setup(&self, host: &mut dyn ClientHost, _event: &mut ClientSetupEvent) {
        let flash_start_time = host.setting_f64("flashStartTime").unwrap_or(20.0) as i32;
        let despawn_time = host.setting_f64("despawnTime").unwrap_or(6000.0) as i32;
        let urgent_flash = host.setting_bool("urgentFlash").unwrap_or(true);
        config::client().set(config::ModConfig {
            flash_start_time,
            despawn_time,
            urgent_flash,
        });
        clear_despawn::ClearDespawn.on_initialize_client();
    }
}
