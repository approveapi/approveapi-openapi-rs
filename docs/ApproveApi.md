# \ApproveApi

All URIs are relative to *https://approve.sh*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_prompt**](ApproveApi.md#create_prompt) | **POST** /prompt | Sending a prompt
[**get_prompt**](ApproveApi.md#get_prompt) | **GET** /prompt/{id} | Retrieve a prompt
[**get_prompt_status**](ApproveApi.md#get_prompt_status) | **GET** /prompt/{id}/status | Check prompt status


# **create_prompt**
> ::models::Prompt create_prompt(ctx, create_prompt_request)
Sending a prompt

Creates a prompt and pushes it to the user (sends via email, sms, or other supported protocols).

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **create_prompt_request** | [**CreatePromptRequest**](CreatePromptRequest.md)|  | 

### Return type

[**::models::Prompt**](Prompt.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_prompt**
> ::models::Prompt get_prompt(ctx, id, optional)
Retrieve a prompt

Retrieve the prompt object with the given ID.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **id** | **String**| The identifier for a pending or completed prompt. This is returned when you create a prompt. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **id** | **String**| The identifier for a pending or completed prompt. This is returned when you create a prompt. | 
 **long_poll** | **bool**| If true, the request waits (long-polls) until the user responds to the prompt or more than 10 minutes pass. Defaults to false. | 

### Return type

[**::models::Prompt**](Prompt.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_prompt_status**
> ::models::PromptStatus get_prompt_status(id)
Check prompt status

Returns whether a prompt has been completed by the user. This request does not require authentication, and so can be used client-side without sharing API credentials.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **id** | **String**| The prompt identifier. | 

### Return type

[**::models::PromptStatus**](PromptStatus.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

