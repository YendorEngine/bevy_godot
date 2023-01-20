use std::sync::mpsc::channel;

use crate::{
    prelude::{godot_prelude::*, *},
    CollisionWatcher, GodotSignalWatcher, InputEventWatcher, SceneTreeWatcher, BEVY_INIT_FUNC,
};

#[doc(hidden)]
#[derive(NativeClass, Default)]
#[inherit(Node)]
pub struct Autoload {
    app: Option<App>,
}

#[methods]
impl Autoload {
    fn new(_base: &Node) -> Self { Self::default() }

    #[method]
    fn _ready(&mut self, #[base] base: &Node) {
        let mut app = App::new();
        app.add_plugin(GodotPlugin);

        (BEVY_INIT_FUNC.lock().as_mut().unwrap())(&mut app);

        // SceneTree
        {
            let (sender, reciever) = channel();
            let scene_tree_watcher = SceneTreeWatcher::new_instance();
            scene_tree_watcher.map_mut(|script, _base| script.notification_channel = Some(sender)).unwrap();
            scene_tree_watcher.base().set_name("SceneTreeWatcher");

            base.add_child(scene_tree_watcher.into_base().into_shared(), true);

            app.insert_non_send_resource(SceneTreeEventReader(reciever));
        }

        // Collision
        {
            let (sender, reciever) = channel();
            let collision_watcher = CollisionWatcher::new_instance();
            collision_watcher.map_mut(|script, _base| script.notification_channel = Some(sender)).unwrap();
            collision_watcher.base().set_name("CollisionWatcher");

            base.add_child(collision_watcher.into_base().into_shared(), true);

            app.insert_non_send_resource(CollisionEventReader(reciever));
        }

        // Signals
        {
            let (sender, reciever) = channel();
            let signal_watcher = GodotSignalWatcher::new_instance();
            signal_watcher.map_mut(|script, _base| script.notification_channel = Some(sender)).unwrap();
            signal_watcher.base().set_name("GodotSignalWatcher");

            base.add_child(signal_watcher.into_base().into_shared(), true);

            app.insert_non_send_resource(GodotSignalReader(reciever));
        }

        // Input
        {
            let (sender, reciever) = channel();
            let input_event_watcher = InputEventWatcher::new_instance();
            input_event_watcher.map_mut(|script, _base| script.notification_channel = Some(sender)).unwrap();
            input_event_watcher.base().set_name("InputEventWatcher");

            base.add_child(input_event_watcher.into_base().into_shared(), true);

            app.insert_non_send_resource(InputEventReader(reciever));
        }

        self.app = Some(app);

        // disable pausing on the autoload to allow bevy updates when paused
        base.set_pause_mode(Node::PAUSE_MODE_PROCESS);
    }

    #[method]
    fn _process(&mut self, _delta: f32) {
        use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};

        if let Some(app) = self.app.as_mut() {
            app.insert_resource(GodotVisualFrame);

            if let Err(e) = catch_unwind(AssertUnwindSafe(|| app.update())) {
                self.app = None;

                eprintln!("bevy app update panicked");
                resume_unwind(e);
            }

            app.world.remove_resource::<GodotVisualFrame>();
        }
    }

    #[method]
    fn _physics_process(&mut self, _delta: f32) {
        use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};

        if let Some(app) = self.app.as_mut() {
            app.insert_resource(GodotPhysicsFrame);

            if let Err(e) = catch_unwind(AssertUnwindSafe(|| app.update())) {
                self.app = None;

                eprintln!("bevy app update panicked");
                resume_unwind(e);
            }

            app.world.remove_resource::<GodotPhysicsFrame>();
        }
    }
}
