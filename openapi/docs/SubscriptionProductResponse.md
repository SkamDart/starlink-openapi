# SubscriptionProductResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**product_reference_id** | Option<**String**> | The unique product identifier | [optional]
**name** | Option<**String**> | Name of the product | [optional]
**price** | Option<**f64**> | Monthly price | [optional]
**iso_currency_code** | Option<**String**> | Currency code for the price | [optional]
**is_sla** | Option<**bool**> | If Product Has SLA | [optional]
**max_number_of_user_terminals** | Option<**i32**> | The maximum number of user terminals that can be assigned to a single service line using this product. Null response means an unlimited number of UTs can be assigned. | [optional]
**data_products** | Option<[**models::DataProductsResponse**](DataProductsResponse.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


