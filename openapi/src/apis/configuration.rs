/*
 * Starlink Enterprise API
 *
 * <h3>Description</h3>API to manage accounts and user terminals. This page is deprecated, please use the new documentation site: <a href='https://starlink.readme.io/'>https://starlink.readme.io/</a><h3>Authentication - OIDC</h3><p>To authenticate with this API using OIDC, <a target='_blank' href='/api/auth/.well-known/openid-configuration'>Well Known URL</a> and attach the result to your requests with the <strong>Authorize</strong> button below.</p>
 *
 * The version of the OpenAPI document: 1
 * 
 * Generated by: https://openapi-generator.tech
 */



#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: reqwest::Client,
    pub basic_auth: Option<BasicAuth>,
    pub oauth_access_token: Option<String>,
    pub bearer_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
}

pub type BasicAuth = (String, Option<String>);

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}


impl Configuration {
    pub fn new() -> Configuration {
        Configuration::default()
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            base_path: "http://localhost".to_owned(),
            user_agent: Some("OpenAPI-Generator/1/rust".to_owned()),
            client: reqwest::Client::new(),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        }
    }
}
