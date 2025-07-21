# \UserTerminalApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**enterprise_v1_account_account_number_user_terminals_batch_config_put**](UserTerminalApi.md#enterprise_v1_account_account_number_user_terminals_batch_config_put) | **PUT** /enterprise/v1/account/{accountNumber}/user-terminals/batch-config | Batch config assignment
[**enterprise_v1_account_account_number_user_terminals_device_id_delete**](UserTerminalApi.md#enterprise_v1_account_account_number_user_terminals_device_id_delete) | **DELETE** /enterprise/v1/account/{accountNumber}/user-terminals/{deviceId} | Remove from account
[**enterprise_v1_account_account_number_user_terminals_device_id_post**](UserTerminalApi.md#enterprise_v1_account_account_number_user_terminals_device_id_post) | **POST** /enterprise/v1/account/{accountNumber}/user-terminals/{deviceId} | Add to account
[**enterprise_v1_account_account_number_user_terminals_device_id_reboot_post**](UserTerminalApi.md#enterprise_v1_account_account_number_user_terminals_device_id_reboot_post) | **POST** /enterprise/v1/account/{accountNumber}/user-terminals/{deviceId}/reboot | Reboot user terminal
[**enterprise_v1_account_account_number_user_terminals_get**](UserTerminalApi.md#enterprise_v1_account_account_number_user_terminals_get) | **GET** /enterprise/v1/account/{accountNumber}/user-terminals | Get all user terminals
[**enterprise_v1_account_account_number_user_terminals_user_terminal_id_service_line_number_delete**](UserTerminalApi.md#enterprise_v1_account_account_number_user_terminals_user_terminal_id_service_line_number_delete) | **DELETE** /enterprise/v1/account/{accountNumber}/user-terminals/{userTerminalId}/{serviceLineNumber} | Remove from service line
[**enterprise_v1_account_account_number_user_terminals_user_terminal_id_service_line_number_post**](UserTerminalApi.md#enterprise_v1_account_account_number_user_terminals_user_terminal_id_service_line_number_post) | **POST** /enterprise/v1/account/{accountNumber}/user-terminals/{userTerminalId}/{serviceLineNumber} | Add to service line



## enterprise_v1_account_account_number_user_terminals_batch_config_put

> models::ServiceResponse enterprise_v1_account_account_number_user_terminals_batch_config_put(account_number, update_batch_device_config_request)
Batch config assignment

Assign the config (or none) to the user terminals. For each terminal if it is currently online, the config will immediately be  sent. Else, the config will be sent when it comes online. On error no assignment occurs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of user terminals | [required] |
**update_batch_device_config_request** | Option<[**UpdateBatchDeviceConfigRequest**](UpdateBatchDeviceConfigRequest.md)> | Request containing config id (or empty) and list of deviceId's to assign to |  |

### Return type

[**models::ServiceResponse**](ServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_user_terminals_device_id_delete

> models::ServiceResponse enterprise_v1_account_account_number_user_terminals_device_id_delete(account_number, device_id)
Remove from account

Remove a user terminal from the account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of user terminal | [required] |
**device_id** | **String** | UTID, kit serial number, or dish serial number. | [required] |

### Return type

[**models::ServiceResponse**](ServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_user_terminals_device_id_post

> models::ServiceResponse enterprise_v1_account_account_number_user_terminals_device_id_post(account_number, device_id)
Add to account

Add a user terminal to an account. This will add the user terminal to the account, but won't start service.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of service line | [required] |
**device_id** | **String** | UTID, kit serial number, or dish serial number. | [required] |

### Return type

[**models::ServiceResponse**](ServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_user_terminals_device_id_reboot_post

> models::ServiceResponse enterprise_v1_account_account_number_user_terminals_device_id_reboot_post(account_number, device_id)
Reboot user terminal

@permission [RequireCustomerPermission] feature:DeviceCommand, permission:Edit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of user terminal | [required] |
**device_id** | **String** | UTID, kit serial number, or dish serial number. | [required] |

### Return type

[**models::ServiceResponse**](ServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_user_terminals_get

> models::UserTerminalResponsePaginatedServiceResponse enterprise_v1_account_account_number_user_terminals_get(account_number, service_line_numbers, user_terminal_ids, has_service_line, active, search_string, limit, page)
Get all user terminals

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of service line | [required] |
**service_line_numbers** | Option<[**Vec<String>**](String.md)> | Filter by a set of service line numbers |  |
**user_terminal_ids** | Option<[**Vec<String>**](String.md)> | Filter by a set of user terminal IDs |  |
**has_service_line** | Option<**bool**> | Filter by user terminals with or without a services lines. Omitting this will return both sets |  |
**active** | Option<**bool**> | only return UTs that are active on this account |  |
**search_string** | Option<**String**> | Filter by partial match of UT ID, serial number, or kit serial number |  |
**limit** | Option<**i32**> | The number of user terminals to return at a time |  |[default to 50]
**page** | Option<**i32**> | The index of the page, starting at 0 |  |[default to 0]

### Return type

[**models::UserTerminalResponsePaginatedServiceResponse**](UserTerminalResponsePaginatedServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_user_terminals_user_terminal_id_service_line_number_delete

> models::ServiceResponse enterprise_v1_account_account_number_user_terminals_user_terminal_id_service_line_number_delete(account_number, user_terminal_id, service_line_number)
Remove from service line

Remove a user terminal from a service line.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of service line | [required] |
**user_terminal_id** | **String** | User terminal ID | [required] |
**service_line_number** | **String** | Service line number | [required] |

### Return type

[**models::ServiceResponse**](ServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_user_terminals_user_terminal_id_service_line_number_post

> models::ServiceResponse enterprise_v1_account_account_number_user_terminals_user_terminal_id_service_line_number_post(account_number, user_terminal_id, service_line_number)
Add to service line

Add a user terminal to a service line.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of service line | [required] |
**user_terminal_id** | **String** | User terminal ID | [required] |
**service_line_number** | **String** | Service line number | [required] |

### Return type

[**models::ServiceResponse**](ServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

