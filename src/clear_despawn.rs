//! 1:1 port of io.github.strikerrocker.cleardespawn.ClearDespawn (the fabric
//! ClientModInitializer).

use crate::render_item_entity_extended::Factory;
use crate::seams::{EntityType, register_entity_renderer};

// java: public class ClearDespawn implements ClientModInitializer
pub struct ClearDespawn;

impl ClearDespawn {
    // java: onInitializeClient():
    //   EntityRendererRegistry.register(EntityType.ITEM,
    //   new RenderItemEntityExtended.Factory());
    pub fn on_initialize_client(&self) {
        register_entity_renderer(EntityType::Item, "RenderItemEntityExtended.Factory");
        let _ = Factory;
    }
}
