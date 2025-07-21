# AddressResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address_reference_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Reference identifier for address. Example: 55ec6574-10d8-bd9c-1951-d4184f4ae467 | [optional]
**address_lines** | Option<**Vec<String>**> | The address lines for the address. Example: 1 Rocket Road | [optional]
**locality** | Option<**String**> | The town/locality of the address. Example: Hawthorne | [optional]
**administrative_area** | Option<**String**> | The administrative area of the address. Example: California | [optional]
**administrative_area_code** | Option<**String**> | The administrative area code of the address. Example: CA | [optional]
**region** | Option<**String**> | The region of the address. Example: United States | [optional]
**region_code** | Option<**String**> | The region code of the address. Example: US | [optional]
**postal_code** | Option<**String**> | The postal code of the address. Example: 90250 | [optional]
**metadata** | Option<**String**> | Optional field that can be used to store information about the address for external purposes | [optional]
**formatted_address** | Option<**String**> | A user readable address. Example: 1 Rocket Road, Hawthorne, CA 90250-6844, US | [optional]
**latitude** | Option<**f64**> | The latitude of the address: Example 33.92 | [optional]
**longitude** | Option<**f64**> | The longitude of the address. Example: -118.32 | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


