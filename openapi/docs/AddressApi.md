# \AddressApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**enterprise_v1_account_account_number_addresses_address_reference_id_get**](AddressApi.md#enterprise_v1_account_account_number_addresses_address_reference_id_get) | **GET** /enterprise/v1/account/{accountNumber}/addresses/{addressReferenceId} | Get address
[**enterprise_v1_account_account_number_addresses_check_capacity_post**](AddressApi.md#enterprise_v1_account_account_number_addresses_check_capacity_post) | **POST** /enterprise/v1/account/{accountNumber}/addresses/check-capacity | Check capacity
[**enterprise_v1_account_account_number_addresses_get**](AddressApi.md#enterprise_v1_account_account_number_addresses_get) | **GET** /enterprise/v1/account/{accountNumber}/addresses | Get all addresses
[**enterprise_v1_account_account_number_addresses_post**](AddressApi.md#enterprise_v1_account_account_number_addresses_post) | **POST** /enterprise/v1/account/{accountNumber}/addresses | Create address
[**enterprise_v1_account_account_number_addresses_put**](AddressApi.md#enterprise_v1_account_account_number_addresses_put) | **PUT** /enterprise/v1/account/{accountNumber}/addresses | Update address



## enterprise_v1_account_account_number_addresses_address_reference_id_get

> models::AddressResponseServiceResponse enterprise_v1_account_account_number_addresses_address_reference_id_get(account_number, address_reference_id)
Get address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of address | [required] |
**address_reference_id** | **uuid::Uuid** | Address reference ID | [required] |

### Return type

[**models::AddressResponseServiceResponse**](AddressResponseServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_addresses_check_capacity_post

> models::CapacityCheckResponseServiceResponse enterprise_v1_account_account_number_addresses_check_capacity_post(account_number, capacity_check_request)
Check capacity

Check Starlink network capacity at a specific coordinate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number | [required] |
**capacity_check_request** | Option<[**CapacityCheckRequest**](CapacityCheckRequest.md)> | CapacityCheckRequest |  |

### Return type

[**models::CapacityCheckResponseServiceResponse**](CapacityCheckResponseServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_addresses_get

> models::AddressResponsePaginatedServiceResponse enterprise_v1_account_account_number_addresses_get(account_number, address_ids, metadata, limit, page)
Get all addresses

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number | [required] |
**address_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Filter by a specific set of Address Reference IDs |  |
**metadata** | Option<**String**> | Filter by metadata |  |
**limit** | Option<**i32**> | The number of addresses to return at a time |  |[default to 50]
**page** | Option<**i32**> | The index of the page, starting at 0 |  |[default to 0]

### Return type

[**models::AddressResponsePaginatedServiceResponse**](AddressResponsePaginatedServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_addresses_post

> models::AddressResponseServiceResponse enterprise_v1_account_account_number_addresses_post(account_number, address_create_request)
Create address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number | [required] |
**address_create_request** | Option<[**AddressCreateRequest**](AddressCreateRequest.md)> | AddressCreateRequest |  |

### Return type

[**models::AddressResponseServiceResponse**](AddressResponseServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_addresses_put

> models::AddressResponseServiceResponse enterprise_v1_account_account_number_addresses_put(account_number, address_update_request)
Update address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of address | [required] |
**address_update_request** | Option<[**AddressUpdateRequest**](AddressUpdateRequest.md)> | AddressUpdateRequest |  |

### Return type

[**models::AddressResponseServiceResponse**](AddressResponseServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

