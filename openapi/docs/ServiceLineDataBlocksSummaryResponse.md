# ServiceLineDataBlocksSummaryResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**recurring_blocks_current_billing_cycle** | Option<[**Vec<models::DataBlockSummaryResponse>**](DataBlockSummaryResponse.md)> | The current billing cycle's monthly recurring data blocks. | [optional]
**recurring_blocks_next_billing_cycle** | Option<[**Vec<models::DataBlockSummaryResponse>**](DataBlockSummaryResponse.md)> | Next billing cycle's monthly recurring data blocks. | [optional]
**delayed_product_recurring_blocks_next_cycle** | Option<[**Vec<models::DataBlockSummaryResponse>**](DataBlockSummaryResponse.md)> | What recurring blocks will be when the service line switches to the delayed product. | [optional]
**top_up_blocks_opt_in_purchase** | Option<[**Vec<models::DataBlockSummaryResponse>**](DataBlockSummaryResponse.md)> | What top up blocks were automatically purchased because the service line was out of data and opted in. | [optional]
**top_up_blocks_one_time_purchase** | Option<[**Vec<models::DataBlockSummaryResponse>**](DataBlockSummaryResponse.md)> | What top up blocks were manually purchased. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


