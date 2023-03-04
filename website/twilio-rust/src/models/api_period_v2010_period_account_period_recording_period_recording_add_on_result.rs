/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.39.1
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingAddOnResult {
    /// The unique string that that we created to identify the Recording AddOnResult resource.
    #[serde(rename = "sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sid: Option<Option<String>>,
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording AddOnResult resource.
    #[serde(rename = "account_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<Option<String>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::RecordingAddOnResultEnumStatus>,
    /// The SID of the Add-on to which the result belongs.
    #[serde(rename = "add_on_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub add_on_sid: Option<Option<String>>,
    /// The SID of the Add-on configuration.
    #[serde(rename = "add_on_configuration_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub add_on_configuration_sid: Option<Option<String>>,
    /// The date and time in GMT that the resource was created specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<Option<String>>,
    /// The date and time in GMT that the resource was last updated specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<Option<String>>,
    /// The date and time in GMT that the result was completed specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_completed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_completed: Option<Option<String>>,
    /// The SID of the recording to which the AddOnResult resource belongs.
    #[serde(rename = "reference_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub reference_sid: Option<Option<String>>,
    /// A list of related resources identified by their relative URIs.
    #[serde(rename = "subresource_uris", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub subresource_uris: Option<Option<serde_json::Value>>,
}

impl ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingAddOnResult {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingAddOnResult {
        ApiPeriodV2010PeriodAccountPeriodRecordingPeriodRecordingAddOnResult {
            sid: None,
            account_sid: None,
            status: None,
            add_on_sid: None,
            add_on_configuration_sid: None,
            date_created: None,
            date_updated: None,
            date_completed: None,
            reference_sid: None,
            subresource_uris: None,
        }
    }
}


