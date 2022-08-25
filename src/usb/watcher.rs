use async_std::task;
use futures::channel::mpsc;
use futures::channel::mpsc::Receiver;
use futures::SinkExt;
use iced_futures::futures;
use log::*;

use super::observer::{Event, Observer};

pub fn subscribe() -> Receiver<Event> {
    let (mut sender, receiver) = mpsc::channel(0);

    task::spawn(async move {
        // let subscription = Observer::new().with_poll_interval(1).subscribe();
        let observer = match Observer::new() {
            Ok(sub) => sub,
            Err(err) => {
                error!("unable to get device descriptor: {:?}", err);
                panic!("unable to start usb observer: {:?}", err)
            }
        };

        let subscription = observer.subscribe();

        for event in subscription.rx_event.iter() {
            let _ = sender.send(event).await;
        }
    });

    receiver
}
