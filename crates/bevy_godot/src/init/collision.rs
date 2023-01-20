use std::sync::mpsc::Sender;

use crate::prelude::{godot_prelude::*, *};

#[derive(NativeClass, Default)]
#[inherit(Node)]
pub struct CollisionWatcher {
    pub(crate) notification_channel: Option<Sender<CollisionEvent>>,
}

#[methods]
impl CollisionWatcher {
    fn new(_base: &Node) -> Self { Self::default() }

    #[method]
    fn collision_event(&self, target: Ref<Node>, origin: Ref<Node>, event_type: CollisionEventType) {
        let (origin, target) = unsafe { (origin.assume_safe(), target.assume_safe()) };
        self.notification_channel
            .as_ref()
            .unwrap()
            .send(CollisionEvent {
                event_type,
                origin: origin.get_instance_id(),
                target: target.get_instance_id(),
            })
            .unwrap();
    }
}
