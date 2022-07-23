use core::panic;
use log::*;
use rusb::{Context, Device, DeviceDescriptor, HotplugBuilder, UsbContext};
use std::{collections::HashSet, hash::Hash, thread, time::Duration};

use crossbeam_channel::{bounded, unbounded, Receiver, Sender};

use crate::{USB_PRODUCT_DFU_ID, USB_PRODUCT_ID, USB_TIMEOUT, USB_VENDOR_ID};

// HOT PLUG HANDLER

struct HotPlugHandler<T: UsbContext> {
    sender: Sender<HotPlugEvent<T>>,
}

enum HotPlugEvent<T: UsbContext> {
    Arrived(Device<T>),
    Left(Device<T>),
}

impl<T: UsbContext> rusb::Hotplug<T> for HotPlugHandler<T> {
    fn device_arrived(&mut self, device: Device<T>) {
        match self.sender.send(HotPlugEvent::Arrived(device)) {
            Ok(_) => (),
            Err(err) => error!("unable to send: {:?}", err),
        }
    }

    fn device_left(&mut self, device: Device<T>) {
        match self.sender.send(HotPlugEvent::Left(device)) {
            Ok(_) => (),
            Err(err) => error!("unable to send: {:?}", err),
        }
    }
}

impl<T: UsbContext> Drop for HotPlugHandler<T> {
    fn drop(&mut self) {
        warn!("hotplug handler dropped");
    }
}

// USB DEVICE

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct UsbDevice {
    pub raw_device: Option<Device<Context>>,
    pub vendor_id: u16,
    pub product_id: u16,
}

impl Hash for UsbDevice {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.vendor_id.hash(state);
        self.product_id.hash(state);
    }
}

impl UsbDevice {
    pub fn new(device: Device<Context>, device_desc: &DeviceDescriptor) -> UsbDevice {
        UsbDevice {
            raw_device: Some(device),
            vendor_id: device_desc.vendor_id(),
            product_id: device_desc.product_id(),
        }
    }

    pub fn is_stm_device(&self) -> bool {
        self.vendor_id == USB_VENDOR_ID && self.product_id == USB_PRODUCT_ID
    }

    pub fn is_dfu_device(&self) -> bool {
        self.vendor_id == USB_VENDOR_ID && self.product_id == USB_PRODUCT_DFU_ID
    }
}

#[derive(Clone)]
pub struct Subscription {
    pub rx_event: Receiver<Event>,
    // When this gets dropped, the channel will become disconnected and the
    // background thread will close
    _tx_close: Sender<()>,
}

pub enum Event {
    /// Initial list of devices when polling starts
    Initial(Vec<UsbDevice>),
    /// A device that has just been connected
    Connected(UsbDevice),
    /// A device that has just disconnected
    Disconnected(UsbDevice),
}

#[derive(Debug, Clone)]
pub struct Observer {
    context: Context,
    tx_event: Sender<Event>,
    rx_event: Receiver<Event>,
    poll_interval: u32,
}

impl Observer {
    pub fn new() -> rusb::Result<Observer> {
        let (tx_event, rx_event) = unbounded();
        Ok(Observer {
            context: Context::new()?,
            tx_event,
            rx_event,
            poll_interval: 1,
        })
    }

    /// runs a quick sweep to determine all connected devices
    pub fn fetch(&mut self) -> Vec<UsbDevice> {
        // iterate over the devices
        match self.context.devices() {
            Ok(devices) => devices.iter().fold(vec![], |mut acc, device| {
                let desc = match device.device_descriptor() {
                    Ok(d) => d,
                    Err(err) => {
                        error!("unable to get device descriptor: {}", err);
                        panic!("unable to continue");
                    }
                };
                acc.push(UsbDevice::new(device, &desc));
                acc
            }),
            Err(err) => {
                error!("failed to enumerate devices: {}", err);
                vec![]
            }
        }
    }

    /// starts the usb monitor
    pub fn subscribe(&self) -> Subscription {
        let (tx_close, rx_close) = bounded::<()>(0);

        // if we have hotplug functionality, use it.
        // otherwise, backup to just compairing what devices are changed.
        if rusb::has_hotplug() {
            info!("hotplug functionality detected");

            thread::Builder::new()
                .name("USB Hotplug Listener Thread".to_string())
                .spawn({
                    // create copy for this thread
                    let mut this = self.clone();

                    move || {
                        // create our inner channel
                        let (tx_event, rx_event) = unbounded::<HotPlugEvent<Context>>();

                        // register the hotplug handler
                        let mut _registration = match HotplugBuilder::new()
                            .enumerate(false)
                            .register(&this.context, Box::new(HotPlugHandler { sender: tx_event }))
                        {
                            Ok(reg) => Some(reg),
                            Err(err) => {
                                warn!("unable to get hotplug registation: {:?}", err);
                                None
                            }
                        };
                        // get initial devices
                        let device_list = &this.fetch();
                        // send initially connected devices
                        if this
                            .tx_event
                            .send(Event::Initial(device_list.clone()))
                            .is_err()
                        {
                            return;
                        }

                        // listen for new devices
                        loop {
                            // Check whether the subscription has been disposed
                            if let Err(crossbeam_channel::RecvTimeoutError::Disconnected) =
                                rx_close.recv_timeout(Duration::from_millis(250))
                            {
                                return;
                            }

                            // handle events
                            match rx_event.recv().unwrap() {
                                HotPlugEvent::Arrived(device) => {
                                    let desc = device.device_descriptor().unwrap();
                                    info!("connected: {:?}", device);
                                    if this
                                        .tx_event
                                        .send(Event::Connected(UsbDevice::new(device, &desc)))
                                        .is_err()
                                    {
                                        return;
                                    }
                                }
                                HotPlugEvent::Left(device) => {
                                    let desc = device.device_descriptor().unwrap();
                                    info!("disconnected: {:?}", device);
                                    if this
                                        .tx_event
                                        .send(Event::Disconnected(UsbDevice::new(device, &desc)))
                                        .is_err()
                                    {
                                        return;
                                    }
                                }
                            };
                        }
                    }
                })
                .expect("Could not spawn background thread");

            // start the hotplug event handler in it's own thread
            thread::Builder::new()
                .name("USB Hotplug Listener Context Thread".to_string())
                .spawn({
                    // create copy for this thread
                    let this = self.clone();
                    move || loop {
                        this.context.handle_events(None).unwrap();
                    }
                })
                .expect("Could not spawn background thread");
        } else {
            info!("hotplug functionality not detected: using backup method");
            thread::Builder::new()
                .name("USB Enumeration Thread".to_string())
                .spawn({
                    // create copy for this thread
                    let mut this = self.clone();
                    move || {
                        let device_list = &this.fetch();
                        // send initially connected devices
                        if this
                            .tx_event
                            .send(Event::Initial(device_list.clone()))
                            .is_err()
                        {
                            return;
                        }

                        // get initial device list into hashset
                        let mut device_list: HashSet<UsbDevice> =
                            device_list.into_iter().cloned().collect();
                        let mut wait_seconds = this.clone().poll_interval as f32;

                        loop {
                            while wait_seconds > 0.0 {
                                // Check whether the subscription has been disposed
                                if let Err(crossbeam_channel::RecvTimeoutError::Disconnected) =
                                    rx_close.recv_timeout(USB_TIMEOUT)
                                {
                                    return;
                                }

                                wait_seconds -= 0.25;
                            }

                            wait_seconds = this.poll_interval as f32;

                            let next_devices: HashSet<UsbDevice> =
                                this.fetch().into_iter().collect();

                            // Send Disconnect for missing devices
                            for device in &device_list {
                                if !next_devices.contains(&device)
                                    && this
                                        .tx_event
                                        .send(Event::Disconnected(device.clone()))
                                        .is_err()
                                {
                                    return;
                                }
                            }

                            // Send Connect for new devices
                            for device in &next_devices {
                                if !device_list.contains(&device)
                                    && this
                                        .tx_event
                                        .send(Event::Connected(device.clone()))
                                        .is_err()
                                {
                                    return;
                                }
                            }

                            device_list = next_devices;
                        }
                    }
                })
                .expect("Could not spawn background thread");
        }

        Subscription {
            rx_event: self.rx_event.clone(),
            _tx_close: tx_close,
        }
    }
}

impl Drop for Observer {
    fn drop(&mut self) {
        warn!("observer dropped");
    }
}
