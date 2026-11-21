//! 1:1 port of io.github.strikerrocker.cleardespawn.IConfigHelper and the
//! fabric ConfigHelper.

use crate::config;

// java: public interface IConfigHelper
pub trait IConfigHelper {
    // java: int getFlashStartTime()
    fn get_flash_start_time(&self) -> i32;

    // java: int getItemDespawnTime(ItemEntity entity)
    fn get_item_despawn_time(&self, entity: &crate::seams::ItemEntity) -> i32;

    // java: boolean isUrgentFlashEnabled()
    fn is_urgent_flash_enabled(&self) -> bool;
}

// java: public class ConfigHelper implements IConfigHelper (the fabric service)
pub struct ConfigHelper;

impl IConfigHelper for ConfigHelper {
    fn get_flash_start_time(&self) -> i32 {
        config::client().get().flash_start_time
    }

    fn get_item_despawn_time(&self, _entity: &crate::seams::ItemEntity) -> i32 {
        // Upstream reads only the config here (ClearDespawn.config.despawnTime);
        // the entity parameter is carried for signature fidelity.
        config::client().get().despawn_time
    }

    fn is_urgent_flash_enabled(&self) -> bool {
        config::client().get().urgent_flash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn helper_reads_the_live_config() {
        let before = config::client().get();
        let helper = ConfigHelper;
        let entity = crate::seams::ItemEntity::for_tests(0);
        assert_eq!(helper.get_flash_start_time(), before.flash_start_time);
        assert_eq!(helper.get_item_despawn_time(&entity), before.despawn_time);
        assert_eq!(helper.is_urgent_flash_enabled(), before.urgent_flash);
    }
}
