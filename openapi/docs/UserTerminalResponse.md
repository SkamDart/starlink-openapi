# UserTerminalResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_terminal_id** | Option<**String**> | User Terminal ID. This ID is not printed anywhere on the kit or hardware. Example: 00020900-002220cc-225b9199 | [optional]
**nickname** | Option<**String**> | Nickname of the user terminal. | [optional]
**kit_serial_number** | Option<**String**> | Kit Serial Number. This ID can be found on the box that the hardware came in. Example: KIT00142069 | [optional]
**dish_serial_number** | Option<**String**> | Dish Serial Number. This ID can be found on the dish itself. Example: 2DHT00542069 | [optional]
**service_line_number** | Option<**String**> | The service line the user terminal is assoicated with if it has service, Example: AST-511274-31364-54 | [optional]
**active** | Option<**bool**> | Indicates if user terminal is active | [optional]
**routers** | Option<[**Vec<models::RouterResponse>**](RouterResponse.md)> | Routers currently bonded to this UT | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


