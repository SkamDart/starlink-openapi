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
pub struct UserLacksRequiredPermission {
    #[serde(rename = "accountId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<Option<String>>,
    #[serde(rename = "requiredPermission", skip_serializing_if = "Option::is_none")]
    pub required_permission: Option<Box<models::FeaturePermission>>,
}

impl UserLacksRequiredPermission {
    pub fn new() -> UserLacksRequiredPermission {
        UserLacksRequiredPermission {
            account_id: None,
            required_permission: None,
        }
    }
}

