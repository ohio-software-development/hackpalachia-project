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
pub struct ApiPeriodV2010PeriodAccountPeriodCallPeriodCallNotification {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Call Notification resource.
    #[serde(rename = "account_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<Option<String>>,
    /// The API version used to create the Call Notification resource.
    #[serde(rename = "api_version", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<Option<String>>,
    /// The SID of the [Call](https://www.twilio.com/docs/voice/api/call-resource) the Call Notification resource is associated with.
    #[serde(rename = "call_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub call_sid: Option<Option<String>>,
    /// The date and time in GMT that the resource was created specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<Option<String>>,
    /// The date and time in GMT that the resource was last updated specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<Option<String>>,
    /// A unique error code for the error condition that is described in our [Error Dictionary](https://www.twilio.com/docs/api/errors).
    #[serde(rename = "error_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<Option<String>>,
    /// An integer log level that corresponds to the type of notification: `0` is ERROR, `1` is WARNING.
    #[serde(rename = "log", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub log: Option<Option<String>>,
    /// The date the notification was actually generated in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format. Message buffering can cause this value to differ from `date_created`.
    #[serde(rename = "message_date", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub message_date: Option<Option<String>>,
    /// The text of the notification.
    #[serde(rename = "message_text", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub message_text: Option<Option<String>>,
    /// The URL for more information about the error condition. This value is a page in our [Error Dictionary](https://www.twilio.com/docs/api/errors).
    #[serde(rename = "more_info", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub more_info: Option<Option<String>>,
    /// The HTTP method used to generate the notification. If the notification was generated during a phone call, this is the HTTP Method used to request the resource on your server. If the notification was generated by your use of our REST API, this is the HTTP method used to call the resource on our servers.
    #[serde(rename = "request_method", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub request_method: Option<Option<RequestMethod>>,
    /// The URL of the resource that generated the notification. If the notification was generated during a phone call, this is the URL of the resource on your server that caused the notification. If the notification was generated by your use of our REST API, this is the URL of the resource you called.
    #[serde(rename = "request_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub request_url: Option<Option<String>>,
    /// The unique string that that we created to identify the Call Notification resource.
    #[serde(rename = "sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sid: Option<Option<String>>,
    /// The URI of the resource, relative to `https://api.twilio.com`.
    #[serde(rename = "uri", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub uri: Option<Option<String>>,
}

impl ApiPeriodV2010PeriodAccountPeriodCallPeriodCallNotification {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodCallPeriodCallNotification {
        ApiPeriodV2010PeriodAccountPeriodCallPeriodCallNotification {
            account_sid: None,
            api_version: None,
            call_sid: None,
            date_created: None,
            date_updated: None,
            error_code: None,
            log: None,
            message_date: None,
            message_text: None,
            more_info: None,
            request_method: None,
            request_url: None,
            sid: None,
            uri: None,
        }
    }
}

/// The HTTP method used to generate the notification. If the notification was generated during a phone call, this is the HTTP Method used to request the resource on your server. If the notification was generated by your use of our REST API, this is the HTTP method used to call the resource on our servers.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RequestMethod {
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
}

impl Default for RequestMethod {
    fn default() -> RequestMethod {
        Self::Head
    }
}

