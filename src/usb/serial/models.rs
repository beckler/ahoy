use serde::Deserialize;

#[derive(Default, Debug, Clone, Deserialize)]
pub struct DeviceDetails {
    #[serde(skip_deserializing, skip_serializing)]
    pub manufacturer: String,
    #[serde(alias = "deviceModel")]
    pub device_model: String,
    #[serde(alias = "firmwareVersion")]
    pub firmware_version: String,
    #[serde(alias = "hardwareVersion")]
    pub hardware_version: String,
    #[serde(alias = "uId")]
    pub uid: String,
    #[serde(alias = "deviceName")]
    pub device_name: String,
    #[serde(alias = "profileId")]
    pub profile_id: String,
}
