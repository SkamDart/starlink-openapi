# \AccountsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**enterprise_v1_accounts_account_number_billing_cycles_query_post**](AccountsApi.md#enterprise_v1_accounts_account_number_billing_cycles_query_post) | **POST** /enterprise/v1/accounts/{accountNumber}/billing-cycles/query | Get data usage
[**enterprise_v1_accounts_account_number_router_local_content_get**](AccountsApi.md#enterprise_v1_accounts_account_number_router_local_content_get) | **GET** /enterprise/v1/accounts/{accountNumber}/router-local-content | Get list of router local content files
[**enterprise_v1_accounts_account_number_router_local_content_post**](AccountsApi.md#enterprise_v1_accounts_account_number_router_local_content_post) | **POST** /enterprise/v1/accounts/{accountNumber}/router-local-content | Upload router local content file.
[**enterprise_v1_accounts_account_number_update_default_router_config_put**](AccountsApi.md#enterprise_v1_accounts_account_number_update_default_router_config_put) | **PUT** /enterprise/v1/accounts/{accountNumber}/update-default-router-config | Set default router config
[**enterprise_v1_accounts_account_number_update_default_router_tls_domain_put**](AccountsApi.md#enterprise_v1_accounts_account_number_update_default_router_tls_domain_put) | **PUT** /enterprise/v1/accounts/{accountNumber}/update-default-router-tls-domain | Set default router tls domain.
[**enterprise_v1_accounts_get**](AccountsApi.md#enterprise_v1_accounts_get) | **GET** /enterprise/v1/accounts | Get accounts



## enterprise_v1_accounts_account_number_billing_cycles_query_post

> models::ServiceLineDataUsageForBillingCyclesPaginatedServiceResponse enterprise_v1_accounts_account_number_billing_cycles_query_post(account_number, query_billing_cycle_request)
Get data usage

Retrieve the real-time data tracking system for an account's data usage data.  For detailed instructions, please refer to the API documentation: https://starlink.readme.io/docs/data-usage-api

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number | [required] |
**query_billing_cycle_request** | Option<[**QueryBillingCycleRequest**](QueryBillingCycleRequest.md)> | QueryBillingCycleRequest |  |

### Return type

[**models::ServiceLineDataUsageForBillingCyclesPaginatedServiceResponse**](ServiceLineDataUsageForBillingCyclesPaginatedServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_accounts_account_number_router_local_content_get

> models::ServiceResponse enterprise_v1_accounts_account_number_router_local_content_get(account_number)
Get list of router local content files

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number | [required] |

### Return type

[**models::ServiceResponse**](ServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_accounts_account_number_router_local_content_post

> models::ServiceResponse enterprise_v1_accounts_account_number_router_local_content_post(account_number, file)
Upload router local content file.

Upload html file to allow it to be configured as the HTTPS server local content file for router configs.  File should be attached as multipart/form-data.  See https://starlink.readme.io/docs/local-content for example upload script.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number | [required] |
**file** | **std::path::PathBuf** |  | [required] |

### Return type

[**models::ServiceResponse**](ServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_accounts_account_number_update_default_router_config_put

> models::AccountResponsePaginatedServiceResponse enterprise_v1_accounts_account_number_update_default_router_config_put(account_number, update_default_config_request)
Set default router config

Set the default router config on the account. Use an empty string to remove the default  config from the account. Any new routers on this account will be assigned this config.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number | [required] |
**update_default_config_request** | Option<[**UpdateDefaultConfigRequest**](UpdateDefaultConfigRequest.md)> | UpdateDefaultConfigRequest |  |

### Return type

[**models::AccountResponsePaginatedServiceResponse**](AccountResponsePaginatedServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_accounts_account_number_update_default_router_tls_domain_put

> models::ServiceResponse enterprise_v1_accounts_account_number_update_default_router_tls_domain_put(account_number, update_default_router_tls_request)
Set default router tls domain.

Set the default tls domain for routers under the account. Use an empty string to remove the default from the account.  By default, all routers on this account will host a HTTPS server at this domain.  A tls config must exist for this domain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number | [required] |
**update_default_router_tls_request** | Option<[**UpdateDefaultRouterTlsRequest**](UpdateDefaultRouterTlsRequest.md)> | UpdateDefaultRouterTlsRequest |  |

### Return type

[**models::ServiceResponse**](ServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_accounts_get

> models::AccountResponsePaginatedServiceResponse enterprise_v1_accounts_get(region_code, limit, page)
Get accounts

Get paginated accounts of the current authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region_code** | Option<[**Vec<String>**](String.md)> | The region code of the account |  |
**limit** | Option<**i32**> | The number of accounts to return at a time |  |[default to 50]
**page** | Option<**i32**> | The index of the page, starting at 0 |  |[default to 0]

### Return type

[**models::AccountResponsePaginatedServiceResponse**](AccountResponsePaginatedServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

