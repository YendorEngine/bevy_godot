use std::sync::mpsc::Sender;

use crate::prelude::{godot_prelude::*, *};

#[derive(NativeClass, Default)]
#[inherit(Node)]
pub struct SceneTreeWatcher {
    pub(crate) notification_channel: Option<Sender<SceneTreeEvent>>,
}

#[methods]
impl SceneTreeWatcher {
    fn new(_base: &Node) -> Self { Self::default() }

    #[method]
    fn scene_tree_event(&self, node: Ref<Node>, event_type: SceneTreeEventType) {
        self.notification_channel
            .as_ref()
            .unwrap()
            .send(SceneTreeEvent {
                node: unsafe { ErasedGodotRef::from_instance_id(node.assume_safe().get_instance_id()) },
                event_type,
            })
            .unwrap();
    }
}
