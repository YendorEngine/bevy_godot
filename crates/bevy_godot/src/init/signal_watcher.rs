use std::sync::mpsc::Sender;

#[doc(hidden)]
use crate::prelude::{bevy_prelude::trace, godot_prelude::*, *};

#[derive(NativeClass, Default)]
#[inherit(Node)]
pub struct GodotSignalWatcher {
    pub notification_channel: Option<Sender<GodotSignal>>,
}

#[methods]
impl GodotSignalWatcher {
    fn new(_base: &Node) -> Self { Self::default() }

    #[allow(clippy::too_many_arguments)]
    #[method]
    fn event(
        &self,
        #[base] base: TRef<Node>,
        #[opt] arg_1: Option<Variant>,
        #[opt] arg_2: Option<Variant>,
        #[opt] arg_3: Option<Variant>,
        #[opt] arg_4: Option<Variant>,
        #[opt] arg_5: Option<Variant>,
        #[opt] arg_6: Option<Variant>,
        #[opt] arg_7: Option<Variant>,
        #[opt] arg_8: Option<Variant>,
        #[opt] arg_9: Option<Variant>,
    ) {
        let args = vec![
            arg_1, arg_2, arg_3, arg_4, arg_5, arg_6, arg_7, arg_8, arg_9,
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

        let signal_args =
            args.iter().take_while(|arg| **arg != Variant::new(base)).cloned().collect::<Vec<_>>();
        let origin = args[signal_args.len() + 1].clone();
        let signal_name = args[signal_args.len() + 2].clone();

        let signal_event = GodotSignal::new(
            signal_name.try_to::<String>().unwrap(),
            unsafe { origin.try_to_object::<Object>().unwrap().assume_safe() },
            signal_args,
        );

        trace!(target: "godot_signal", signal = ?signal_event);

        self.notification_channel.as_ref().unwrap().send(signal_event).unwrap();
    }
}
