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
pub struct ApiPeriodV2010PeriodAccountPeriodMessagePeriodMedia {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created this Media resource.
    #[serde(rename = "account_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<Option<String>>,
    /// The default [mime-type](https://en.wikipedia.org/wiki/Internet_media_type) of the media, for example `image/jpeg`, `image/png`, or `image/gif`
    #[serde(rename = "content_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<Option<String>>,
    /// The date and time in GMT that this resource was created specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<Option<String>>,
    /// The date and time in GMT that this resource was last updated, specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<Option<String>>,
    /// The SID of the resource that created the media.
    #[serde(rename = "parent_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent_sid: Option<Option<String>>,
    /// The unique string that that we created to identify this Media resource.
    #[serde(rename = "sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sid: Option<Option<String>>,
    /// The URI of this resource, relative to `https://api.twilio.com`.
    #[serde(rename = "uri", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub uri: Option<Option<String>>,
}

impl ApiPeriodV2010PeriodAccountPeriodMessagePeriodMedia {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodMessagePeriodMedia {
        ApiPeriodV2010PeriodAccountPeriodMessagePeriodMedia {
            account_sid: None,
            content_type: None,
            date_created: None,
            date_updated: None,
            parent_sid: None,
            sid: None,
            uri: None,
        }
    }
}


