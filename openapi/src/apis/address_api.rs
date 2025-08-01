/*
 * Starlink Enterprise API
 *
 * <h3>Description</h3>API to manage accounts and user terminals. This page is deprecated, please use the new documentation site: <a href='https://starlink.readme.io/'>https://starlink.readme.io/</a><h3>Authentication - OIDC</h3><p>To authenticate with this API using OIDC, <a target='_blank' href='/api/auth/.well-known/openid-configuration'>Well Known URL</a> and attach the result to your requests with the <strong>Authorize</strong> button below.</p>
 *
 * The version of the OpenAPI document: 1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`enterprise_v1_account_account_number_addresses_address_reference_id_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnterpriseV1AccountAccountNumberAddressesAddressReferenceIdGetError {
    Status403(),
    Status422(models::AddressResponseServiceResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`enterprise_v1_account_account_number_addresses_check_capacity_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnterpriseV1AccountAccountNumberAddressesCheckCapacityPostError {
    Status403(),
    Status422(models::CapacityCheckResponseServiceResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`enterprise_v1_account_account_number_addresses_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnterpriseV1AccountAccountNumberAddressesGetError {
    Status403(),
    Status422(models::AddressResponsePaginatedServiceResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`enterprise_v1_account_account_number_addresses_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnterpriseV1AccountAccountNumberAddressesPostError {
    Status403(),
    Status422(models::AddressResponseServiceResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`enterprise_v1_account_account_number_addresses_put`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnterpriseV1AccountAccountNumberAddressesPutError {
    Status403(),
    Status422(models::AddressResponseServiceResponse),
    UnknownValue(serde_json::Value),
}


pub async fn enterprise_v1_account_account_number_addresses_address_reference_id_get(configuration: &configuration::Configuration, account_number: &str, address_reference_id: &str) -> Result<models::AddressResponseServiceResponse, Error<EnterpriseV1AccountAccountNumberAddressesAddressReferenceIdGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_account_number = account_number;
    let p_address_reference_id = address_reference_id;

    let uri_str = format!("{}/enterprise/v1/account/{accountNumber}/addresses/{addressReferenceId}", configuration.base_path, accountNumber=crate::apis::urlencode(p_account_number), addressReferenceId=crate::apis::urlencode(p_address_reference_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AddressResponseServiceResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AddressResponseServiceResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<EnterpriseV1AccountAccountNumberAddressesAddressReferenceIdGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Check Starlink network capacity at a specific coordinate.
pub async fn enterprise_v1_account_account_number_addresses_check_capacity_post(configuration: &configuration::Configuration, account_number: &str, capacity_check_request: Option<models::CapacityCheckRequest>) -> Result<models::CapacityCheckResponseServiceResponse, Error<EnterpriseV1AccountAccountNumberAddressesCheckCapacityPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_account_number = account_number;
    let p_capacity_check_request = capacity_check_request;

    let uri_str = format!("{}/enterprise/v1/account/{accountNumber}/addresses/check-capacity", configuration.base_path, accountNumber=crate::apis::urlencode(p_account_number));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_capacity_check_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::CapacityCheckResponseServiceResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::CapacityCheckResponseServiceResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<EnterpriseV1AccountAccountNumberAddressesCheckCapacityPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn enterprise_v1_account_account_number_addresses_get(configuration: &configuration::Configuration, account_number: &str, address_ids: Option<Vec<uuid::Uuid>>, metadata: Option<&str>, limit: Option<i32>, page: Option<i32>) -> Result<models::AddressResponsePaginatedServiceResponse, Error<EnterpriseV1AccountAccountNumberAddressesGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_account_number = account_number;
    let p_address_ids = address_ids;
    let p_metadata = metadata;
    let p_limit = limit;
    let p_page = page;

    let uri_str = format!("{}/enterprise/v1/account/{accountNumber}/addresses", configuration.base_path, accountNumber=crate::apis::urlencode(p_account_number));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_address_ids {
        req_builder = match "multi" {
            "multi" => req_builder.query(&param_value.into_iter().map(|p| ("addressIds".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => req_builder.query(&[("addressIds", &param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref param_value) = p_metadata {
        req_builder = req_builder.query(&[("metadata", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AddressResponsePaginatedServiceResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AddressResponsePaginatedServiceResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<EnterpriseV1AccountAccountNumberAddressesGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn enterprise_v1_account_account_number_addresses_post(configuration: &configuration::Configuration, account_number: &str, address_create_request: Option<models::AddressCreateRequest>) -> Result<models::AddressResponseServiceResponse, Error<EnterpriseV1AccountAccountNumberAddressesPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_account_number = account_number;
    let p_address_create_request = address_create_request;

    let uri_str = format!("{}/enterprise/v1/account/{accountNumber}/addresses", configuration.base_path, accountNumber=crate::apis::urlencode(p_account_number));
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_address_create_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AddressResponseServiceResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AddressResponseServiceResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<EnterpriseV1AccountAccountNumberAddressesPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn enterprise_v1_account_account_number_addresses_put(configuration: &configuration::Configuration, account_number: &str, address_update_request: Option<models::AddressUpdateRequest>) -> Result<models::AddressResponseServiceResponse, Error<EnterpriseV1AccountAccountNumberAddressesPutError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_account_number = account_number;
    let p_address_update_request = address_update_request;

    let uri_str = format!("{}/enterprise/v1/account/{accountNumber}/addresses", configuration.base_path, accountNumber=crate::apis::urlencode(p_account_number));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_address_update_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AddressResponseServiceResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AddressResponseServiceResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<EnterpriseV1AccountAccountNumberAddressesPutError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

