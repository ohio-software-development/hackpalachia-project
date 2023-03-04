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
pub enum AccountEnumStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "closed")]
    Closed,

}

impl ToString for AccountEnumStatus {
    fn to_string(&self) -> String {
        match self {
            Self::Active => String::from("active"),
            Self::Suspended => String::from("suspended"),
            Self::Closed => String::from("closed"),
        }
    }
}

impl Default for AccountEnumStatus {
    fn default() -> AccountEnumStatus {
        Self::Active
    }
}




