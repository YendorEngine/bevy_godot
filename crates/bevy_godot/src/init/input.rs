use std::sync::mpsc::Sender;

use crate::prelude::{
    godot_prelude::{InputEvent as GodotInputEvent, *},
    *,
};

#[derive(NativeClass, Default)]
#[inherit(Node)]
pub struct InputEventWatcher {
    pub(crate) notification_channel: Option<Sender<(InputEventType, Ref<GodotInputEvent>)>>,
}

#[methods]
impl InputEventWatcher {
    fn new(_base: &Node) -> Self { Self::default() }

    #[method]
    fn _unhandled_input(&mut self, input_event: Ref<GodotInputEvent>) {
        self.notification_channel.as_ref().unwrap().send((InputEventType::Unhandled, input_event)).unwrap();
    }

    #[method]
    fn _input(&mut self, input_event: Ref<GodotInputEvent>) {
        self.notification_channel.as_ref().unwrap().send((InputEventType::Normal, input_event)).unwrap();
    }
}
