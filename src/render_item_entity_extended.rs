//! 1:1 port of io.github.strikerrocker.cleardespawn.RenderItemEntityExtended.

use crate::config_helper::IConfigHelper;
use crate::seams::{
    EntityRenderer, EntityRendererContext, ItemEntity, ItemEntityRenderer, MultiBufferSource,
    PoseStack,
};
use crate::services::Services;

// java: public class RenderItemEntityExtended extends ItemEntityRenderer
pub struct RenderItemEntityExtended<R: ItemEntityRenderer> {
    // java: super(context) — the base ItemEntityRenderer.
    base: R,
}

impl<R: ItemEntityRenderer> RenderItemEntityExtended<R> {
    pub fn new(base: R) -> Self {
        RenderItemEntityExtended { base }
    }

    // java: public void render(ItemEntity entity, float entityYaw,
    // float partialTicks, PoseStack matrixStack, MultiBufferSource buffer,
    // int packedLight) — the flash logic verbatim; falls through to
    // super.render when the item is not in its flashing window.
    pub fn render(
        &self,
        entity: &ItemEntity,
        entity_yaw: f32,
        partial_ticks: f32,
        matrix_stack: &mut PoseStack,
        buffer: &MultiBufferSource,
        packed_light: i32,
    ) {
        let config_helper = Services::config_helper();
        let remaining_time = config_helper.get_item_despawn_time(entity) - entity.get_age();
        if remaining_time <= 20 * config_helper.get_flash_start_time() {
            if config_helper.is_urgent_flash_enabled() {
                let flash_factor = (remaining_time / 20).max(2);
                // java: remainingTime % flashFactor < 0.5f * flashFactor — the
                // int remainder promotes to float against the float product.
                let flash_phase = ((remaining_time % flash_factor) as f32) < (0.5 * flash_factor as f32);
        if flash_phase {
                    return;
                }
            } else if remaining_time % 20 < 10 {
                return;
            }
        }
        self.base
            .render(entity, entity_yaw, partial_ticks, matrix_stack, buffer, packed_light);
    }
}

// java: the class itself is the EntityRenderer handed to registration; render
// dispatches to the extended logic.
impl<R: ItemEntityRenderer> EntityRenderer for RenderItemEntityExtended<R> {
    fn render(
        &self,
        entity: &ItemEntity,
        entity_yaw: f32,
        partial_ticks: f32,
        matrix_stack: &mut PoseStack,
        buffer: &MultiBufferSource,
        packed_light: i32,
    ) {
        RenderItemEntityExtended::render(
            self,
            entity,
            entity_yaw,
            partial_ticks,
            matrix_stack,
            buffer,
            packed_light,
        )
    }
}

// java: public static class Factory implements
// EntityRendererProvider<ItemEntity> — create() builds the extended renderer
// over the base ItemEntityRenderer from the provider context.
pub struct Factory;

impl Factory {
    // java: EntityRenderer<ItemEntity> create(Context context)
    pub fn create(&self, _context: &EntityRendererContext, base: impl ItemEntityRenderer) -> impl EntityRenderer {
        RenderItemEntityExtended::new(base)
    }
}

#[cfg(test)]
mod tests {
    use crate::config;
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    // a recording base renderer: super.render was reached or suppressed
    #[derive(Clone, Default)]
    struct Recording {
        calls: Rc<RefCell<Vec<(i32, i32, f32)>>>,
    }

    impl Recording {
        fn new() -> Recording {
            Recording::default()
        }
    }

    impl ItemEntityRenderer for Recording {
        fn render(
            &self,
            entity: &ItemEntity,
            _entity_yaw: f32,
            _partial_ticks: f32,
            _matrix_stack: &mut PoseStack,
            _buffer: &MultiBufferSource,
            _packed_light: i32,
        ) {
            self.calls
                .borrow_mut()
                .push((entity.get_age(), 0, 0.0));
        }
    }

    fn drew(rec: &Recording) -> Vec<i32> {
        rec.calls.borrow().iter().map(|(age, _, _)| *age).collect()
    }

    fn render_at(age: i32, rec: &Recording) {
        let renderer = RenderItemEntityExtended::new(rec.clone());
        let entity = ItemEntity::for_tests(age);
        renderer.render(&entity, 0.0, 0.0, &mut PoseStack, &MultiBufferSource, 0);
    }

    #[test]
    fn well_before_the_window_renders_normally() {
        let rec = Recording::new();
        // remaining 6000 - 0 = 6000 > 20 * 20: normal render
        render_at(0, &rec);
        assert_eq!(drew(&rec), vec![0]);
    }

    #[test]
    fn urgent_flash_suppresses_and_shows_alternately() {
        let rec = Recording::new();
        // remaining = 6000 - 5880 = 120 <= 400; flashFactor = max(2, 120/20) = 6;
        // 120 % 6 = 0 < 3: suppressed
        render_at(5880, &rec);
        // remaining = 6000 - 5886 = 114; flashFactor = max(2, 114/20) = 5;
        // 114 % 5 = 4, not < 2.5: rendered
        render_at(5886, &rec);
        assert_eq!(drew(&rec), vec![5886]);
    }

    #[test]
    fn urgent_flash_floor_is_two() {
        let rec = Recording::new();
        // remaining = 9 < 20: flashFactor = max(2, 9/20=0) = 2; 9 % 2 = 1 < 1? no: rendered
        render_at(5991, &rec);
        // remaining = 8: 8 % 2 = 0 < 1: suppressed
        render_at(5992, &rec);
        assert_eq!(drew(&rec), vec![5991]);
    }

    #[test]
    fn plain_flash_blinks_in_ten_tick_halves() {
        let rec = Recording::new();
        // remaining = 6000 - 5910 = 90 <= 400; urgent off; 90 % 20 = 10, not < 10: rendered
        // then remaining = 85: 85 % 20 = 5 < 10: suppressed
        config::client().set(config::ModConfig {
            urgent_flash: false,
            ..config::DEFAULTS
        });
        render_at(5910, &rec);
        render_at(5915, &rec);
        config::client().set(config::DEFAULTS);
        assert_eq!(drew(&rec), vec![5910]);
    }

    #[test]
    fn the_window_boundary_is_inclusive() {
        let rec = Recording::new();
        // remaining = 6000 - 5600 = 400 == 20 * 20: inside the window; urgent
        // flashFactor = 20; 400 % 20 = 0 < 10: suppressed
        render_at(5600, &rec);
        // remaining = 401: outside: rendered
        render_at(5599, &rec);
        assert_eq!(drew(&rec), vec![5599]);
    }

    #[test]
    fn negative_remaining_keeps_java_truncation() {
        let rec = Recording::new();
        // remaining = 6000 - 6010 = -10 <= 400: inside the window; urgent
        // flashFactor = max(2, -10/20 = 0) = 2; -10 % 2 = 0 < 1: suppressed.
        // (Java's % truncates toward zero; Rust's i32 matches.)
        render_at(6010, &rec);
        // remaining = -9; -9 % 2 = -1 < 1: suppressed too
        render_at(6009, &rec);
        // remaining = 6000 - 6120 = -120; -120 % 20 = 0 < 10 with urgent off
        config::client().set(config::ModConfig {
            urgent_flash: false,
            ..config::DEFAULTS
        });
        render_at(6120, &rec);
        config::client().set(config::DEFAULTS);
        assert!(drew(&rec).is_empty());
    }

    #[test]
    fn factory_builds_the_extended_renderer() {
        let factory = Factory;
        let rec = Recording::new();
        let renderer = factory.create(&EntityRendererContext, rec.clone());
        // the built renderer dispatches through the extended logic
        let entity = ItemEntity::for_tests(0);
        renderer.render(&entity, 0.0, 0.0, &mut PoseStack, &MultiBufferSource, 0);
        assert_eq!(drew(&rec), vec![0]);
    }
}
