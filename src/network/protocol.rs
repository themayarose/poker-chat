use rmp_serde;
use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Bytes;

use crate::network::client;

#[derive(Clone, Serialize, Deserialize)]
pub enum ClientMessage {
   Join(String),
   SetNickname(String),
   SendMessage(String),
   Ping,
   Quit,
}

impl Into<Bytes> for ClientMessage {
   fn into(self) -> Bytes {
        rmp_serde::to_vec(&self).unwrap().into()
   }
}

impl TryFrom<&Bytes> for ClientMessage {
   type Error = rmp_serde::decode::Error;

   fn try_from(bytes: &Bytes) -> Result<Self, Self::Error> {
        rmp_serde::from_slice(bytes.as_slice())
   }
}

impl ClientMessage {
   pub fn forward(&self, from: &uuid::Uuid) -> Option<ServerMessage> {
      match self {
         ClientMessage::SetNickname(nickname) => Some(ServerMessage::NicknameChanged(from.to_owned(), nickname.clone())),
         ClientMessage::SendMessage(message) => Some(ServerMessage::TextMessage(from.to_owned(), message.clone())),
         ClientMessage::Quit => Some(ServerMessage::UserLeft(from.to_owned())),
         ClientMessage::Ping => Some(ServerMessage::Pong),
         _ => None,
      }
   }
}


#[derive(Clone, Serialize, Deserialize)]
pub enum ServerMessage {
   Welcome(uuid::Uuid),
   NicknameChanged(uuid::Uuid, String),
   TextMessage(uuid::Uuid, String),
   UserLeft(uuid::Uuid),
   UserJoined(client::User),
   CurrentUsers(Vec<client::User>),
   NicknameAlreadyTaken(String),
   Pong,
   NoNicknameError,
}

impl Into<Bytes> for ServerMessage {
   fn into(self) -> Bytes {
        rmp_serde::to_vec(&self).unwrap().into()
   }
}

impl TryFrom<Bytes> for ServerMessage {
   type Error = rmp_serde::decode::Error;

   fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        rmp_serde::from_slice(bytes.as_slice())
   }
}
