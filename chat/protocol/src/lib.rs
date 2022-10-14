//! Chapter 20 Asynchronous Programming Protocol
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::sync::Arc;

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum Request {
    Join {
        group_name: Arc<String>,
    },
    Post {
        group_name: Arc<String>,
        message: Arc<String>,
    },
}

impl TryFrom<String> for Request {
    type Error = String;
    fn try_from(line: String) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref JOIN: Regex = Regex::new(r"^\s*join\s*(\S+)\s*$",).expect("join regex");
            static ref POST: Regex = Regex::new(r"^\s*post\s*(\S+)\s*(.+)$",).expect("post regex");
        }
        match JOIN.captures(&line) {
            None => match POST.captures(&line) {
                None => Err(format!("wrong post: {line:?}")),
                Some(match_) => Ok(Self::Post {
                    group_name: Arc::new(match_[1].to_string()),
                    message: Arc::new(match_[2].to_string()),
                }),
            },
            Some(match_) => Ok(Self::Join {
                group_name: Arc::new(match_[1].to_string()),
            }),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum Response {
    Message {
        group_name: Arc<String>,
        message: Arc<String>,
    },
    Error(String),
}

// We don't need impl From<String> for Response.
#[allow(clippy::from_over_into)]
impl Into<String> for Response {
    fn into(self) -> String {
        match self {
            Self::Error(err) => format!("error: {err}"),
            Self::Message {
                group_name,
                message,
            } => format!("{group_name}: {message}"),
        }
    }
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
    fn request_try_from_string() {
        let join = "join dao".to_string();
        let got = Request::try_from(join).unwrap();
        let want = Request::Join {
            group_name: Arc::new("dao".to_string()),
        };
        assert_eq!(got, want);
        let post = "post dao let's create dao by 2023".to_string();
        let got = Request::try_from(post).unwrap();
        let want = Request::Post {
            group_name: Arc::new("dao".to_string()),
            message: Arc::new("let's create dao by 2023".to_string()),
        };
        assert_eq!(got, want);
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

    #[test]
    fn response_into_string() {
        let resp = Response::Message {
            group_name: Arc::new("dao".to_string()),
            message: Arc::new("let's create dao by 2023".to_string()),
        };
        let got: String = resp.into();
        let want = "dao: let's create dao by 2023".to_string();
        assert_eq!(got, want);
        let resp = Response::Error("something wrong".to_string());
        let got: String = resp.into();
        let want = "error: something wrong".to_string();
        assert_eq!(got, want);
    }
}
