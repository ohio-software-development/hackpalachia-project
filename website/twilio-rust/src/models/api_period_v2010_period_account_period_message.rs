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
pub struct ApiPeriodV2010PeriodAccountPeriodMessage {
    /// The message text. Can be up to 1,600 characters long.
    #[serde(rename = "body", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub body: Option<Option<String>>,
    /// The number of segments that make up the complete message. A message body that is too large to be sent in a single SMS message is segmented and charged as multiple messages. Inbound messages over 160 characters are reassembled when the message is received. Note: When using a Messaging Service to send messages, `num_segments` will always be 0 in Twilio's response to your API request.
    #[serde(rename = "num_segments", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub num_segments: Option<Option<String>>,
    #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<crate::models::MessageEnumDirection>,
    /// The phone number (in [E.164](https://en.wikipedia.org/wiki/E.164) format), [alphanumeric sender ID](https://www.twilio.com/docs/sms/send-messages#use-an-alphanumeric-sender-id), or [Wireless SIM](https://www.twilio.com/docs/wireless/tutorials/communications-guides/how-to-send-and-receive-text-messages) that initiated the message. For incoming messages, this will be the number of the sending phone. For outgoing messages, this value will be one of your Twilio phone numbers or the alphanumeric sender ID used.
    #[serde(rename = "from", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub from: Option<Option<String>>,
    /// The phone number in [E.164](https://en.wikipedia.org/wiki/E.164) format that received the message. For incoming messages, this will be one of your Twilio phone numbers. For outgoing messages, this will be the sending phone.
    #[serde(rename = "to", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub to: Option<Option<String>>,
    /// The date and time in GMT that the resource was last updated specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<Option<String>>,
    /// The amount billed for the message, in the currency specified by `price_unit`.  Note that your account is charged for each segment we send to the handset. Populated after the message has been sent. May not be immediately available.
    #[serde(rename = "price", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub price: Option<Option<String>>,
    /// The description of the `error_code` if your message `status` is `failed` or `undelivered`. If the message was successful, this value is null.
    #[serde(rename = "error_message", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<Option<String>>,
    /// The URI of the resource, relative to `https://api.twilio.com`.
    #[serde(rename = "uri", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub uri: Option<Option<String>>,
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that sent the message that created the resource.
    #[serde(rename = "account_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<Option<String>>,
    /// The number of media files associated with the message. A message can send up to 10 media files.
    #[serde(rename = "num_media", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub num_media: Option<Option<String>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::MessageEnumStatus>,
    /// The SID of the [Messaging Service](https://www.twilio.com/docs/sms/services/api) used with the message. The value is null if a Messaging Service was not used.
    #[serde(rename = "messaging_service_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub messaging_service_sid: Option<Option<String>>,
    /// The unique string that that we created to identify the Message resource.
    #[serde(rename = "sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sid: Option<Option<String>>,
    /// The date and time in GMT that the resource was sent specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format. For outgoing messages, this is when we sent the message. For incoming messages, this is when we made the HTTP request to your application. 
    #[serde(rename = "date_sent", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_sent: Option<Option<String>>,
    /// The date and time in GMT that the resource was created specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<Option<String>>,
    /// The error code returned if your message `status` is `failed` or `undelivered`. The error_code provides more information about the failure. If the message was successful, this value is null.
    #[serde(rename = "error_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<Option<i32>>,
    /// The currency in which `price` is measured, in [ISO 4127](https://www.iso.org/iso/home/standards/currency_codes.htm) format (e.g. `usd`, `eur`, `jpy`).
    #[serde(rename = "price_unit", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub price_unit: Option<Option<String>>,
    /// The API version used to process the message.
    #[serde(rename = "api_version", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<Option<String>>,
    /// A list of related resources identified by their URIs relative to `https://api.twilio.com`
    #[serde(rename = "subresource_uris", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub subresource_uris: Option<Option<serde_json::Value>>,
}

impl ApiPeriodV2010PeriodAccountPeriodMessage {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodMessage {
        ApiPeriodV2010PeriodAccountPeriodMessage {
            body: None,
            num_segments: None,
            direction: None,
            from: None,
            to: None,
            date_updated: None,
            price: None,
            error_message: None,
            uri: None,
            account_sid: None,
            num_media: None,
            status: None,
            messaging_service_sid: None,
            sid: None,
            date_sent: None,
            date_created: None,
            error_code: None,
            price_unit: None,
            api_version: None,
            subresource_uris: None,
        }
    }
}


