/*
 * Airbyte Configuration API
 *
 * Airbyte Configuration API [https://airbyte.io](https://airbyte.io).  This API is a collection of HTTP RPC-style methods. While it is not a REST API, those familiar with REST should find the conventions of this API recognizable.  Here are some conventions that this API follows: * All endpoints are http POST methods. * All endpoints accept data via `application/json` request bodies. The API does not accept any data via query params. * The naming convention for endpoints is: localhost:8000/{VERSION}/{METHOD_FAMILY}/{METHOD_NAME} e.g. `localhost:8000/v1/connections/create`. * For all `update` methods, the whole object must be passed in, even the fields that did not change.  Change Management: * The major version of the API endpoint can be determined / specified in the URL `localhost:8080/v1/connections/create` * Minor version bumps will be invisible to the end user. The user cannot specify minor versions in requests. * All backwards incompatible changes will happen in major version bumps. We will not make backwards incompatible changes in minor version bumps. Examples of non-breaking changes (includes but not limited to...):   * Adding fields to request or response bodies.   * Adding new HTTP endpoints.
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: contact@airbyte.io
 * Generated by: https://openapi-generator.tech
 */

/// AirbyteStreamConfiguration : the mutable part of the stream to configure the destination

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AirbyteStreamConfiguration {
    #[serde(rename = "syncMode")]
    pub sync_mode: crate::models::SyncMode,
    /// Path to the field that will be used to determine if a record is new or modified since the last sync. This field is REQUIRED if `sync_mode` is `incremental`. Otherwise it is ignored.
    #[serde(rename = "cursorField", skip_serializing_if = "Option::is_none")]
    pub cursor_field: Option<Vec<String>>,
    #[serde(rename = "destinationSyncMode")]
    pub destination_sync_mode: crate::models::DestinationSyncMode,
    /// Paths to the fields that will be used as primary key. This field is REQUIRED if `destination_sync_mode` is `*_dedup`. Otherwise it is ignored.
    #[serde(rename = "primaryKey", skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<Vec<Vec<String>>>,
    /// Alias name to the stream to be used in the destination
    #[serde(rename = "aliasName", skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    #[serde(rename = "selected", skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,
}

impl AirbyteStreamConfiguration {
    /// the mutable part of the stream to configure the destination
    pub fn new(
        sync_mode: crate::models::SyncMode,
        destination_sync_mode: crate::models::DestinationSyncMode,
    ) -> AirbyteStreamConfiguration {
        AirbyteStreamConfiguration {
            sync_mode,
            cursor_field: None,
            destination_sync_mode,
            primary_key: None,
            alias_name: None,
            selected: None,
        }
    }
}
