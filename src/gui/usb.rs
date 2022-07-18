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
                        observer::Event::Initial(devices) => {
                            match devices
                                .iter()
                                .find(|device| device.try_get_serial_port().is_some())
                            {
                                Some(device) => (
                                    Some(Event::Connect(device.clone())),
                                    State::Listener(subscription),
                                ),
                                None => (None, State::Listener(subscription)),
                            }
                        }
                        observer::Event::Connected(device) => match &device.try_get_serial_port() {
                            Some(_) => (
                                Some(Event::Connect(device.clone())),
                                State::Listener(subscription),
                            ),
                            None => (None, State::Listener(subscription)),
                        },
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
