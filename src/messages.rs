use chrono::{DateTime, LocalResult, TimeZone, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum ClientMsgBody {
    Login(String),
    SendToRoom { contents: String },
    SendToUser { contents: String, to: String },
    Move { target: String },
    QueryRoom,
    GetTime,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Timestamp(i64, u32);

impl Into<Option<DateTime<Utc>>> for Timestamp {
    fn into(self) -> Option<DateTime<Utc>> {
        match Utc.timestamp_opt(self.0, self.1) {
            LocalResult::Single(dt) => Some(dt),
            _ => None,
        }
    }
}

impl From<DateTime<Utc>> for Timestamp {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value.timestamp(), value.timestamp_subsec_nanos())
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ClientMsg {
    pub token: Option<String>,
    pub body: ClientMsgBody,
    pub timestamp: Timestamp,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Status {
    Yes,
    No(String),
    JustNo,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ChatMsg {
    pub sender: String,
    pub timestamp: Timestamp,
    pub content: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum ServerMsgBody {
    Empty,
    LoginSuccess {
        token: String,
    },
    RoomData {
        query_ts: Timestamp,
        logs: Vec<ChatMsg>,
        occupants: Vec<String>,
    },
    ChatRecv {
        direct: bool,
        chat_msg: ChatMsg,
    },
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ServerMsg {
    pub status: Status,
    pub timestamp: Timestamp,
    pub body: ServerMsgBody,
}
