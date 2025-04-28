use mobot_derive::BotRequest;
use serde::{Deserialize, Serialize};

use super::API;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    /// Unique identifier for this chat. This number may be greater than 32 bits and some programming languages may have difficulty/silent defects in interpreting it. But it is smaller than 52 bits, so a signed 64 bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,

    /// Type of chat, can be either “private”, “group”, “supergroup” or “channel”
    #[serde(rename = "type")]
    pub chat_type: String,

    /// Title, for supergroups, channels and group chats
    pub title: Option<String>,

    /// Username, for private chats, supergroups and channels if available
    pub username: Option<String>,

    /// First name of the other party in a private chat
    pub first_name: Option<String>,

    /// Last name of the other party in a private chat
    pub last_name: Option<String>,

    /// True if a group has ‘All Members Are Admins’ enabled.
    pub all_members_are_administrators: Option<bool>,

    /// True if a channel has a discussion group, or a supergroup is public
    /// and has more than 200 members.
    pub is_forum: Option<bool>,
}

impl<T: Into<String>> From<T> for Chat {
    fn from(s: T) -> Self {
        let from = s.into();
        Self {
            chat_type: "private".to_string(),
            username: Some(from.clone()),
            first_name: Some(from),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatAction {
    #[serde(rename = "typing")]
    Typing,
    #[serde(rename = "upload_photo")]
    UploadPhoto,
    #[serde(rename = "record_video")]
    RecordVideo,
    #[serde(rename = "upload_video")]
    UploadVideo,
    #[serde(rename = "record_audio")]
    RecordAudio,
    #[serde(rename = "upload_audio")]
    UploadAudio,
    #[serde(rename = "upload_document")]
    UploadDocument,
    #[serde(rename = "find_location")]
    FindLocation,
    #[serde(rename = "record_video_note")]
    RecordVideoNote,
    #[serde(rename = "upload_video_note")]
    UploadVideoNote,
}

#[derive(Debug, Clone, Serialize, Deserialize, BotRequest)]
pub struct SendChatActionRequest {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,

    /// Unique identifier for the target message thread.
    pub message_thread_id: Option<i64>,

    /// Type of action to broadcast.
    pub action: ChatAction,
}

impl SendChatActionRequest {
    pub fn new(chat_id: i64, action: ChatAction) -> Self {
        Self {
            chat_id,
            action,
            message_thread_id: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatPermissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_audios: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_documents: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_photos: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_videos: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_video_notes: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_voice_notes: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_polls: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_other_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_web_page_previews: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>
}

#[derive(Debug, Clone, Serialize, Deserialize, BotRequest)]
pub struct SetChatPermissionRequest {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: String,
    pub permissions: ChatPermissions,
    pub use_independent_chat_permissions: Option<bool>
}

impl SetChatPermissionRequest {
    pub fn new(chat_id: String, permissions: ChatPermissions, use_independent_chat_permissions: Option<bool>) -> Self {
        Self {
            chat_id,
            permissions,
            use_independent_chat_permissions
        }
    }
}

/// API methods for sending, editing, set message permission, and deleting messages.
impl API {
    /// Send a message.
    pub async fn send_chat_action(&self, req: &SendChatActionRequest) -> anyhow::Result<bool> {
        self.client.post("sendChatAction", req).await
    }

    /// Use this method to set default chat permissions for all members.
    /// The bot must be an administrator in the group or a supergroup for this to work and must have the can_restrict_members administrator rights.
    /// Returns True on success
    pub async fn set_chat_permissions(&self, req: &SetChatPermissionRequest) -> anyhow::Result<bool> {
        self.client.post("setChatPermissions", req).await
    }
}
