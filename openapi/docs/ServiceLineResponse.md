# ServiceLineResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address_reference_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Address Reference ID of the address associated with the service line. Example: 55ec6574-10d8-bd9c-1951-d4184f4ae467 | [optional]
**service_line_number** | Option<**String**> | The Service Line Number. Example: AST-511274-31364-54 | [optional]
**nickname** | Option<**String**> | A user-defined nickname for this service line | [optional]
**product_reference_id** | Option<**String**> | The unique product identifier | [optional]
**delayed_product_id** | Option<**String**> | Scheduled product change for next bill date | [optional]
**opt_in_product_id** | Option<**String**> | Opt-in product id, opted out if empty | [optional]
**start_date** | Option<**String**> | The start date of the subscription. This is in UTC. | [optional]
**end_date** | Option<**String**> | The service line deactivation date, which only appears if the service line is deactivated. This is in UTC. | [optional]
**public_ip** | Option<**bool**> | Indicates if service line is public IP | [optional]
**active** | Option<**bool**> | Indicates if service line is active | [optional]
**aviation_metadata** | Option<[**models::AviationMetadataResponse**](AviationMetadataResponse.md)> |  | [optional]
**data_blocks** | Option<[**models::ServiceLineDataBlocksSummaryResponse**](ServiceLineDataBlocksSummaryResponse.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


