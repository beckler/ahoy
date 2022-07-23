use crate::usb::{
    observer::{self, UsbDevice},
    watcher,
};
use futures::channel::mpsc::Receiver;
use futures::StreamExt;
use iced_native::subscription::{self, Subscription};

#[derive(Debug, Clone)]
pub enum Event {
    Connect(UsbDevice),
    Disconnect(UsbDevice),
}

enum State {
    ListenerStarting,
    Listener(Receiver<observer::Event>),
}

pub fn listener() -> Subscription<Event> {
    struct BGWorker;

    subscription::unfold(
        std::any::TypeId::of::<BGWorker>(),
        State::ListenerStarting,
        |state| async move {
            match state {
                State::ListenerStarting => {
                    let subscription = watcher::subscribe();
                    (None, State::Listener(subscription))
                }
                State::Listener(mut subscription) => {
                    let event = subscription.select_next_some().await;
                    match event {
                        // when the app is first launched - this is all the initial connected devices
                        observer::Event::Initial(devices) => {
                            match devices
                                .iter()
                                .find(|device| device.is_dfu_device() || device.is_stm_device())
                            {
                                Some(device) => (
                                    Some(Event::Connect(device.clone())),
                                    State::Listener(subscription),
                                ),
                                None => (None, State::Listener(subscription)),
                            }
                        }
                        // app has already launched - but detects a new device
                        observer::Event::Connected(device) => {
                            if device.is_dfu_device() || device.is_stm_device() {
                                (
                                    Some(Event::Connect(device.clone())),
                                    State::Listener(subscription),
                                )
                            } else {
                                (None, State::Listener(subscription))
                            }
                        }
                        // app has already launched - but detects a disconnected device
                        observer::Event::Disconnected(device) => (
                            Some(Event::Disconnect(device)),
                            State::Listener(subscription),
                        ),
                    }
                }
            }
        },
    )
}
