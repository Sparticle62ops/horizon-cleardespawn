//! Host seams: every Minecraft client type the port touches sits here, as a
//! thin no-op-default surface the engine binds. Signatures mirror the upstream
//! Java calls one to one.

use std::sync::{Mutex, OnceLock};

// ---------------------------------------------------------------- logging

pub struct Logger;

impl Logger {
    pub fn debug(&self, msg: &str) {
        let _ = msg;
    }
}

pub fn logger() -> &'static Logger {
    static LOGGER: OnceLock<Logger> = OnceLock::new();
    LOGGER.get_or_init(|| Logger)
}

// ------------------------------------------------------------------ entity

/// java: net.minecraft.world.entity.EntityType, reduced to the constant the
/// port registers a renderer for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    Item,
}

/// java: net.minecraft.world.entity.item.ItemEntity, reduced to the reads
/// RenderItemEntityExtended performs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemEntity {
    age: i32,
}

impl ItemEntity {
    // java: entity.getAge()
    pub fn get_age(&self) -> i32 {
        self.age
    }

    // test construction of the seam surface
    pub fn for_tests(age: i32) -> ItemEntity {
        ItemEntity { age }
    }
}

// ----------------------------------------------------------------- render

// java: com.mojang.blaze3d.vertex.PoseStack
pub struct PoseStack;

// java: net.minecraft.client.renderer.MultiBufferSource
pub struct MultiBufferSource;

// java: net.minecraft.client.renderer.entity.ItemEntityRenderer — the base
// renderer RenderItemEntityExtended extends and calls into (super.render).
pub trait ItemEntityRenderer {
    fn render(
        &self,
        entity: &ItemEntity,
        entity_yaw: f32,
        partial_ticks: f32,
        matrix_stack: &mut PoseStack,
        buffer: &MultiBufferSource,
        packed_light: i32,
    );
}

// java: net.minecraft.client.renderer.entity.EntityRendererProvider.Context
pub struct EntityRendererContext;

// java: net.minecraft.client.renderer.entity.EntityRenderer — the extended
// renderer the Factory hands back, as the registration stores it.
pub trait EntityRenderer {
    fn render(
        &self,
        entity: &ItemEntity,
        entity_yaw: f32,
        partial_ticks: f32,
        matrix_stack: &mut PoseStack,
        buffer: &MultiBufferSource,
        packed_light: i32,
    );
}

// java: EntityRendererProvider.Context — captured at create() time.
pub trait RendererContext {}

// ------------------------------------------------- renderer registration

// java: net.fabricmc.fabric.api.client.rendering.v1.EntityRendererRegistry
// .register(EntityType.ITEM, factory) — the engine-bound seam. Until the mod
// API carries a surface for registering a custom renderer for an existing
// entity type, the registration is recorded here (see the mod API gap filed on
// horizon-rust/horizon).
struct Registration {
    entity_type: EntityType,
    factory: &'static str,
}

fn registrations() -> &'static Mutex<Vec<Registration>> {
    static REGISTRATIONS: OnceLock<Mutex<Vec<Registration>>> = OnceLock::new();
    REGISTRATIONS.get_or_init(|| Mutex::new(Vec::new()))
}

pub fn register_entity_renderer(entity_type: EntityType, factory: &'static str) {
    registrations().lock().unwrap().push(Registration {
        entity_type,
        factory,
    });
}

pub fn registered_for_tests() -> Vec<(EntityType, &'static str)> {
    registrations()
        .lock()
        .unwrap()
        .iter()
        .map(|r| (r.entity_type, r.factory))
        .collect()
}
