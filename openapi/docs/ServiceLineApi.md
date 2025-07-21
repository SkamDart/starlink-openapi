# \ServiceLineApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**enterprise_v1_account_account_number_service_lines_available_products_get**](ServiceLineApi.md#enterprise_v1_account_account_number_service_lines_available_products_get) | **GET** /enterprise/v1/account/{accountNumber}/service-lines/available-products | Get available products
[**enterprise_v1_account_account_number_service_lines_get**](ServiceLineApi.md#enterprise_v1_account_account_number_service_lines_get) | **GET** /enterprise/v1/account/{accountNumber}/service-lines | Get all service lines
[**enterprise_v1_account_account_number_service_lines_post**](ServiceLineApi.md#enterprise_v1_account_account_number_service_lines_post) | **POST** /enterprise/v1/account/{accountNumber}/service-lines | Create service line
[**enterprise_v1_account_account_number_service_lines_service_line_number_billing_cycle_all_get**](ServiceLineApi.md#enterprise_v1_account_account_number_service_lines_service_line_number_billing_cycle_all_get) | **GET** /enterprise/v1/account/{accountNumber}/service-lines/{serviceLineNumber}/billing-cycle/all | Get data usage
[**enterprise_v1_account_account_number_service_lines_service_line_number_billing_cycle_partial_periods_get**](ServiceLineApi.md#enterprise_v1_account_account_number_service_lines_service_line_number_billing_cycle_partial_periods_get) | **GET** /enterprise/v1/account/{accountNumber}/service-lines/{serviceLineNumber}/billing-cycle/partial-periods | Get partial periods
[**enterprise_v1_account_account_number_service_lines_service_line_number_delete**](ServiceLineApi.md#enterprise_v1_account_account_number_service_lines_service_line_number_delete) | **DELETE** /enterprise/v1/account/{accountNumber}/service-lines/{serviceLineNumber} | Deactivate a service line
[**enterprise_v1_account_account_number_service_lines_service_line_number_get**](ServiceLineApi.md#enterprise_v1_account_account_number_service_lines_service_line_number_get) | **GET** /enterprise/v1/account/{accountNumber}/service-lines/{serviceLineNumber} | Get service line
[**enterprise_v1_account_account_number_service_lines_service_line_number_nickname_put**](ServiceLineApi.md#enterprise_v1_account_account_number_service_lines_service_line_number_nickname_put) | **PUT** /enterprise/v1/account/{accountNumber}/service-lines/{serviceLineNumber}/nickname | Update nickname
[**enterprise_v1_account_account_number_service_lines_service_line_number_opt_in_post**](ServiceLineApi.md#enterprise_v1_account_account_number_service_lines_service_line_number_opt_in_post) | **POST** /enterprise/v1/account/{accountNumber}/service-lines/{serviceLineNumber}/opt-in | Opt in
[**enterprise_v1_account_account_number_service_lines_service_line_number_opt_out_delete**](ServiceLineApi.md#enterprise_v1_account_account_number_service_lines_service_line_number_opt_out_delete) | **DELETE** /enterprise/v1/account/{accountNumber}/service-lines/{serviceLineNumber}/opt-out | Opt out
[**enterprise_v1_account_account_number_service_lines_service_line_number_product_product_reference_id_post**](ServiceLineApi.md#enterprise_v1_account_account_number_service_lines_service_line_number_product_product_reference_id_post) | **POST** /enterprise/v1/account/{accountNumber}/service-lines/{serviceLineNumber}/product/{productReferenceId} | Update product
[**enterprise_v1_account_account_number_service_lines_service_line_number_product_product_reference_id_put**](ServiceLineApi.md#enterprise_v1_account_account_number_service_lines_service_line_number_product_product_reference_id_put) | **PUT** /enterprise/v1/account/{accountNumber}/service-lines/{serviceLineNumber}/product/{productReferenceId} | Update product
[**enterprise_v1_account_account_number_service_lines_service_line_number_public_ip_put**](ServiceLineApi.md#enterprise_v1_account_account_number_service_lines_service_line_number_public_ip_put) | **PUT** /enterprise/v1/account/{accountNumber}/service-lines/{serviceLineNumber}/public-ip | Set public IP
[**enterprise_v1_account_account_number_service_lines_service_line_number_recurring_data_put**](ServiceLineApi.md#enterprise_v1_account_account_number_service_lines_service_line_number_recurring_data_put) | **PUT** /enterprise/v1/account/{accountNumber}/service-lines/{serviceLineNumber}/recurring-data | Sets the recurring blocks on a service line if it is on a top-up plan.
[**enterprise_v1_account_account_number_service_lines_service_line_number_top_up_data_post**](ServiceLineApi.md#enterprise_v1_account_account_number_service_lines_service_line_number_top_up_data_post) | **POST** /enterprise/v1/account/{accountNumber}/service-lines/{serviceLineNumber}/top-up-data | Adds a one-time top up data block to a service line if it is on a compatible plan.



## enterprise_v1_account_account_number_service_lines_available_products_get

> models::SubscriptionProductResponsePaginatedServiceResponse enterprise_v1_account_account_number_service_lines_available_products_get(account_number, limit, page)
Get available products

@permission [RequireCustomerPermission] feature:ServicePlan, permission:View

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of service line | [required] |
**limit** | Option<**i32**> | The number of subscriptions to return at a time |  |[default to 50]
**page** | Option<**i32**> | The index of the page, starting at 0 |  |[default to 0]

### Return type

[**models::SubscriptionProductResponsePaginatedServiceResponse**](SubscriptionProductResponsePaginatedServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_service_lines_get

> models::ServiceLineResponsePaginatedServiceResponse enterprise_v1_account_account_number_service_lines_get(account_number, address_reference_id, search_string, limit, page, order_by_created_date_descending)
Get all service lines

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of service line | [required] |
**address_reference_id** | Option<**uuid::Uuid**> | Filter by an Address Reference ID |  |
**search_string** | Option<**String**> | Filter by fuzzy match of nickname, or exact match on UT ID, UT nickname, UT serial number, or service line number |  |
**limit** | Option<**i32**> | The number of service lines to return at a time |  |[default to 50]
**page** | Option<**i32**> | The index of the page, starting at 0 |  |[default to 0]
**order_by_created_date_descending** | Option<**bool**> | Sort the paginated results by created date |  |[default to true]

### Return type

[**models::ServiceLineResponsePaginatedServiceResponse**](ServiceLineResponsePaginatedServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_service_lines_post

> models::ServiceLineResponseServiceResponse enterprise_v1_account_account_number_service_lines_post(account_number, service_line_create_request)
Create service line

Create a service line. This must be linked to an address and a subscription/product-ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of service line | [required] |
**service_line_create_request** | Option<[**ServiceLineCreateRequest**](ServiceLineCreateRequest.md)> | ServiceLineCreateRequest |  |

### Return type

[**models::ServiceLineResponseServiceResponse**](ServiceLineResponseServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_service_lines_service_line_number_billing_cycle_all_get

> models::DataUsageResponseServiceResponse enterprise_v1_account_account_number_service_lines_service_line_number_billing_cycle_all_get(account_number, service_line_number, include_unknown_data_bin)
Get data usage

@permission [RequireCustomerPermission] feature:DeviceTelemetry, permission:View

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of service line | [required] |
**service_line_number** | **String** | Service line number | [required] |
**include_unknown_data_bin** | Option<**bool**> | When true; unknown data bins will be included in results. |  |[default to false]

### Return type

[**models::DataUsageResponseServiceResponse**](DataUsageResponseServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_service_lines_service_line_number_billing_cycle_partial_periods_get

> models::PartialPeriodResponseIEnumerableServiceResponse enterprise_v1_account_account_number_service_lines_service_line_number_billing_cycle_partial_periods_get(account_number, service_line_number)
Get partial periods

Get the previous partial periods for this service line. For more information about this endpoint, see https://starlink.readme.io/docs/understanding-proration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of service line | [required] |
**service_line_number** | **String** | Service line number | [required] |

### Return type

[**models::PartialPeriodResponseIEnumerableServiceResponse**](PartialPeriodResponseIEnumerableServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_service_lines_service_line_number_delete

> models::ServiceResponse enterprise_v1_account_account_number_service_lines_service_line_number_delete(account_number, service_line_number, reason_for_cancellation, end_now)
Deactivate a service line

@permission [RequireCustomerPermission] feature:ServicePlan, permission:Edit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of service line | [required] |
**service_line_number** | **String** | Service line number | [required] |
**reason_for_cancellation** | Option<**String**> | Optional reason for cancelling this service line |  |
**end_now** | Option<**bool**> | If service should end now, or on next bill day. Default is false |  |[default to false]

### Return type

[**models::ServiceResponse**](ServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_service_lines_service_line_number_get

> models::ServiceLineResponseServiceResponse enterprise_v1_account_account_number_service_lines_service_line_number_get(account_number, service_line_number)
Get service line

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of service line | [required] |
**service_line_number** | **String** | Service line number | [required] |

### Return type

[**models::ServiceLineResponseServiceResponse**](ServiceLineResponseServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_service_lines_service_line_number_nickname_put

> models::ServiceLineResponseServiceResponse enterprise_v1_account_account_number_service_lines_service_line_number_nickname_put(account_number, service_line_number, service_line_update_nickname_request)
Update nickname

@permission [RequireCustomerPermission] feature:DeviceManagement, permission:Edit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of service line | [required] |
**service_line_number** | **String** | Service line number | [required] |
**service_line_update_nickname_request** | Option<[**ServiceLineUpdateNicknameRequest**](ServiceLineUpdateNicknameRequest.md)> | ServiceLineUpdateNicknameRequest |  |

### Return type

[**models::ServiceLineResponseServiceResponse**](ServiceLineResponseServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_service_lines_service_line_number_opt_in_post

> models::OptInPeriodResponseServiceResponse enterprise_v1_account_account_number_service_lines_service_line_number_opt_in_post(account_number, service_line_number)
Opt in

Opt in the service line to continue using priority data after reaching the plan capacity.  Only applies to some products.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of service line | [required] |
**service_line_number** | **String** | Service line number | [required] |

### Return type

[**models::OptInPeriodResponseServiceResponse**](OptInPeriodResponseServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_service_lines_service_line_number_opt_out_delete

> models::OptInPeriodResponseServiceResponse enterprise_v1_account_account_number_service_lines_service_line_number_opt_out_delete(account_number, service_line_number)
Opt out

Opt out the service line. If the service line reaches the plan capacity, it will switch to using standard data.  Only applies to some products.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of service line | [required] |
**service_line_number** | **String** | Service line number | [required] |

### Return type

[**models::OptInPeriodResponseServiceResponse**](OptInPeriodResponseServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_service_lines_service_line_number_product_product_reference_id_post

> models::ServiceLineResponseServiceResponse enterprise_v1_account_account_number_service_lines_service_line_number_product_product_reference_id_post(account_number, service_line_number, product_reference_id, set_recurring_data_blocks_request)
Update product

@permission [RequireCustomerPermission] feature:ServicePlan, permission:Edit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of service line | [required] |
**service_line_number** | **String** | Service line number | [required] |
**product_reference_id** | **String** | The unique reference id of the product to update to | [required] |
**set_recurring_data_blocks_request** | Option<[**SetRecurringDataBlocksRequest**](SetRecurringDataBlocksRequest.md)> | The recurring data blocks to set on the new product, if it on a compatible product. Leave null if otherwise. |  |

### Return type

[**models::ServiceLineResponseServiceResponse**](ServiceLineResponseServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_service_lines_service_line_number_product_product_reference_id_put

> models::ServiceLineResponseServiceResponse enterprise_v1_account_account_number_service_lines_service_line_number_product_product_reference_id_put(account_number, service_line_number, product_reference_id)
Update product

@permission [RequireCustomerPermission] feature:ServicePlan, permission:Edit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of service line | [required] |
**service_line_number** | **String** | Service line number | [required] |
**product_reference_id** | **String** | The unique reference id of the product to update to | [required] |

### Return type

[**models::ServiceLineResponseServiceResponse**](ServiceLineResponseServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_service_lines_service_line_number_public_ip_put

> models::ServiceResponse enterprise_v1_account_account_number_service_lines_service_line_number_public_ip_put(account_number, service_line_number, service_line_set_public_ip_request)
Set public IP

@permission [RequireCustomerPermission] feature:ServicePlan, permission:Edit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | Account number of service line | [required] |
**service_line_number** | **String** | Service line number | [required] |
**service_line_set_public_ip_request** | Option<[**ServiceLineSetPublicIpRequest**](ServiceLineSetPublicIpRequest.md)> | ServiceLineSetPublicIpRequest |  |

### Return type

[**models::ServiceResponse**](ServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_service_lines_service_line_number_recurring_data_put

> models::ServiceResponse enterprise_v1_account_account_number_service_lines_service_line_number_recurring_data_put(account_number, service_line_number, set_recurring_data_blocks_request)
Sets the recurring blocks on a service line if it is on a top-up plan.

@permission [RequireCustomerPermission] feature:ServicePlan, permission:Edit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | The account number. | [required] |
**service_line_number** | **String** | The service line number. | [required] |
**set_recurring_data_blocks_request** | Option<[**SetRecurringDataBlocksRequest**](SetRecurringDataBlocksRequest.md)> | The request object. |  |

### Return type

[**models::ServiceResponse**](ServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enterprise_v1_account_account_number_service_lines_service_line_number_top_up_data_post

> models::ServiceResponse enterprise_v1_account_account_number_service_lines_service_line_number_top_up_data_post(account_number, service_line_number, public_add_data_block_request)
Adds a one-time top up data block to a service line if it is on a compatible plan.

@permission [RequireCustomerPermission] feature:ServicePlan, permission:Edit

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_number** | **String** | The account number. | [required] |
**service_line_number** | **String** | The service line number. | [required] |
**public_add_data_block_request** | Option<[**PublicAddDataBlockRequest**](PublicAddDataBlockRequest.md)> | The request object. |  |

### Return type

[**models::ServiceResponse**](ServiceResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

