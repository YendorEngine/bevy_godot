use bevy::{app::PluginGroupBuilder, prelude::PluginGroup};

pub mod assets;
pub mod core;
pub mod packed_scene;

pub struct DefaultGodotPlugins;
impl PluginGroup for DefaultGodotPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(core::GodotCorePlugin)
            .add(assets::GodotAssetsPlugin)
            .add(packed_scene::PackedScenePlugin)
    }
}
