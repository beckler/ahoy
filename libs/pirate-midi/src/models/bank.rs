use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use serde_with::serde_as;

use crate::{AuxMessages, Message, MessageStack};

/* SWITCHGROUPS */

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BroadcastMode {
    #[serde(rename = "txRx")]
    TxRx,
    #[serde(rename = "tx")]
    Tx,
    #[serde(rename = "rx")]
    Rx,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum RespondTo {
    OnOff,
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ResponseType {
    Or,
    And,
    Toggle,
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SwitchGroup {
    pub switch: i32,
    pub is_primary: bool,
    pub broadcast_mode: String,
    pub respond_to: RespondTo,
    pub response_type: ResponseType,
}

/* FOOTSWITCHES */

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PrimaryMode {
    Toggle,
    Momentary,
    TapTempo,
    Sequential,
    SequentialLinked,
    Scrolling,
    ScrollingLinked,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SecondaryMode {
    DoublePressToggle,
    HoldToggle,
    DoublePressMomentary,
    HoldMomentary,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum LEDMode {
    OnOff,
    Dim,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PatternDirection {
    Forward,
    Reverse,
    Pendulum,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RepeatMode {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "last2")]
    LastTwo,
    #[serde(rename = "last3")]
    LastThree,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SendMode {
    Always,
    Primary,
    Secondary,
}

#[derive(Deserialize_enum_str, Serialize_enum_str, Debug, Clone, PartialEq)]
pub enum Frequency {
    #[serde(rename = "1/4")]
    SyncQuarter,
    #[serde(rename = "1/4T")]
    SyncQuarterTriplet,
    #[serde(rename = "1/4.")]
    SyncDottedQuarter,
    #[serde(rename = "1/8")]
    SyncEight,
    #[serde(rename = "1/8T")]
    SyncEightTriplet,
    #[serde(rename = "1/8.")]
    SyncDottedEight,
    #[serde(rename = "1/16")]
    SyncSixteenth,
    #[serde(rename = "1/16T")]
    SyncSixteenthTriple,
    #[serde(rename = "1/16.")]
    SyncDottedSixteenth,
    #[serde(other)]
    Free(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Trigger {
    #[serde(rename = "null")]
    None,
    Primary,
    Secondary,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Waveform {
    Sine,
    Triangle,
    Saw,
    Ramp,
    Square,
    Random,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum FootswitchType {
    ToggleOn,
    ToggleOff,
    Press,
    Release,
    DoublePress,
    Hold,
    HoldRelease,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LowFrequencyOscillator {
    pub state: Option<bool>,
    pub frequency: Frequency,
    pub min_limit: i32,
    pub max_limit: i32,
    pub trigger: Trigger,
    pub waveform: Waveform,
    pub resolution: i32, //TODO: make this an enum that formats to an int
    pub reset_on_stop: bool,
    pub clock: Option<i32>,
    pub messages: Option<FootswitchType>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SequentialMessageStack {
    pub label: String,
    pub color: serde_bytes::ByteBuf,
    #[serde(rename = "numMessages")]
    pub count: i32,
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SequentialMessages {
    #[serde(rename = "numSteps")]
    pub count: i32,
    pub steps: Vec<SequentialMessageStack>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScrollingMessages {
    pub step_interval: i32,
    pub min_scroll_limit: i32,
    pub max_scroll_limit: i32,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Footswitch {
    pub name: String,
    pub primary_state: bool,
    pub secondary_state: bool,
    pub primary_mode: PrimaryMode,
    pub secondary_mode: SecondaryMode,
    pub primary_color: serde_bytes::ByteBuf,
    pub secondary_color: serde_bytes::ByteBuf,
    pub primary_led_mode: LEDMode,
    pub secondary_led_mode: LEDMode,
    pub sequential_pattern: PatternDirection,
    pub sequential_repeat: RepeatMode,
    pub sequential_send_mode: SendMode,
    pub linked_switch: i32,
    pub current_step: i32,
    pub midi_clock: Option<i32>,
    pub lfo: Option<LowFrequencyOscillator>,
    pub sequential_messages: Option<SequentialMessages>,
    pub scrolling_messages: Option<ScrollingMessages>,

    // footswitch message stacks
    pub toggle_on_messages: MessageStack,
    pub toggle_off_messages: MessageStack,
    pub press_messages: MessageStack,
    pub release_messages: MessageStack,
    pub double_press_messages: MessageStack,
    pub hold_messages: MessageStack,
    pub hold_release_messages: MessageStack,
    pub secondary_toggle_on_messages: MessageStack,
    pub secondary_toggle_off_messages: MessageStack,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BankSettings {
    pub id: String,
    pub name: String,
    pub bank_messages: MessageStack,
    pub exp_messages: Vec<MessageStack>,
    pub aux_messages: Vec<AuxMessages>,
    pub switch_groups: Vec<Vec<SwitchGroup>>,
    pub footswitches: Vec<Footswitch>,
}

#[cfg(test)]
mod tests {
    use std::{env, path::PathBuf};

    use crate::models::bank::BankSettings;

    #[test]
    fn test_parsing_bank_messages() {
        match env::var("CARGO_MANIFEST_DIR") {
            Ok(path) => {
                let test_file_path =
                    PathBuf::from(format!("{path}/resources/test/bank-settings-example1.json"));
                println!("manifest: {}", test_file_path.display());
                let contents =
                    std::fs::read_to_string(&test_file_path).expect("unable to read file");
                let response: Result<BankSettings, serde_json::Error> =
                    serde_json::from_str(&contents);
                match response {
                    // if we didn't panic, we passed
                    Ok(result) => println!("{:?}", result),
                    Err(e) => panic!("parsing of response failed! - {:?}", e),
                };
            }
            Err(err) => panic!("unable to load CARGO_MANIFEST_DIR: {:?}", err),
        }
    }
}
