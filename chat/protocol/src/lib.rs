//! Chapter 20 Asynchronous Programming Protocol
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum Request {
    Join {
        group_name: Arc<String>,
    },
    Post {
        group_name: Arc<String>,
        message: Arc<String>,
    },
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum Response {
    Message {
        group_name: Arc<String>,
        message: Arc<String>,
    },
    Error(String),
}

#[cfg(test)]
mod tests {
    use super::{Request, Response};
    use std::sync::Arc;

    #[test]
    fn request() {
        let post = Request::Post {
            group_name: Arc::new("dao".to_string()),
            message: Arc::new("let's create dao by 2025".to_string()),
        };
        match &post {
            Request::Post { message, .. } => {
                assert_eq!(*message, Arc::new("let's create dao by 2025".to_string()));
            }
            _ => panic!("something wrong!"),
        }
        let json = serde_json::to_string(&post).unwrap();
        assert_eq!(
            json,
            r#"{"Post":{"group_name":"dao","message":"let's create dao by 2025"}}"#,
        );
        assert_eq!(serde_json::from_str::<Request>(&json).unwrap(), post);
    }

    #[test]
    fn response() {
        let resp = Response::Message {
            group_name: Arc::new("dao".to_string()),
            message: Arc::new("for sure!".to_string()),
        };
        match &resp {
            Response::Message { message, .. } => {
                assert_eq!(*message, Arc::new("for sure!".to_string()));
            }
            _ => panic!("something wrong!"),
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert_eq!(
            json,
            r#"{"Message":{"group_name":"dao","message":"for sure!"}}"#,
        );
        assert_eq!(serde_json::from_str::<Response>(&json).unwrap(), resp);
    }
}
