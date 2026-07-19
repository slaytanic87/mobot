use chrono::Utc;
use mobot_derive::BotRequest;
use serde::{Deserialize, Serialize};

use super::{chat::Chat, sticker::Sticker, user::User, Document, PhotoSize, ReplyMarkup, API};

/// This object represents a point on the map.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Location {
    /// Latitude as defined by the sender
    pub latitude: f64,
    /// Longitude as defined by the sender
    pub longitude: f64,
    /// Optional. The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: Option<f64>,
    /// Optional. Time relative to the message sending date, during which the location can be updated; in seconds.
    /// For active live locations only.
    pub live_period: Option<i64>,
    /// Optional. The direction in which user is moving, in degrees; 1-360. For active live locations only.
    pub heading: Option<i64>,
    /// Optional. The maximum distance for proximity alerts about approaching another chat member, in meters.
    /// For sent live locations only.
    pub proximity_alert_radius: Option<i64>,

}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VideoQuality {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. 
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    pub width: i64,
    pub height: i64,
    /// Codec that was used to encode the video, for example, "h264", "h265", or "av01"
    pub codec: String,
    /// File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it.
    /// But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Video {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. 
    /// Can't be used to download or reuse the file.    
    pub file_unique_id: String,
    pub width: i64,
    pub height: i64,
    /// Duration of the video in seconds as defined by the sender
    pub duration: i64,
    pub thumbnail: Option<PhotoSize>,
    /// Available sizes of the cover of the video in the message
    pub cover: Option<Vec<PhotoSize>>,
    /// Timestamp in seconds from which the video will play in the message    
    pub start_timestamp: Option<i64>,
    /// List of available qualities of the video
    pub qualities: Option<Vec<VideoQuality>>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    /// File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it.
    /// But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    pub file_size: Option<i64>,
}

/// This object represents an audio file to be treated as music by the Telegram clients.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Audio {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. 
    /// Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Duration of the audio in seconds as defined by the sender
    pub duration: i64,
    /// Optional. Performer of the audio as defined by the sender or by audio tags
    pub performer: Option<String>,
    /// Optional. Title of the audio as defined by the sender or by audio tags
    pub title: Option<String>,
    /// Optional. Original filename as defined by the sender
    pub file_name: Option<String>,
    /// Optional. MIME type of the file as defined by the sender
    pub mime_type: Option<String>,
    /// Optional. File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it.
    /// But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    pub file_size: Option<i64>,
    /// Optional. Thumbnail of the album cover to which the music file belongs
    pub thumbnail: Option<PhotoSize>,
}

/// This object represents a service message about a new forum topic created in the chat.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ForumTopicCreated {
    /// Name of the topic
    pub name: String,

    /// Color of the topic icon in RGB format
    pub icon_color: i64,

    /// Emoji associated with the topic icon
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

/// `Message` represents a message sent in a chat. It can be a text message, a sticker, a photo, etc.
/// <https://core.telegram.org/bots/api#message>
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Message {
    /// Unique message identifier inside this chat
    pub message_id: i64,

    /// Optional. Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,

    /// Sender, empty for messages sent to channels
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<User>,

    /// Date the message was sent in Unix time
    pub date: i64,

    /// Message text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Message is a photo, available sizes of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,

    /// Optional. Message is an audio file, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,

    /// Message is a general file, information about the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<Document>,

    /// Message is a video, information about the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,

    /// Conversation the message belongs to
    /// - For sent messages, the first available identifier of the chat
    /// - For messages forwarded to the chat, the identifier of the original chat
    /// - For messages in channels, the identifier of the channel is contained in the `chat_id` field
    pub chat: Chat,

    /// Optional. Caption for the animation, audio, document, paid media, photo, video or voice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    /// Optional. Message is a shared location, information about the location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    /// For forwarded messages, sender of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_from: Option<User>,

    /// For messages forwarded from channels, information about the original channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_from_chat: Option<Chat>,

    /// For messages forwarded from channels, identifier of the original message in the channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_from_message_id: Option<i64>,

    /// For messages forwarded from channels, signature of the post author if present
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_signature: Option<String>,

    /// Sender's name for messages forwarded from users who disallow adding a link to their account in forwarded messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_sender_name: Option<String>,

    /// For forwarded messages, date the original message was sent in Unix time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_date: Option<i64>,

    /// For replies, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message: Option<serde_json::value::Value>,

    /// Sticker for messages with a sticker
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,

    /// Optional. Service message: forum topic created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_created: Option<ForumTopicCreated>,

    /// Inline keyboard attached to the message.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl Message {
    /// Creates a new `Message` with the given `text` and `from` fields.
    pub fn new(from: impl Into<String>, text: impl Into<String>) -> Self {
        let mut message = Message::fake(from.into());
        message.text = Some(text.into());
        message
    }

    pub fn fake(from: impl AsRef<str>) -> Self {
        Message {
            message_id: rand::random(),
            from: Some(from.as_ref().into()),
            date: Utc::now().timestamp(),
            chat: from.as_ref().into(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ParseMode {
    #[serde(rename = "MarkdownV2")]
    MarkdownV2,
    #[serde(rename = "Markdown")]
    Markdown,
    #[serde(rename = "HTML")]
    HTML,
    #[serde(rename = "")]
    Text,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct ReplyParameters {
    // Identifier of the original message
    pub message_id: i64,

    // Unique identifier for the target chat or username of the target channel. This
    // field is an i64 OR a String, so we use serde_json::value::Value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<serde_json::value::Value>,

    // Allow sending the message without a reply
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,

    // Quoted part of the message to be replied to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_parse_mode: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_entities: Option<serde_json::value::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_position: Option<i64>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, BotRequest)]
pub struct SendMessageRequest {
    /// Unique identifier for the target chat or username of the target
    pub chat_id: i64,

    /// Optional	Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    pub message_thread_id: Option<i64>,

    /// Text of the message to be sent
    pub text: String,

    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    /// Parse mode for the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    /// Reply markup for the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendMessageRequest {
    pub fn new(chat_id: i64, text: impl Into<String>) -> Self {
        Self {
            chat_id,
            text: text.into(),
            ..Default::default()
        }
    }
    pub fn with_message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = Some(message_thread_id);
        self
    }
    
    pub fn with_reply_markup(mut self, reply_markup: ReplyMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }

    pub fn with_parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct EditMessageBase {
    /// Required if `inline_message_id` is not specified. Unique identifier for the
    /// target chat or username of the target channel (in the format @channelusername)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,

    /// Required if `inline_message_id` is not specified. Identifier of the message
    /// to edit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,

    /// Inline message identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,

    /// Mode for parsing entities in the message text. See formatting options for
    /// more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    /// Reply markup for the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<String>,
}

impl EditMessageBase {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = Some(chat_id);
        self
    }

    pub fn with_message_id(mut self, message_id: i64) -> Self {
        self.message_id = Some(message_id);
        self
    }

    pub fn with_parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    pub fn with_reply_markup(mut self, reply_markup: ReplyMarkup) -> Self {
        self.reply_markup = Some(serde_json::to_string(&reply_markup).unwrap());
        self
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, BotRequest)]
pub struct EditMessageTextRequest {
    /// Base fields for edit requests
    #[serde(flatten)]
    pub base: EditMessageBase,

    /// The new text of the message, 1-4096 characters after entities parsing
    /// (Markdown or HTML)
    pub text: String,
}

impl EditMessageTextRequest {
    pub fn new(text: String) -> Self {
        Self {
            base: EditMessageBase::new(),
            text,
        }
    }

    pub fn with_chat_id(mut self, chat_id: i64) -> Self {
        self.base.chat_id = Some(chat_id);
        self
    }

    pub fn with_message_id(mut self, message_id: i64) -> Self {
        self.base.message_id = Some(message_id);
        self
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, BotRequest)]
pub struct EditMessageCaptionRequest {
    /// Base fields for edit requests
    #[serde(flatten)]
    pub base: EditMessageBase,

    /// New caption of the message, 0-1024 characters after entities parsing
    /// (Markdown or HTML)
    pub caption: String,
}

impl EditMessageCaptionRequest {
    pub fn new(caption: String) -> Self {
        Self {
            base: EditMessageBase::new(),
            caption,
        }
    }

    pub fn with_chat_id(mut self, chat_id: i64) -> Self {
        self.base.chat_id = Some(chat_id);
        self
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, BotRequest)]
pub struct EditMessageReplyMarkupRequest {
    /// Base fields for edit requests
    #[serde(flatten)]
    pub base: EditMessageBase,
}

impl EditMessageReplyMarkupRequest {
    pub fn new(reply_markup: ReplyMarkup) -> Self {
        Self {
            base: EditMessageBase::new().with_reply_markup(reply_markup),
        }
    }

    pub fn with_chat_id(mut self, chat_id: i64) -> Self {
        self.base.chat_id = Some(chat_id);
        self
    }

    pub fn with_message_id(mut self, message_id: i64) -> Self {
        self.base.message_id = Some(message_id);
        self
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, BotRequest)]
pub struct DeleteMessageRequest {
    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    pub chat_id: i64,

    /// Identifier of the message to delete
    pub message_id: i64,
}

impl DeleteMessageRequest {
    pub fn new(chat_id: i64, message_id: i64) -> Self {
        Self {
            chat_id,
            message_id,
        }
    }
}

#[derive(Debug, Serialize, Clone, BotRequest)]
pub struct ReactionType {
    /// Type of the reaction. For type "emoji", the emoji field must be set. For type "custom_emoji", the custom_emoji_id field must be set. Otherwise "paid" for paid emojis
    #[serde(rename = "type")]
    pub type_: String,
    /// Reaction emoji. Currently, it can be one of "❤", "👍", "👎", "🔥", "🥰", "👏", "😁", "🤔", "🤯",
    ///  "😱", "🤬", "😢", "🎉", "🤩", "🤮", "💩", "🙏", "👌", "🕊", "🤡", "🥱", "🥴", "😍", "🐳", "❤‍🔥", "🌚",
    /// "🌭", "💯", "🤣", "⚡", "🍌", "🏆", "💔", "🤨", "😐", "🍓", "🍾", "💋", "🖕", "😈", "😴", "😭", "🤓", "👻",
    /// "👨‍💻", "👀", "🎃", "🙈", "😇", "😨", "🤝", "✍", "🤗", "🫡", "🎅", "🎄", "☃", "💅", "🤪", "🗿", "🆒", "💘",
    /// "🙉", "🦄", "😘", "💊", "🙊", "😎", "👾", "🤷‍♂", "🤷", "🤷‍♀", "😡".
    pub emoji: Option<String>,
    pub custom_emoji_id: Option<String>,
}

impl ReactionType {
    pub fn new(type_: String, emoji: Option<String>, custom_emoji_id: Option<String>) -> Self {
        Self {
            type_,
            emoji,
            custom_emoji_id,
        }
    }
}

#[derive(Default, Debug, Serialize, Clone, BotRequest)]
pub struct MessageReactionRequest {
    pub chat_id: i64,
    pub message_id: i64,
    pub reaction: Option<ReactionType>,
    pub is_big: Option<bool>,
}

impl MessageReactionRequest {
    pub fn new(chat_id: i64, message_id: i64, reaction: Option<ReactionType>, is_big: Option<bool>) -> Self {
        Self {
            chat_id,
            message_id,
            reaction,
            is_big,
        }
    }
}

/// API methods for sending, editing, and deleting messages.
impl API {
    /// Send a message to a chat or channel.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use mobot::*;
    /// # #[tokio::main]
    /// # async fn main() {
    /// #    let api = API::new(Client::new(String::from("boo")));
    /// #    let chat_id = 123456789;
    ///    api.send_message(&api::SendMessageRequest::new(chat_id, "Hello!")).await;
    /// # }
    /// ```
    pub async fn send_message(&self, req: &SendMessageRequest) -> anyhow::Result<Message> {
        self.client.post("sendMessage", req).await
    }

    /// Edit the text of a previously sent message.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use mobot::*;
    /// # #[tokio::main]
    /// # async fn main() {
    /// #    let api = API::new(Client::new(String::from("boo")));
    /// #    let chat_id = 123456789;
    /// #    let message_id = 0;
    /// api.edit_message_text(
    ///   &api::EditMessageTextRequest::new(String::from("Changed my mind: Goodbye world!"))
    ///      .with_chat_id(chat_id)
    ///      .with_message_id(message_id)
    /// ).await;
    /// # }
    /// ```
    pub async fn edit_message_text(&self, req: &EditMessageTextRequest) -> anyhow::Result<Message> {
        self.client.post("editMessageText", req).await
    }

    /// Edit the caption of a message.
    pub async fn edit_message_caption(
        &self,
        req: &EditMessageCaptionRequest,
    ) -> anyhow::Result<Message> {
        self.client.post("editMessageCaption", req).await
    }

    /// Edit the reply markup of a message.
    pub async fn edit_message_reply_markup(
        &self,
        req: &EditMessageReplyMarkupRequest,
    ) -> anyhow::Result<Message> {
        self.client.post("editMessageReplyMarkup", req).await
    }

    /// Delete a message.
    pub async fn delete_message(&self, req: &DeleteMessageRequest) -> anyhow::Result<bool> {
        self.client.post("deleteMessage", req).await
    }

    pub async fn remove_reply_keyboard(
        &self,
        chat_id: i64,
        text: String,
    ) -> anyhow::Result<Message> {
        self.send_message(
            &SendMessageRequest::new(chat_id, text)
                .with_reply_markup(ReplyMarkup::reply_keyboard_remove()),
        )
        .await
    }

    /// Use this method to change the chosen reactions on a message. 
    /// Service messages of some types can't be reacted to.
    /// Automatically forwarded messages from a channel to its discussion group have the same available reactions as messages in the channel.
    /// Bots can't use paid reactions. Returns True on success.
    pub async fn set_message_reaction(
        &self,
        req: &MessageReactionRequest,
    ) -> anyhow::Result<bool> {
        self.client.post("setMessageReaction", req).await
    }
}
