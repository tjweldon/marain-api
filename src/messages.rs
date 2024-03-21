use chrono::{DateTime, LocalResult, TimeZone, Utc};
use serde::{Deserialize, Serialize};

pub type Key = [u8; 32];

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Hash)]
pub enum ClientMsgBody {
    Login(String, Key),
    SendToRoom { contents: String },
    SendToUser { contents: String, to: String },
    Move { target: String },
    QueryRoom,
    GetTime,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Hash)]
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Hash)]
pub struct ClientMsg {
    pub token: Option<String>,
    pub body: ClientMsgBody,
    pub timestamp: Timestamp,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Hash)]
pub enum Status {
    Yes,
    No(String),
    JustNo,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Hash)]
pub struct ChatMsg {
    pub sender: String,
    pub timestamp: Timestamp,
    pub content: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Hash)]
pub enum ServerMsgBody {
    Empty,
    LoginSuccess {
        token: String,
        public_key: Key,
    },
    RoomData {
        room_name: String,
        query_ts: Timestamp,
        logs: Vec<ChatMsg>,
        occupants: Vec<String>,
    },
    ChatRecv {
        direct: bool,
        chat_msg: ChatMsg,
    },
    Notification {
        body: String
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Hash)]
pub struct ServerMsg {
    pub status: Status,
    pub timestamp: Timestamp,
    pub body: ServerMsgBody,
}
