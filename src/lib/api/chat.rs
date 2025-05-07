use mobot_derive::BotRequest;
use serde::{Deserialize, Serialize};
use super::user::User;
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

#[derive(Debug, Clone, Serialize, Deserialize, BotRequest)]
pub struct RestrictChatMemberRequest {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: String,

    /// Unique identifier of the target user
    pub user_id: i64,

    /// A JSON-serialized object for new user permissions
    pub permissions: ChatPermissions,

    /// Pass True if the chat permissions are set independently.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_independent_chat_permissions: Option<bool>,

    /// Date when restrictions will be lifted for the user; Unix time. If user is restricted for more than 366 days or less than 30 seconds from the current time, they are considered to be restricted forever
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>
}

impl RestrictChatMemberRequest {
    pub fn new(chat_id: String, user_id: i64, permissions: ChatPermissions, use_independent_chat_permissions: Option<bool>, until_date: Option<i64>) -> Self {
        Self {
            chat_id,
            user_id,
            permissions,
            use_independent_chat_permissions,
            until_date
        }
    }

}

#[derive(Debug, Clone, Serialize, Deserialize, BotRequest)]
pub struct GetChatAdministratorsRequest {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @channelusername)
    pub chat_id: String,
}

impl GetChatAdministratorsRequest {
    pub fn new(chat_id: String) -> Self {
        Self { chat_id }
    }
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct ChatMemberAdministrator {
    /// The member's status in the chat, always “administrator”
    pub status: String,
    /// Information about the user
    pub user: User,
    /// True, if the bot is allowed to edit administrator privileges of that user
    pub can_be_edited: Option<bool>,
    /// True, if the user's presence in the chat is hidden
    pub is_anonymous: Option<bool>,
    /// True, if the administrator can access the chat event log, get boost list,
    /// see hidden supergroup and channel members, report spam messages and ignore slow mode.
    /// Implied by any other administrator privilege.
    pub can_manage_chat: Option<bool>,
    /// True, if the administrator can delete messages of other users
    pub can_delete_messages: Option<bool>,
    /// True, if the administrator can manage video chats
    pub can_manage_video_chats: Option<bool>,
    /// True, if the administrator can restrict, ban or unban chat members, or access supergroup statistics
    pub can_restrict_members: Option<bool>,
    /// True, if the administrator can add new administrators with a subset of their own privileges or demote administrators that they have promoted,
    /// directly or indirectly (promoted by administrators that were appointed by the user)
    pub can_promote_members: Option<bool>,
    /// True, if the administrator can change chat title, photo and other settings
    pub can_change_info: Option<bool>,
    /// True, if the administrator can invite new users to the chat
    pub can_invite_users: Option<bool>,
    /// True, if the administrator can post stories to the chat
    pub can_post_stories: Option<bool>,
    /// True, if the administrator can edit stories posted by other users,
    /// post stories to the chat page, pin chat stories, and access the chat's story archive
    pub can_edit_stories: Option<bool>,
    /// True, if the administrator can delete stories posted by other users
    pub can_delete_stories: Option<bool>,
    /// Optional. True, if the administrator can post messages in the channel, or access channel statistics; for channels only
    pub can_post_messages: Option<bool>,
    /// Optional. True, if the administrator can edit messages of other users and can pin messages; for channels only
    pub can_edit_messages: Option<bool>,
    /// Optional. True, if the user is allowed to pin messages; for groups and supergroups only
    pub can_pin_messages: Option<bool>,
    /// Optional. True, if the user is allowed to create, rename, close, and reopen forum topics; for supergroups only
    pub can_manage_topics: Option<bool>,
    /// Optional. Custom title for this user
    pub custom_title: Option<String>
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

    /// Use this method to restrict a user in a supergroup.
    /// The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights.
    /// Pass True for all permissions to lift restrictions from a user.
    /// Returns True on success.
    pub async fn restrict_chat_member(&self, req: &RestrictChatMemberRequest) -> anyhow::Result<bool> {
        self.client.post("restrictChatMember", req).await
    }

    /// Use this method to get a list of administrators in a chat, which aren't bots. Returns an Array of ChatMember objects.
    pub async fn get_chat_administrators(&self, req: &GetChatAdministratorsRequest) -> anyhow::Result<Vec<ChatMemberAdministrator>> {
        self.client.post("getChatAdministrators", req).await
    }
}
