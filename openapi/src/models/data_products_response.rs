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
pub struct DataProductsResponse {
    #[serde(rename = "topUpProduct", skip_serializing_if = "Option::is_none")]
    pub top_up_product: Option<Box<models::DataProductResponse>>,
    /// The data block products that are available for this service plan.
    #[serde(rename = "dataBlockProducts", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub data_block_products: Option<Option<Vec<models::DataProductResponse>>>,
}

impl DataProductsResponse {
    pub fn new() -> DataProductsResponse {
        DataProductsResponse {
            top_up_product: None,
            data_block_products: None,
        }
    }
}

