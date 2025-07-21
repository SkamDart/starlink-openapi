# DataUsageBillingCycleV2

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start_date** | Option<**String**> |  | [optional]
**end_date** | Option<**String**> |  | [optional]
**daily_data_usage** | Option<[**Vec<models::DataUsageDailyV2>**](DataUsageDailyV2.md)> |  | [optional]
**overage_lines** | Option<[**Vec<models::DataUsageOverageLine>**](DataUsageOverageLine.md)> |  | [optional]
**data_pool_usage** | Option<[**Vec<models::DataPoolUsagePublicResponse>**](DataPoolUsagePublicResponse.md)> |  | [optional]
**total_priority_gb** | Option<**f64**> |  | [optional][readonly]
**total_standard_gb** | Option<**f64**> |  | [optional][readonly]
**total_opt_in_priority_gb** | Option<**f64**> |  | [optional][readonly]
**total_non_billable_gb** | Option<**f64**> |  | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


