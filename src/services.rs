//! 1:1 port of io.github.strikerrocker.cleardespawn.Services.

use crate::config_helper::{ConfigHelper, IConfigHelper};
use crate::constants::log;

// java: public class Services
pub struct Services;

impl Services {
    // java: ServiceLoader.load(IConfigHelper.class).findFirst().orElseThrow(...)
    // with the debug log. The fabric service is this mod's own ConfigHelper; the
    // engine keeps it reachable through the same path, so the loader resolves to
    // it directly (the orElseThrow is unreachable for this mod's own class).
    pub fn config_helper() -> ConfigHelper {
        log().debug("Loaded service IConfigHelper");
        ConfigHelper
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_resolves() {
        // java: .orElseThrow(() -> new NullPointerException(...)) — a resolved
        // service answers all three config reads.
        let helper = Services::config_helper();
        let entity = crate::seams::ItemEntity::for_tests(0);
        let _ = helper.get_flash_start_time();
        let _ = helper.get_item_despawn_time(&entity);
        let _ = helper.is_urgent_flash_enabled();
    }
}
