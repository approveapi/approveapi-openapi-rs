# CreatePromptRequest

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**body** | **String** | The body of the approval request to show the user. | 
**title** | **String** | The title of an approval request. Defaults to an empty string. | [optional] 
**reject_text** | **String** | The reject action text. Defaults to 'Reject'. | [optional] 
**expires_in** | **f32** | The number of seconds until this request can no longer be answered. | [optional] 
**long_poll** | **bool** | If true, the request waits (long-polls) until the user responds to the prompt or more than 10 minutes pass. Defaults to false. | [optional] 
**user** | **String** | The user to send the approval request to. Can be either an email address or a phone number. | 
**approve_text** | **String** | The approve action text. Defaults to 'Approve'. | [optional] 
**metadata** | [***::models::PromptMetadata**](PromptMetadata.md) |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


