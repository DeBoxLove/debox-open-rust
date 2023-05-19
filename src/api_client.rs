use serde_json::json;

use crate::client::Client;

// result schema { code: 200, data: null, message: 'success', success: true }
#[derive(Clone, Debug)]
pub struct ApiResult {
    pub code: u32,
    pub data: serde_json::Value,
    pub message: String,
    pub success: bool,
}

// body schema { "url": "http://xxx.com", "http_method": "POST" }
#[derive(Clone, Debug)]
pub struct RegisterCallbackUrlBody {
    pub url: String,
    pub http_method: String,
}

// body schema { groupId: '111',toUserId: 'DeBox.Love',message: 'Hello World',}
pub struct SendChatMsgBody {
    pub group_id: String,
    pub to_user_id: String,
    pub message: String,
}

impl Client {
    pub async fn register_callbak_url(
        &self,
        body: &RegisterCallbackUrlBody,
    ) -> Result<ApiResult, reqwest::Error> {
        let url: &str = "openapi/register_callbak_url";
        let send_body = json!({
          "url": body.url.clone(),
          "http_method": body.http_method.clone(),
        });
        let res_value = self.post(&url, &send_body).await?;
        let res = ApiResult {
            code: res_value["code"].as_u64().unwrap() as u32,
            data: res_value["data"].clone(),
            message: res_value["message"].as_str().unwrap().to_string(),
            success: res_value["success"].as_bool().unwrap_or_default(),
        };
        Ok(res)
    }

    pub async fn send_chat_msg(&self, body: &SendChatMsgBody) -> Result<ApiResult, reqwest::Error> {
        let send_body = json!({
          "group_id": body.group_id.clone(),
          "to_user_id": body.to_user_id.clone(),
          "message": body.message.clone(),
        });
        let url: &str = "openapi/send_chat_message";
        let res_value = self.post(&url, &send_body).await?;
        let res = ApiResult {
            code: res_value["code"].as_u64().unwrap() as u32,
            data: res_value["data"].clone(),
            message: res_value["message"].as_str().unwrap().to_string(),
            success: res_value["success"].as_bool().unwrap_or_default(),
        };
        Ok(res)
    }
}
