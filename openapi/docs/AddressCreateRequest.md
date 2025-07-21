# AddressCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address_lines** | **Vec<String>** | The address lines for the address. Example: 1 Rocket Road | 
**locality** | Option<**String**> | The town/locality of the address. Example: Hawthorne | [optional]
**administrative_area** | Option<**String**> | The administrative area of the address. Example: California | [optional]
**administrative_area_code** | **String** | The administrative area code of the address. Example: CA | 
**region** | Option<**String**> | The region of the address. Example: United States | [optional]
**region_code** | **String** | The region code of the address. Example: US | 
**postal_code** | Option<**String**> | The postal code of the address. Example: 90250-6844 | [optional]
**metadata** | Option<**String**> | Optional field that can be used to store information about the address for external purposes | [optional]
**formatted_address** | **String** | A user readable address. Example: 1 Rocket Road, Hawthorne, CA 90250-6844, US | 
**latitude** | **f64** | The required latitude of the address. Example: 33.92 | 
**longitude** | **f64** | The required longitude of the address. Example: -118.32 | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


