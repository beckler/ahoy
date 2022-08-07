use serde::Deserialize;

#[derive(Default, Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CheckResponse {
    #[serde(alias = "uId")]
    pub uid: String,
    pub device_model: String,
    pub firmware_version: String,
    pub hardware_version: String,
    pub device_name: String,
    pub profile_id: String,
}

// {"deviceModel":"Bridge6","firmwareVersion":"1.1.0","hardwareVersion":"1.0.1","uId":"4d00404d00444d0048","deviceName":"Bridge 6","profileId":"0"}
