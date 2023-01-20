pub use crate::plugins::{assets::GodotResource, core::*, packed_scene::*};

pub mod bevy_prelude {
    pub use bevy::prelude::*;
}
pub use bevy;

pub mod godot_prelude {
    pub use gdnative::{api::*, prelude::*};
}
pub use bevy::{
    app::prelude::*,
    asset::{prelude::*, *},
    core::prelude::*,
    ecs::prelude::*,
    hierarchy::*,
    log::prelude::*,
    math::prelude::*,
    prelude::Transform as BevyTransform,
    reflect::{prelude::*, TypeUuid},
    time::prelude::*,
    utils::prelude::*,
};
pub use bevy_godot_derive::NodeTreeView;
pub use gdnative;
pub use godot_prelude::{
    Control, InitHandle, Node, Node2D, Object, Ref, SceneTree, Spatial, TRef, Transform as GodotTransform,
    Transform2D as GodotTransform2D, Variant, Vector2, Vector3,
};

pub use crate::{bevy_godot_init, node_tree_view::NodeTreeView, GodotPlugin};
