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
pub struct ServiceLineCreateRequest {
    /// Address Reference ID to associate with the service line. Example: 55ec6574-10d8-bd9c-1951-d4184f4ae467
    #[serde(rename = "addressReferenceId")]
    pub address_reference_id: uuid::Uuid,
    /// Subscription Product ID to associate with the service line. Example: business-subscription-100
    #[serde(rename = "productReferenceId")]
    pub product_reference_id: String,
    #[serde(rename = "dataBlockProducts", skip_serializing_if = "Option::is_none")]
    pub data_block_products: Option<Box<models::SetRecurringDataBlocksRequest>>,
}

impl ServiceLineCreateRequest {
    pub fn new(address_reference_id: uuid::Uuid, product_reference_id: String) -> ServiceLineCreateRequest {
        ServiceLineCreateRequest {
            address_reference_id,
            product_reference_id,
            data_block_products: None,
        }
    }
}

