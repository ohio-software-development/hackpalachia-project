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
pub enum SiprecEnumStatus {
    #[serde(rename = "in-progress")]
    InProgress,
    #[serde(rename = "stopped")]
    Stopped,

}

impl ToString for SiprecEnumStatus {
    fn to_string(&self) -> String {
        match self {
            Self::InProgress => String::from("in-progress"),
            Self::Stopped => String::from("stopped"),
        }
    }
}

impl Default for SiprecEnumStatus {
    fn default() -> SiprecEnumStatus {
        Self::InProgress
    }
}




