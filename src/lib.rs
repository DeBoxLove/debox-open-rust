//! # debox-open-sdk
//!
//! This project is the programming interface of Debox social chat service (Chat Service) API, the encapsulation and implementation of Chat Service Rest API, to help developers program and use Debox's chat message service faster.
//! For detailed API interface and meaning, please refer to: https://docs.debox.love
//!
//! ## Usage
//! ```rust
//! use debox_open_sdk::{RegisterCallbackUrlBody, Client, ClientOptions, SendChatMsgBody};
//!
//! #[tokio::main]
//! async fn main() {
//!     let opt = ClientOptions {
//!         endpoint: "https://open.debox.pro".to_string(),
//!         api_key: "api_key".to_string(),
//!         user_agent: None,
//!         request_time_out: None,
//!         auth_version: None,
//!     };
//!     let client = Client::new(&opt);
//!     let body = RegisterCallbackUrlBody {
//!         url: "http://xxx.com".to_string(),
//!         http_method: "POST".to_string(),
//!     };
//!     let res = client.register_callbak_url(&body).await;
//!     match res {
//!         Ok(res) => {
//!             println!("register_callbak_url res: {:?}", res);
//!         }
//!         Err(e) => {
//!             println!("register_callbak_url err: {:?}", e);
//!         }
//!     }
//!     let body = SendChatMsgBody {
//!         group_id: "group_id".to_string(),
//!         to_user_id: "DeBox.Love".to_string(),
//!         message: "Hello World".to_string(),
//!     };
//!     let res = client.send_chat_msg(&body).await;
//!     match res {
//!         Ok(res) => {
//!             println!("send_chat_msg res: {:?}", res);
//!         }
//!         Err(e) => {
//!             println!("send_chat_msg err: {:?}", e);
//!         }
//!     }
//! }
//! ```
//!

mod api_client;
mod client;

pub use api_client::{ApiResult, RegisterCallbackUrlBody, SendChatMsgBody};
pub use client::{Client, ClientOptions};
