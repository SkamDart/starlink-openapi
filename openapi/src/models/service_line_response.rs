/*
 * Starlink Enterprise API
 *
 * <h3>Description</h3>API to manage accounts and user terminals. This page is deprecated, please use the new documentation site: <a href='https://starlink.readme.io/'>https://starlink.readme.io/</a><h3>Authentication - OIDC</h3><p>To authenticate with this API using OIDC, <a target='_blank' href='/api/auth/.well-known/openid-configuration'>Well Known URL</a> and attach the result to your requests with the <strong>Authorize</strong> button below.</p>
 *
 * The version of the OpenAPI document: 1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceLineResponse {
    /// Address Reference ID of the address associated with the service line. Example: 55ec6574-10d8-bd9c-1951-d4184f4ae467
    #[serde(rename = "addressReferenceId", skip_serializing_if = "Option::is_none")]
    pub address_reference_id: Option<uuid::Uuid>,
    /// The Service Line Number. Example: AST-511274-31364-54
    #[serde(rename = "serviceLineNumber", skip_serializing_if = "Option::is_none")]
    pub service_line_number: Option<String>,
    /// A user-defined nickname for this service line
    #[serde(rename = "nickname", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub nickname: Option<Option<String>>,
    /// The unique product identifier
    #[serde(rename = "productReferenceId", skip_serializing_if = "Option::is_none")]
    pub product_reference_id: Option<String>,
    /// Scheduled product change for next bill date
    #[serde(rename = "delayedProductId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub delayed_product_id: Option<Option<String>>,
    /// Opt-in product id, opted out if empty
    #[serde(rename = "optInProductId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub opt_in_product_id: Option<Option<String>>,
    /// The start date of the subscription. This is in UTC.
    #[serde(rename = "startDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Option<String>>,
    /// The service line deactivation date, which only appears if the service line is deactivated. This is in UTC.
    #[serde(rename = "endDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Option<String>>,
    /// Indicates if service line is public IP
    #[serde(rename = "publicIp", skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<bool>,
    /// Indicates if service line is active
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "aviationMetadata", skip_serializing_if = "Option::is_none")]
    pub aviation_metadata: Option<Box<models::AviationMetadataResponse>>,
    #[serde(rename = "dataBlocks", skip_serializing_if = "Option::is_none")]
    pub data_blocks: Option<Box<models::ServiceLineDataBlocksSummaryResponse>>,
}

impl ServiceLineResponse {
    pub fn new() -> ServiceLineResponse {
        ServiceLineResponse {
            address_reference_id: None,
            service_line_number: None,
            nickname: None,
            product_reference_id: None,
            delayed_product_id: None,
            opt_in_product_id: None,
            start_date: None,
            end_date: None,
            public_ip: None,
            active: None,
            aviation_metadata: None,
            data_blocks: None,
        }
    }
}

