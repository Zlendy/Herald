use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionInfoModel {
    pub version: String,
    pub commit: String,

    #[serde(rename = "buildDate")]
    pub build_date: String,
}
