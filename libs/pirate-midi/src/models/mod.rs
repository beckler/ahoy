pub mod bank;
pub mod check;
pub mod global;

use self::{bank::BankSettings, check::CheckResponse};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/* SHARED STRUCTS */

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Outputs {
    #[serde(rename = "midi0")]
    pub midi: bool,
    #[serde(rename = "flexi1")]
    pub flexi_1: bool,
    #[serde(rename = "flexi2")]
    pub flext_2: bool,
    #[serde(rename = "usb")]
    pub usb: bool,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub sweep: Option<String>,
    pub min_limit: Option<i32>,
    pub max_limit: Option<i32>,
    pub status_byte: Option<serde_bytes::ByteBuf>,
    #[serde(rename = "dataByte1")]
    pub data_byte1: Option<serde_bytes::ByteBuf>,
    #[serde(rename = "dataByte2")]
    pub data_byte2: Option<serde_bytes::ByteBuf>,
    pub outputs: Outputs,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MessageStack {
    #[serde(rename = "numMessages")]
    pub count: i32,
    pub messages: Vec<Message>,
}

/* AUX MESSAGES */
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuxMessageStack {
    pub press_messages: MessageStack,
    pub hold_messages: MessageStack,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuxMessages {
    pub tip: AuxMessageStack,
    pub ring: AuxMessageStack,
    pub tip_ring: AuxMessageStack,
}

/* RESPONSES */

#[derive(Debug)]
pub enum DataRequestResponse {
    BankSettings(BankSettings),
    GlobalSettings(String),
}

#[derive(Debug)]
pub enum Response {
    Check(CheckResponse),
    Control(Result<()>),
    DataRequest(DataRequestResponse),
    DataTransmit(String),
    Reset(String),
}
