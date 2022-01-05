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
pub struct OperationRead {
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
    #[serde(rename = "operationId")]
    pub operation_id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "operatorConfiguration")]
    pub operator_configuration: Box<crate::models::OperatorConfiguration>,
}

impl OperationRead {
    pub fn new(
        workspace_id: String,
        operation_id: String,
        name: String,
        operator_configuration: crate::models::OperatorConfiguration,
    ) -> OperationRead {
        OperationRead {
            workspace_id,
            operation_id,
            name,
            operator_configuration: Box::new(operator_configuration),
        }
    }
}
