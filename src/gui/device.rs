use crate::{command::device::try_get_serial_port, usb::watcher};
use futures::channel::mpsc::Receiver;
use futures::StreamExt;
use iced_native::subscription::{self, Subscription};
use usb_enumeration::{Event as UsbEvent, UsbDevice};

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
                        UsbEvent::Initial(devices) => {
                            match devices
                                .iter()
                                .find(|device| try_get_serial_port(device).is_some())
                            {
                                Some(device) => (
                                    Some(Event::Connect(device.clone())),
                                    State::Listener(subscription),
                                ),
                                None => (None, State::Listener(subscription)),
                            }
                        }
                        UsbEvent::Connect(device) => match try_get_serial_port(&device) {
                            Some(_) => (
                                Some(Event::Connect(device.clone())),
                                State::Listener(subscription),
                            ),
                            None => (None, State::Listener(subscription)),
                        },
                        UsbEvent::Disconnect(device) => (
                            Some(Event::Disconnect(device)),
                            State::Listener(subscription),
                        ),
                    }
                }
            }
        },
    )
}

#[derive(Debug, Clone)]
pub enum Event {
    Connect(UsbDevice),
    Disconnect(UsbDevice),
}

enum State {
    ListenerStarting,
    Listener(Receiver<usb_enumeration::Event>),
}
