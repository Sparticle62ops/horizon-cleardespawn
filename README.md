# ClearDespawn (Horizon Rust port)

Exact 1:1 Rust port of [ClearDespawn](https://github.com/StrikerRockers-Mods/ClearDespawn)
(upstream Java, MIT) by StrikerRockers-Mods, for the Horizon client.

Item entities blink out as they near their despawn time: inside the last
`flashStartTime` seconds the extended item entity renderer drops the item's
draw on the flash schedule, faster as it gets closer when `urgentFlash` is on.

## Modules (one per upstream class)

| Upstream | Port |
| --- | --- |
| `Constants` | `src/constants.rs` |
| `ModConfig` | `src/config.rs` |
| `IConfigHelper` / `ConfigHelper` | `src/config_helper.rs` |
| `Services` | `src/services.rs` |
| `RenderItemEntityExtended` (+ `Factory`) | `src/render_item_entity_extended.rs` |
| `ClearDespawn` (client initializer) | `src/clear_despawn.rs` |
| `ModMenuIntegration` | `src/mod_menu.rs` |

The mod API wires the original's three config entries as settings with
upstream's own autoconfig lang keys as the labels, and `client_setup` syncs
them into the live config cell the `ConfigHelper` reads. The engine-bound
entity renderer registration sits in `src/seams.rs` until the mod API carries
a surface for it.

## Licence

Upstream licence: MIT. Copyright (c) 2019, 2020, 2021, 2022 StrikerRocker;
Copyright (c) 2020 Guntram Blohm. Carried verbatim in `LICENSE`; the Rust
translation is distributed under the same licence.

## Source

https://github.com/Sparticle62ops/horizon-cleardespawn
