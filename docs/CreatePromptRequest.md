# CreatePromptRequest

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user** | **String** | The user to send the approval request to. Can be either an email address or a phone number. | 
**body** | **String** | The body of the approval request to show the user. | 
**title** | **String** | The title of an approval request. Defaults to an empty string. | [optional] 
**approve_text** | **String** | The approve action text. Defaults to 'Approve'. | [optional] 
**approve_redirect_url** | **String** | An HTTPS URL to redirect the user to if the prompt is approved. This URL is kept secret until the user is redirected to it. | [optional] 
**reject_text** | **String** | The reject action text. If not specified the reject button will NOT be rendered, and the user will only see an approve action button. | [optional] 
**reject_redirect_url** | **String** | An HTTPS URL to redirect the user to if the prompt is rejected. This URL is kept secret until the user is redirected to it. | [optional] 
**long_poll** | **bool** | If true, the request waits (long-polls) until the user responds to the prompt or more than 10 minutes pass. Defaults to false. | [optional] 
**expires_in** | **f32** | The number of seconds until this request can no longer be answered. | [optional] 
**metadata** | [***::models::PromptMetadata**](PromptMetadata.md) |  | [optional] 
**internal_data** | **::std::collections::HashMap<String, String>** |  | [optional] 
**idempotency_key** | **String** | Allows calling `create_prompt` multiple times idempotently, such that a prompt is sent at-most once. This key should contain sufficient randomness. Idempotent requests are stored for 24 hours. After that time, the same key will create a new request. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


