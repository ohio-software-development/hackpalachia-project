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
pub struct ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumber {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created this IncomingPhoneNumber resource.
    #[serde(rename = "account_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<Option<String>>,
    /// The SID of the Address resource associated with the phone number.
    #[serde(rename = "address_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub address_sid: Option<Option<String>>,
    #[serde(rename = "address_requirements", skip_serializing_if = "Option::is_none")]
    pub address_requirements: Option<crate::models::IncomingPhoneNumberEnumAddressRequirement>,
    /// The API version used to start a new TwiML session.
    #[serde(rename = "api_version", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<Option<String>>,
    /// Whether the phone number is new to the Twilio platform. Can be: `true` or `false`.
    #[serde(rename = "beta", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub beta: Option<Option<bool>>,
    #[serde(rename = "capabilities", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Option<Box<crate::models::ApiV2010AccountIncomingPhoneNumberCapabilities>>>,
    /// The date and time in GMT that the resource was created specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<Option<String>>,
    /// The date and time in GMT that the resource was last updated specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<Option<String>>,
    /// The string that you assigned to describe the resource.
    #[serde(rename = "friendly_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<Option<String>>,
    /// The SID of the Identity resource that we associate with the phone number. Some regions require an Identity to meet local regulations.
    #[serde(rename = "identity_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub identity_sid: Option<Option<String>>,
    /// The phone number in [E.164](https://www.twilio.com/docs/glossary/what-e164) format, which consists of a + followed by the country code and subscriber number.
    #[serde(rename = "phone_number", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<Option<String>>,
    /// The phone number's origin. `twilio` identifies Twilio-owned phone numbers and `hosted` identifies hosted phone numbers.
    #[serde(rename = "origin", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub origin: Option<Option<String>>,
    /// The unique string that that we created to identify this IncomingPhoneNumber resource.
    #[serde(rename = "sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sid: Option<Option<String>>,
    /// The SID of the application that handles SMS messages sent to the phone number. If an `sms_application_sid` is present, we ignore all `sms_*_url` values and use those of the application.
    #[serde(rename = "sms_application_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sms_application_sid: Option<Option<String>>,
    /// The HTTP method we use to call `sms_fallback_url`. Can be: `GET` or `POST`.
    #[serde(rename = "sms_fallback_method", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sms_fallback_method: Option<Option<SmsFallbackMethod>>,
    /// The URL that we call when an error occurs while retrieving or executing the TwiML from `sms_url`.
    #[serde(rename = "sms_fallback_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sms_fallback_url: Option<Option<String>>,
    /// The HTTP method we use to call `sms_url`. Can be: `GET` or `POST`.
    #[serde(rename = "sms_method", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sms_method: Option<Option<SmsMethod>>,
    /// The URL we call when the phone number receives an incoming SMS message.
    #[serde(rename = "sms_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sms_url: Option<Option<String>>,
    /// The URL we call using the `status_callback_method` to send status information to your application.
    #[serde(rename = "status_callback", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub status_callback: Option<Option<String>>,
    /// The HTTP method we use to call `status_callback`. Can be: `GET` or `POST`.
    #[serde(rename = "status_callback_method", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub status_callback_method: Option<Option<StatusCallbackMethod>>,
    /// The SID of the Trunk that handles calls to the phone number. If a `trunk_sid` is present, we ignore all of the voice urls and voice applications and use those set on the Trunk. Setting a `trunk_sid` will automatically delete your `voice_application_sid` and vice versa.
    #[serde(rename = "trunk_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub trunk_sid: Option<Option<String>>,
    /// The URI of the resource, relative to `https://api.twilio.com`.
    #[serde(rename = "uri", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub uri: Option<Option<String>>,
    #[serde(rename = "voice_receive_mode", skip_serializing_if = "Option::is_none")]
    pub voice_receive_mode: Option<crate::models::IncomingPhoneNumberEnumVoiceReceiveMode>,
    /// The SID of the application that handles calls to the phone number. If a `voice_application_sid` is present, we ignore all of the voice urls and use those set on the application. Setting a `voice_application_sid` will automatically delete your `trunk_sid` and vice versa.
    #[serde(rename = "voice_application_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub voice_application_sid: Option<Option<String>>,
    /// Whether we look up the caller's caller-ID name from the CNAM database ($0.01 per look up). Can be: `true` or `false`.
    #[serde(rename = "voice_caller_id_lookup", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub voice_caller_id_lookup: Option<Option<bool>>,
    /// The HTTP method we use to call `voice_fallback_url`. Can be: `GET` or `POST`.
    #[serde(rename = "voice_fallback_method", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub voice_fallback_method: Option<Option<VoiceFallbackMethod>>,
    /// The URL that we call when an error occurs retrieving or executing the TwiML requested by `url`.
    #[serde(rename = "voice_fallback_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub voice_fallback_url: Option<Option<String>>,
    /// The HTTP method we use to call `voice_url`. Can be: `GET` or `POST`.
    #[serde(rename = "voice_method", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub voice_method: Option<Option<VoiceMethod>>,
    /// The URL we call when the phone number receives a call. The `voice_url` will not be used if a `voice_application_sid` or a `trunk_sid` is set.
    #[serde(rename = "voice_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub voice_url: Option<Option<String>>,
    #[serde(rename = "emergency_status", skip_serializing_if = "Option::is_none")]
    pub emergency_status: Option<crate::models::IncomingPhoneNumberEnumEmergencyStatus>,
    /// The SID of the emergency address configuration that we use for emergency calling from this phone number.
    #[serde(rename = "emergency_address_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub emergency_address_sid: Option<Option<String>>,
    #[serde(rename = "emergency_address_status", skip_serializing_if = "Option::is_none")]
    pub emergency_address_status: Option<crate::models::IncomingPhoneNumberEnumEmergencyAddressStatus>,
    /// The SID of the Bundle resource that you associate with the phone number. Some regions require a Bundle to meet local Regulations.
    #[serde(rename = "bundle_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub bundle_sid: Option<Option<String>>,
    #[serde(rename = "status", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<String>>,
}

impl ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumber {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumber {
        ApiPeriodV2010PeriodAccountPeriodIncomingPhoneNumber {
            account_sid: None,
            address_sid: None,
            address_requirements: None,
            api_version: None,
            beta: None,
            capabilities: None,
            date_created: None,
            date_updated: None,
            friendly_name: None,
            identity_sid: None,
            phone_number: None,
            origin: None,
            sid: None,
            sms_application_sid: None,
            sms_fallback_method: None,
            sms_fallback_url: None,
            sms_method: None,
            sms_url: None,
            status_callback: None,
            status_callback_method: None,
            trunk_sid: None,
            uri: None,
            voice_receive_mode: None,
            voice_application_sid: None,
            voice_caller_id_lookup: None,
            voice_fallback_method: None,
            voice_fallback_url: None,
            voice_method: None,
            voice_url: None,
            emergency_status: None,
            emergency_address_sid: None,
            emergency_address_status: None,
            bundle_sid: None,
            status: None,
        }
    }
}

/// The HTTP method we use to call `sms_fallback_url`. Can be: `GET` or `POST`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SmsFallbackMethod {
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

impl Default for SmsFallbackMethod {
    fn default() -> SmsFallbackMethod {
        Self::Head
    }
}
/// The HTTP method we use to call `sms_url`. Can be: `GET` or `POST`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SmsMethod {
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

impl Default for SmsMethod {
    fn default() -> SmsMethod {
        Self::Head
    }
}
/// The HTTP method we use to call `status_callback`. Can be: `GET` or `POST`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StatusCallbackMethod {
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

impl Default for StatusCallbackMethod {
    fn default() -> StatusCallbackMethod {
        Self::Head
    }
}
/// The HTTP method we use to call `voice_fallback_url`. Can be: `GET` or `POST`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VoiceFallbackMethod {
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

impl Default for VoiceFallbackMethod {
    fn default() -> VoiceFallbackMethod {
        Self::Head
    }
}
/// The HTTP method we use to call `voice_url`. Can be: `GET` or `POST`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VoiceMethod {
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

impl Default for VoiceMethod {
    fn default() -> VoiceMethod {
        Self::Head
    }
}

