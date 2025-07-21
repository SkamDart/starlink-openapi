# \RouterApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**enterprise_v1_account_account_number_routers_batch_config_put**](RouterApi.md#enterprise_v1_account_account_number_routers_batch_config_put) | **PUT** /enterprise/v1/account/{accountNumber}/routers/batch-config | Batch config assignment
[**enterprise_v1_account_account_number_routers_configs_config_id_get**](RouterApi.md#enterprise_v1_account_account_number_routers_configs_config_id_get) | **GET** /enterprise/v1/account/{accountNumber}/routers/configs/{configId} | Get router config
[**enterprise_v1_account_account_number_routers_configs_config_id_put**](RouterApi.md#enterprise_v1_account_account_number_routers_configs_config_id_put) | **PUT** /enterprise/v1/account/{accountNumber}/routers/configs/{configId} | Update router config
[**enterprise_v1_account_account_number_routers_configs_get**](RouterApi.md#enterprise_v1_account_account_number_routers_configs_get) | **GET** /enterprise/v1/account/{accountNumber}/routers/configs | Get all router configs
[**enterprise_v1_account_account_number_routers_configs_post**](RouterApi.md#enterprise_v1_account_account_number_routers_configs_post) | **POST** /enterprise/v1/account/{accountNumber}/routers/configs | Create router config
[**enterprise_v1_account_account_number_routers_router_id_config_delete**](RouterApi.md#enterprise_v1_account_account_number_routers_router_id_config_delete) | **DELETE** /enterprise/v1/account/{accountNumber}/routers/{routerId}/config | Remove config assignment
[**enterprise_v1_account_account_number_routers_router_id_config_put**](RouterApi.md#enterprise_v1_account_account_number_routers_router_id_config_put) | **PUT** /enterprise/v1/account/{accountNumber}/routers/{routerId}/config | Set config assignment
[**enterprise_v1_account_account_number_routers_router_id_get**](RouterApi.md#enterprise_v1_account_account_number_routers_router_id_get) | **GET** /enterprise/v1/account/{accountNumber}/routers/{routerId} | Get router
[**enterprise_v1_account_account_number_routers_router_id_reboot_post**](RouterApi.md#enterprise_v1_account_account_number_routers_router_id_reboot_post) | **POST** /enterprise/v1/account/{accountNumber}/routers/{routerId}/reboot | Reboot router
[**enterprise_v1_account_account_number_routers_sandbox_client_post**](RouterApi.md#enterprise_v1_account_account_number_routers_sandbox_client_post) | **POST** /enterprise/v1/account/{accountNumber}/routers/sandbox/client | Update sandbox client
[**enterprise_v1_account_account_number_routers_sandbox_clients_get**](RouterApi.md#enterprise_v1_account_account_number_routers_sandbox_clients_get) | **GET** /enterprise/v1/account/{accountNumber}/routers/sandbox/clients | Get sandbox clients
[**enterprise_v1_account_account_number_routers_sandbox_clients_post**](RouterApi.md#enterprise_v1_account_account_number_routers_sandbox_clients_post) | **POST** /enterprise/v1/account/{accountNumber}/routers/sandbox/clients | Batch update sandbox clients
[**enterprise_v1_account_account_number_routers_sandbox_heartbeat_put**](RouterApi.md#enterprise_v1_account_account_number_routers_sandbox_heartbeat_put) | **PUT** /enterprise/v1/account/{accountNumber}/routers/sandbox/heartbeat | Update sandbox heartbeat
[**enterprise_v1_account_account_number_routers_sandbox_sandbox_id_put**](RouterApi.md#enterprise_v1_account_account_number_routers_sandbox_sandbox_id_put) | **PUT** /enterprise/v1/account/{accountNumber}/routers/sandbox/{sandboxId} | Update sandbox
[**enterprise_v1_account_account_number_routers_tls_config_delete**](RouterApi.md#enterprise_v1_account_account_number_routers_tls_config_delete) | **DELETE** /enterprise/v1/account/{accountNumber}/routers/tls-config | Delete tls config.
[**enterprise_v1_account_account_number_routers_tls_config_post**](RouterApi.md#enterprise_v1_account_account_number_routers_tls_config_post) | **POST** /enterprise/v1/account/{accountNumber}/routers/tls-config | Create TLS config
[**enterprise_v1_account_account_number_routers_tls_configs_get**](RouterApi.md#enterprise_v1_account_account_number_routers_tls_configs_get) | **GET** /enterprise/v1/account/{accountNumber}/routers/tls-configs | Get TLS configs



## enterprise_v1_account_account_number_routers_batch_config_put

> models::ServiceResponse enterprise_v1_account_account_number_routers_batch_config_put(account_number, update_batch_device_config_request)
Batch config assignment

Assign the config (or none) to the routers. For each router if it is currently online, the config will immediately be sent to the router.  Else, the config will be sent to the router when it comes online. Configs are sent to the router within 1-2 minutes. On error no assignment occurs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of router | [required] |
**update_batch_device_config_request** | Option<[**UpdateBatchDeviceConfigRequest**](UpdateBatchDeviceConfigRequest.md)> | Request containing config id (or empty) and list of deviceId's to assign to |  |

### Return type

[**models::ServiceResponse**](ServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_routers_configs_config_id_get

> models::RouterConfigResponseServiceResponse enterprise_v1_account_account_number_routers_configs_config_id_get(account_number, config_id)
Get router config

@permission [RequireCustomerPermission] feature:DeviceCommand, permission:View

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of router config | [required] |
**config_id** | **String** | Config Id | [required] |

### Return type

[**models::RouterConfigResponseServiceResponse**](RouterConfigResponseServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_routers_configs_config_id_put

> models::RouterConfigResponseServiceResponse enterprise_v1_account_account_number_routers_configs_config_id_put(account_number, config_id, router_config_request)
Update router config

Update a given router config. Any router assigned to this config will immediately receive the update if they are online.  Otherwise, the router will receive the update when it comes online.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of router config | [required] |
**config_id** | **String** | Config Id | [required] |
**router_config_request** | Option<[**RouterConfigRequest**](RouterConfigRequest.md)> | RouterConfigRequest |  |

### Return type

[**models::RouterConfigResponseServiceResponse**](RouterConfigResponseServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_routers_configs_get

> models::RouterConfigResponsePaginatedServiceResponse enterprise_v1_account_account_number_routers_configs_get(account_number, page)
Get all router configs

@permission [RequireCustomerPermission] feature:DeviceCommand, permission:View

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number | [required] |
**page** | Option<**i32**> | Page to get |  |[default to 0]

### Return type

[**models::RouterConfigResponsePaginatedServiceResponse**](RouterConfigResponsePaginatedServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_routers_configs_post

> models::RouterConfigResponseServiceResponse enterprise_v1_account_account_number_routers_configs_post(account_number, router_config_request)
Create router config

@permission [RequireCustomerPermission] feature:DeviceCommand, permission:Edit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of router config | [required] |
**router_config_request** | Option<[**RouterConfigRequest**](RouterConfigRequest.md)> | RouterConfigRequest |  |

### Return type

[**models::RouterConfigResponseServiceResponse**](RouterConfigResponseServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_routers_router_id_config_delete

> models::ServiceResponse enterprise_v1_account_account_number_routers_router_id_config_delete(account_number, router_id)
Remove config assignment

Remove the config assignment on the router. The config itself will still exist. The router will no longer  get its config from the cloud, and its config may be changed in the Starlink app or via factory reset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of router | [required] |
**router_id** | **String** | Router Id | [required] |

### Return type

[**models::ServiceResponse**](ServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_routers_router_id_config_put

> models::ServiceResponse enterprise_v1_account_account_number_routers_router_id_config_put(account_number, router_id, body)
Set config assignment

Assign the config to the router. If the router is currently online, the config will immediately be sent to the router.  Else, the config will be sent to the router when it comes online. Configs are sent to the router within 1-2 minutes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of router | [required] |
**router_id** | **String** | Router Id | [required] |
**body** | Option<**String**> | Config Id |  |

### Return type

[**models::ServiceResponse**](ServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_routers_router_id_get

> models::RouterResponseServiceResponse enterprise_v1_account_account_number_routers_router_id_get(account_number, router_id)
Get router

@permission [RequireCustomerPermission] feature:DeviceCommand, permission:View

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of router | [required] |
**router_id** | **String** | Router Id | [required] |

### Return type

[**models::RouterResponseServiceResponse**](RouterResponseServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_routers_router_id_reboot_post

> models::ServiceResponse enterprise_v1_account_account_number_routers_router_id_reboot_post(account_number, router_id)
Reboot router

@permission [RequireCustomerPermission] feature:DeviceCommand, permission:Edit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of router | [required] |
**router_id** | **String** | Router Id | [required] |

### Return type

[**models::ServiceResponse**](ServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_routers_sandbox_client_post

> models::ServiceResponse enterprise_v1_account_account_number_routers_sandbox_client_post(account_number, update_sandbox_client_request)
Update sandbox client

Update the sandbox state for a client. If sandboxing is enabled, sanboxed client will only have access  to domains in the sandbox domain allow list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of sandbox | [required] |
**update_sandbox_client_request** | Option<[**UpdateSandboxClientRequest**](UpdateSandboxClientRequest.md)> | UpdateSandboxClientRequest |  |

### Return type

[**models::ServiceResponse**](ServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_routers_sandbox_clients_get

> models::ServiceResponse enterprise_v1_account_account_number_routers_sandbox_clients_get(account_number, sandbox_id, expiry_after, page, limit)
Get sandbox clients

Returns clients that were unsandboxed through the management API and have access that  has not yet expired. Clients that are allowed internet access due to sandbox disablement  are not returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of sandbox | [required] |
**sandbox_id** | Option<**i32**> | Sandbox Id |  |
**expiry_after** | Option<**String**> | Include clients whose access expires after this time |  |
**page** | Option<**i32**> | Page to get |  |[default to 0]
**limit** | Option<**i32**> | The number of sandbox clients to return at a time |  |[default to 100]

### Return type

[**models::ServiceResponse**](ServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_routers_sandbox_clients_post

> models::ServiceResponse enterprise_v1_account_account_number_routers_sandbox_clients_post(account_number, update_batch_sandbox_client_request)
Batch update sandbox clients

Update the sandbox state for multiple clients. If sandboxing is enabled, sandboxed clients will only have access  to domains in the sandbox domain allow list.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of sandbox | [required] |
**update_batch_sandbox_client_request** | Option<[**Vec<models::UpdateBatchSandboxClientRequest>**](UpdateBatchSandboxClientRequest.md)> | UpdateSandboxClientRequest |  |

### Return type

[**models::ServiceResponse**](ServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_routers_sandbox_heartbeat_put

> models::ServiceResponse enterprise_v1_account_account_number_routers_sandbox_heartbeat_put(account_number, sandbox_heartbeat_request)
Update sandbox heartbeat

Heartbeats verify the health of enterprise systems that manage router sandboxing.  If heartbeats are not received for an account, Starlink API will instruct routers  under the account disable sandboxes until reboot.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number | [required] |
**sandbox_heartbeat_request** | Option<[**SandboxHeartbeatRequest**](SandboxHeartbeatRequest.md)> | SandboxHeartbeatRequest |  |

### Return type

[**models::ServiceResponse**](ServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_routers_sandbox_sandbox_id_put

> models::UpdateSandboxResponseServiceResponse enterprise_v1_account_account_number_routers_sandbox_sandbox_id_put(account_number, sandbox_id, update_sandbox_request)
Update sandbox

Enable or disable a client sandbox. This applies to all router configs the sandbox ID is present in.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of sandbox | [required] |
**sandbox_id** | **i32** | Sandbox Id | [required] |
**update_sandbox_request** | Option<[**UpdateSandboxRequest**](UpdateSandboxRequest.md)> | UpdateSandboxRequest |  |

### Return type

[**models::UpdateSandboxResponseServiceResponse**](UpdateSandboxResponseServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_routers_tls_config_delete

> models::TlsConfigPublicResponseServiceResponse enterprise_v1_account_account_number_routers_tls_config_delete(account_number, body)
Delete tls config.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of the tls configs | [required] |
**body** | Option<**String**> | Tls config identifier |  |

### Return type

[**models::TlsConfigPublicResponseServiceResponse**](TlsConfigPublicResponseServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_routers_tls_config_post

> models::TlsConfigPublicResponseServiceResponse enterprise_v1_account_account_number_routers_tls_config_post(account_number, tls_config_create_request)
Create TLS config

Tls configurations can be used by routers to host a HTTPS server.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of tls config | [required] |
**tls_config_create_request** | Option<[**TlsConfigCreateRequest**](TlsConfigCreateRequest.md)> | TlsConfigCreateRequest |  |

### Return type

[**models::TlsConfigPublicResponseServiceResponse**](TlsConfigPublicResponseServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_routers_tls_configs_get

> models::TlsConfigPublicResponseServiceResponse enterprise_v1_account_account_number_routers_tls_configs_get(account_number, domain)
Get TLS configs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of the tls configs | [required] |
**domain** | Option<**String**> | Domain name |  |

### Return type

[**models::TlsConfigPublicResponseServiceResponse**](TlsConfigPublicResponseServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

