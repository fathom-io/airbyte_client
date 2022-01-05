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
pub struct WorkspaceRead {
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
    #[serde(rename = "customerId")]
    pub customer_id: String,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "slug")]
    pub slug: String,
    #[serde(rename = "initialSetupComplete")]
    pub initial_setup_complete: bool,
    #[serde(rename = "displaySetupWizard", skip_serializing_if = "Option::is_none")]
    pub display_setup_wizard: Option<bool>,
    #[serde(rename = "anonymousDataCollection", skip_serializing_if = "Option::is_none")]
    pub anonymous_data_collection: Option<bool>,
    #[serde(rename = "news", skip_serializing_if = "Option::is_none")]
    pub news: Option<bool>,
    #[serde(rename = "securityUpdates", skip_serializing_if = "Option::is_none")]
    pub security_updates: Option<bool>,
    #[serde(rename = "notifications", skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Vec<crate::models::Notification>>,
    #[serde(rename = "firstCompletedSync", skip_serializing_if = "Option::is_none")]
    pub first_completed_sync: Option<bool>,
    #[serde(rename = "feedbackDone", skip_serializing_if = "Option::is_none")]
    pub feedback_done: Option<bool>,
}

impl WorkspaceRead {
    pub fn new(workspace_id: String, customer_id: String, name: String, slug: String, initial_setup_complete: bool) -> WorkspaceRead {
        WorkspaceRead {
            workspace_id,
            customer_id,
            email: None,
            name,
            slug,
            initial_setup_complete,
            display_setup_wizard: None,
            anonymous_data_collection: None,
            news: None,
            security_updates: None,
            notifications: None,
            first_completed_sync: None,
            feedback_done: None,
        }
    }
}


