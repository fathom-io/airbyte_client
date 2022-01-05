/*
 * Airbyte Configuration API
 *
 * Airbyte Configuration API [https://airbyte.io](https://airbyte.io).  This API is a collection of HTTP RPC-style methods. While it is not a REST API, those familiar with REST should find the conventions of this API recognizable.  Here are some conventions that this API follows: * All endpoints are http POST methods. * All endpoints accept data via `application/json` request bodies. The API does not accept any data via query params. * The naming convention for endpoints is: localhost:8000/{VERSION}/{METHOD_FAMILY}/{METHOD_NAME} e.g. `localhost:8000/v1/connections/create`. * For all `update` methods, the whole object must be passed in, even the fields that did not change.  Change Management: * The major version of the API endpoint can be determined / specified in the URL `localhost:8080/v1/connections/create` * Minor version bumps will be invisible to the end user. The user cannot specify minor versions in requests. * All backwards incompatible changes will happen in major version bumps. We will not make backwards incompatible changes in minor version bumps. Examples of non-breaking changes (includes but not limited to...):   * Adding fields to request or response bodies.   * Adding new HTTP endpoints.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: contact@airbyte.io
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Notification {
    #[serde(rename = "notificationType")]
    pub notification_type: crate::models::NotificationType,
    #[serde(rename = "sendOnSuccess")]
    pub send_on_success: bool,
    #[serde(rename = "sendOnFailure")]
    pub send_on_failure: bool,
    #[serde(rename = "slackConfiguration", skip_serializing_if = "Option::is_none")]
    pub slack_configuration: Option<Box<crate::models::SlackNotificationConfiguration>>,
}

impl Notification {
    pub fn new(
        notification_type: crate::models::NotificationType,
        send_on_success: bool,
        send_on_failure: bool,
    ) -> Notification {
        Notification {
            notification_type,
            send_on_success,
            send_on_failure,
            slack_configuration: None,
        }
    }
}
