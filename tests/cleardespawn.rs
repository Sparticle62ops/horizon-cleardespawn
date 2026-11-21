//! Integration tests for the 1:1 port. A process-wide lock serializes the
//! tests that touch the crate's global state (the live config cell and the
//! renderer registration), mirroring the JVM's single-instance statics.

use horizon_mod_cleardespawn::*;
use std::sync::{Mutex, OnceLock};

fn state_lock() -> std::sync::MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
}

// java: the AutoConfig defaults, and the holder round-trip the serializer does
#[test]
fn config_holder_roundtrip() {
    let _g = state_lock();
    let before = config::client().get();
    assert_eq!(before, config::DEFAULTS);

    config::client().set(config::ModConfig {
        flash_start_time: 7,
        despawn_time: 1200,
        urgent_flash: false,
    });
    let changed = config::client().get();
    assert_eq!(changed.flash_start_time, 7);
    assert_eq!(changed.despawn_time, 1200);
    assert!(!changed.urgent_flash);
    config::client().set(before);
}

// java: the helper reads the live config, the entity argument carried only
#[test]
fn config_helper_tracks_the_holder() {
    let _g = state_lock();
    let before = config::client().get();
    config::client().set(config::ModConfig {
        flash_start_time: 30,
        despawn_time: 36000,
        urgent_flash: true,
    });
    let helper = services::Services::config_helper();
    let entity = seams::ItemEntity::for_tests(35990);
    assert_eq!(config_helper::IConfigHelper::get_flash_start_time(&helper), 30);
    assert_eq!(
        config_helper::IConfigHelper::get_item_despawn_time(&helper, &entity),
        36000
    );
    assert!(config_helper::IConfigHelper::is_urgent_flash_enabled(&helper));
    config::client().set(before);
}

// java: onInitializeClient registers the extended renderer for EntityType.ITEM
#[test]
fn client_init_registers_the_item_renderer() {
    let _g = state_lock();
    let before = seams::registered_for_tests().len();
    clear_despawn::ClearDespawn.on_initialize_client();
    let after = seams::registered_for_tests();
    assert_eq!(after.len(), before + 1);
    assert_eq!(after.last().unwrap().0, seams::EntityType::Item);
    assert_eq!(after.last().unwrap().1, "RenderItemEntityExtended.Factory");
}
