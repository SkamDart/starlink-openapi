# DataServicePlan

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**iso_currency_code** | **String** |  | 
**is_mobile_plan** | **bool** |  | 
**active_from** | Option<**String**> |  | [optional]
**subscription_active_from** | Option<**String**> |  | [optional]
**subscription_end_date** | Option<**String**> |  | [optional]
**overage_name** | Option<**String**> |  | [optional]
**overage_description** | Option<**String**> |  | [optional]
**is_opted_into_overage** | **bool** |  | 
**overage_line_deactivated_date** | Option<**String**> |  | [optional]
**overage_line** | Option<[**models::DataUsageOverageLine**](DataUsageOverageLine.md)> |  | [optional]
**data_pool_usage** | Option<[**models::DataPoolUsagePublicResponse**](DataPoolUsagePublicResponse.md)> |  | [optional]
**product_id** | **String** |  | 
**usage_limit_gb** | **f64** |  | 
**data_category_mapping** | [**std::collections::HashMap<String, models::DataBucketType>**](DataBucketType.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


