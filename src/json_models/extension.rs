use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::collections::HashMap;
use crate::json_models::khr_xmp::{KhrXmpPacket, KhrXmp};

#[derive(Serialize, Deserialize)]
pub struct Extension {
    #[serde(rename = "KHR_xmp", skip_serializing_if = "Option::is_none")]
    pub khr_xmp: Option<KhrXmp>,

    #[serde(flatten)]
    pub other_extensions: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
pub struct PacketExtension {
    #[serde(rename = "KHR_xmp", skip_serializing_if = "Option::is_none")]
    pub khr_xmp: Option<KhrXmpPacket>,

    #[serde(flatten)]
    pub other_extensions: HashMap<String, Value>,
}

/// Struct used to represent objects where we only care about the extension property.
#[derive(Serialize, Deserialize)]
pub struct ExtensionsOnly {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<PacketExtension>,

    #[serde(flatten)]
    pub other_fields: HashMap<String, Value>,
}
