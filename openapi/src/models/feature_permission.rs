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
pub struct FeaturePermission {
    #[serde(rename = "featureAccess")]
    pub feature_access: models::FeatureAccess,
    #[serde(rename = "permission")]
    pub permission: models::Permission,
}

impl FeaturePermission {
    pub fn new(feature_access: models::FeatureAccess, permission: models::Permission) -> FeaturePermission {
        FeaturePermission {
            feature_access,
            permission,
        }
    }
}

