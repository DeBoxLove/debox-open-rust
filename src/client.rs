pub struct ClientOptions {
    pub endpoint: String,
    pub api_key: String,
    pub user_agent: Option<String>,
    pub request_time_out: Option<u32>,
    // retry_times: Option<u32>,
    pub auth_version: Option<String>,
}

pub struct Client {
    client: reqwest::Client,
    endpoint: String,
    api_key: String,
    user_agent: String,
    // retry_times: u32, // TODO
    auth_version: String,
}

impl Client {
    pub fn new(opt: &ClientOptions) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(
                opt.request_time_out.unwrap_or(30).into(),
            ))
            .build()
            .unwrap();
        let user_agent = opt
            .user_agent
            .clone()
            .unwrap_or("@deboxdao/debox-open-sdk".to_string());
        let auth_version = opt.auth_version.clone().unwrap_or("0.6.0".to_string());
        // let retry_times = opt.retry_times.unwrap_or(3);
        Client {
            client,
            endpoint: opt.endpoint.clone(),
            api_key: opt.api_key.clone(),
            user_agent,
            // retry_times,
            auth_version,
        }
    }

    pub async fn post(
        &self,
        path: &str,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let echo_json: serde_json::Value = self
            .client
            .post(format!("{}/{}", self.endpoint, path))
            .header(
                "X-Chat-Bodyrawsize",
                serde_json::to_string(body).unwrap().len().to_string(),
            )
            .header("X-Api-Key", self.api_key.clone())
            .header("X-Chat-Apiversion", self.auth_version.clone())
            .header("User-Agent", self.user_agent.clone())
            .header("Content-Type", "application/json")
            .header("Accept-Encoding", "deflate")
            .json(body)
            .send()
            .await?
            .json()
            .await?;
        Ok(echo_json)
    }
}
