/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.39.1
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UsageTriggerEnumTriggerField {
    #[serde(rename = "count")]
    Count,
    #[serde(rename = "usage")]
    Usage,
    #[serde(rename = "price")]
    Price,

}

impl ToString for UsageTriggerEnumTriggerField {
    fn to_string(&self) -> String {
        match self {
            Self::Count => String::from("count"),
            Self::Usage => String::from("usage"),
            Self::Price => String::from("price"),
        }
    }
}

impl Default for UsageTriggerEnumTriggerField {
    fn default() -> UsageTriggerEnumTriggerField {
        Self::Count
    }
}




