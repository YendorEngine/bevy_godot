use bevy::prelude::App;
use gdnative::prelude::InitHandle;
use once_cell::sync::Lazy;
use parking_lot::Mutex;

mod autoload;
mod collision;
mod input;
mod scene_tree;
mod signal_watcher;

pub static BEVY_INIT_FUNC: Lazy<Mutex<Option<Box<dyn Fn(&mut App) + Send>>>> = Lazy::new(|| Mutex::new(None));

pub fn godot_init(init: &InitHandle) {
    init.add_class::<autoload::Autoload>();
    init.add_class::<input::InputEventWatcher>();
    init.add_class::<collision::CollisionWatcher>();
    init.add_class::<scene_tree::SceneTreeWatcher>();
    init.add_class::<signal_watcher::GodotSignalWatcher>();
}

#[macro_export]
macro_rules! bevy_godot_init {
    ( $init: ident, $app: ident ) => {
        fn godot_init(init: InitHandle) {
            bevy_godot::init::godot_init(&init);
            $init(&init);

            let mut init_func = bevy_godot::init::BEVY_INIT_FUNC.lock();
            if init_func.is_none() {
                *init_func = Some(Box::new($app));
            }
        }

        bevy_godot::prelude::godot_prelude::godot_init!(godot_init);
    };
}
