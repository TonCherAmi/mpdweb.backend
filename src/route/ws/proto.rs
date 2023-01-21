use std::str::FromStr;

use axum::extract::ws::Message;
use serde::Deserialize;
use serde::Serialize;
use serde_json as json;

use crate::mpd;
use crate::route::ws::action::Action;
use crate::route::ws::data;
use crate::route::ws::sub;

#[derive(Deserialize)]
pub struct Request<T> {
    pub id: String,
    pub content: T,
}

#[derive(Serialize)]
pub struct Status {
    code: i16,
    message: Option<String>,
}

impl Status {
    pub const fn new(code: i16, message: Option<String>) -> Self {
        Status {
            code,
            message,
        }
    }

    pub const fn success() -> Self {
        Status::new(Status::SUCCESS_CODE, None)
    }
}

impl Status {
    pub const DISCONNECTED_ERR_CODE: i16 = -3;
    pub const PARSE_ERR_CODE: i16 = -2;
    pub const INTERNAL_ERR_CODE: i16 = -1;

    pub const SUCCESS_CODE: i16 = 0;

    pub const FORBIDDEN_ERR_CODE: i16 = 1;
    pub const NOT_FOUND_ERR_CODE: i16 = 2;
    pub const CONFLICT_ERR_CODE: i16 = 3;
}

#[derive(Serialize)]
pub struct Response {
    pub id: String,
    #[serde(flatten)]
    pub content: Content<ResponseContent>,
}

impl Response {
    pub fn new(id: String, content: Content<ResponseContent>) -> Self {
        Response { id, content }
    }
}

#[derive(Serialize)]
pub struct ResponseContent {
    #[serde(flatten)]
    pub status: Status,
}

#[derive(Serialize)]
pub struct Content<T> {
    pub content: T,
}

impl<T> Content<T> {
    pub fn new(content: T) -> Self {
        Content { content }
    }
}

#[derive(Serialize)]
pub struct Update {
    pub items: Option<Vec<UpdateKind>>,
    #[serde(flatten)]
    pub status: Status,
}

impl Update {
    pub fn from_data(data: Vec<UpdateKind>) -> Self {
        Update {
            items: Some(data),
            status: Status::success(),
        }
    }

    pub fn from_err(err: mpd::Error) -> Self {
        Update {
            items: None,
            status: err.into(),
        }
    }
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Out {
    Update(Content<Update>),
    Response(Response),
}

impl Out {
    pub fn update(update: Update) -> Self {
        Out::Update(Content::new(update))
    }

    pub fn response(id: String, status: Status) -> Self {
        let response = Response {
            id,
            content: Content::new(ResponseContent { status })
        };

        Out::Response(response)
    }
}

#[derive(Serialize)]
#[serde(tag = "type", content = "data", rename_all = "camelCase")]
pub enum UpdateKind {
    Db,
    Playlists,
    Status(data::Status),
    Queue(Vec<data::QueueItem>),
}

impl From<mpd::Error> for Status {
    fn from(err: mpd::Error) -> Self {
        let code = match &err {
            mpd::Error::Internal(_) => Status::INTERNAL_ERR_CODE,
            mpd::Error::Forbidden(_) => Status::FORBIDDEN_ERR_CODE,
            mpd::Error::NotFound(_) => Status::NOT_FOUND_ERR_CODE,
            mpd::Error::AlreadyExists(_) => Status::CONFLICT_ERR_CODE,
            mpd::Error::Disconnected(_) | mpd::Error::Unavailable(_) => Status::DISCONNECTED_ERR_CODE,
        };

        Status {
            code,
            message: Some(err.to_string()),
        }
    }
}

impl From<mpd::Error> for Update {
    fn from(err: mpd::Error) -> Self {
        Update {
            items: None,
            status: err.into(),
        }
    }
}

impl From<sub::Update> for UpdateKind {
    fn from(upd: sub::Update) -> Self {
        match upd {
            sub::Update::Db => {
                UpdateKind::Db
            },
            sub::Update::Playlists => {
                UpdateKind::Playlists
            },
            sub::Update::Status(status) => {
                UpdateKind::Status(status.into())
            },
            sub::Update::Queue(queue) => {
                UpdateKind::Queue(queue.into_iter().map(Into::into).collect())
            },
        }
    }
}

impl FromStr for Request<Action> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        json::from_str(s).map_err(|e| format!("failed to parse message: {e}"))
    }
}

impl From<Out> for Message {
    fn from(out: Out) -> Self {
        Message::Text(
            json::to_string(&out)
                .expect("serialization of outgoing message failed")
        )
    }
}
