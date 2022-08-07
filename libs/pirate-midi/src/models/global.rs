use serde::{Deserialize, Serialize};

use crate::{AuxMessages, MessageStack, Outputs};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum Division {
    #[serde(rename = "1/4")]
    Quarter,
    #[serde(rename = "1/4T")]
    QuarterTriplet,
    #[serde(rename = "1/4.")]
    DottedQuarter,
    #[serde(rename = "1/8")]
    Eight,
    #[serde(rename = "1/8T")]
    EightTriplet,
    #[serde(rename = "1/8.")]
    DottedEight,
    #[serde(rename = "1/16")]
    Sixteenth,
    #[serde(rename = "1/16T")]
    SixteenthTriple,
    #[serde(rename = "1/16.")]
    DottedSixteenth,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MIDIClock {
    pub tempo: Option<f32>,
    pub state: Option<bool>,
    pub division: Option<Division>,
    pub outputs: Outputs,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UIMode {
    Simple,
    Standard,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum FlexiportMode {
    Unassigned,
    #[serde(rename = "midiOutTypeA")]
    MIDIOutTypeA,
    #[serde(rename = "midiOutTypeB")]
    MIDIOutTypeB,
    #[serde(rename = "midiOutTip")]
    MIDIOutTip,
    #[serde(rename = "midiOutRing")]
    MIDIOutRing,
    DeviceLink,
    #[serde(rename = "expSingle")]
    ExppressionSingle,
    #[serde(rename = "expDouble")]
    ExpressionDual,
    SwitchIn,
    SwitchOut,
    TapTempo,
    RelayOut,
    PulseOut,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GlobalSettings {
    pub device_name: String,
    pub profile_id: serde_bytes::ByteBuf,
    pub current_bank: i32,
    pub midi_channel: i32,
    pub led_brightness: i32,
    pub flexi1_mode: FlexiportMode,
    pub flexi2_mode: FlexiportMode,
    pub flexi1_clock: Option<i32>,
    pub flexi2_clock: Option<i32>,
    pub ui_mode: UIMode,
    pub preserve_states: bool,
    pub send_states: bool,
    pub bank_template_index: i32,
    pub custom_led_colours: [serde_bytes::ByteBuf; 12],
    pub midi_clocks: Vec<MIDIClock>,
    pub flexi1_thru_handles: Outputs,
    pub flexi2_thru_handles: Outputs,
    pub midi0_thru_handles: Outputs,
    pub usb_thru_handles: Outputs,
    pub exp_messages: Vec<MessageStack>,
    pub aux_messages: Vec<AuxMessages>,
}

#[cfg(test)]
mod tests {
    use std::{env, path::PathBuf};

    use crate::models::global::GlobalSettings;

    #[test]
    fn test_parsing_global_messages() {
        match env::var("CARGO_MANIFEST_DIR") {
            Ok(path) => {
                let test_file_path = PathBuf::from(format!(
                    "{path}/resources/test/global-settings-example1.json"
                ));
                println!("manifest: {}", test_file_path.display());
                let contents =
                    std::fs::read_to_string(&test_file_path).expect("unable to read file");
                let response: Result<GlobalSettings, serde_json::Error> =
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
