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
pub struct ApiPeriodV2010PeriodAccountPeriodAuthorizedConnectApp {
    /// The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the AuthorizedConnectApp resource.
    #[serde(rename = "account_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<Option<String>>,
    /// The company name set for the Connect App.
    #[serde(rename = "connect_app_company_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub connect_app_company_name: Option<Option<String>>,
    /// A detailed description of the Connect App.
    #[serde(rename = "connect_app_description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub connect_app_description: Option<Option<String>>,
    /// The name of the Connect App.
    #[serde(rename = "connect_app_friendly_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub connect_app_friendly_name: Option<Option<String>>,
    /// The public URL for the Connect App.
    #[serde(rename = "connect_app_homepage_url", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub connect_app_homepage_url: Option<Option<String>>,
    /// The SID that we assigned to the Connect App.
    #[serde(rename = "connect_app_sid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub connect_app_sid: Option<Option<String>>,
    /// The date and time in GMT that the resource was created specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<Option<String>>,
    /// The date and time in GMT that the resource was last updated specified in [RFC 2822](https://www.ietf.org/rfc/rfc2822.txt) format.
    #[serde(rename = "date_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<Option<String>>,
    /// The set of permissions that you authorized for the Connect App.  Can be: `get-all` or `post-all`.
    #[serde(rename = "permissions", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Option<Vec<crate::models::AuthorizedConnectAppEnumPermission>>>,
    /// The URI of the resource, relative to `https://api.twilio.com`.
    #[serde(rename = "uri", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub uri: Option<Option<String>>,
}

impl ApiPeriodV2010PeriodAccountPeriodAuthorizedConnectApp {
    pub fn new() -> ApiPeriodV2010PeriodAccountPeriodAuthorizedConnectApp {
        ApiPeriodV2010PeriodAccountPeriodAuthorizedConnectApp {
            account_sid: None,
            connect_app_company_name: None,
            connect_app_description: None,
            connect_app_friendly_name: None,
            connect_app_homepage_url: None,
            connect_app_sid: None,
            date_created: None,
            date_updated: None,
            permissions: None,
            uri: None,
        }
    }
}


