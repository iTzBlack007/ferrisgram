mod bot_command;
pub use bot_command::BotCommand;

mod reply_parameters;
pub use reply_parameters::ReplyParameters;

mod input_contact_message_content;
pub use input_contact_message_content::InputContactMessageContent;

mod shipping_query;
pub use shipping_query::ShippingQuery;

mod external_reply_info;
pub use external_reply_info::ExternalReplyInfo;

mod inline_query_result_video;
pub use inline_query_result_video::InlineQueryResultVideo;

mod story;
pub use story::Story;

mod passport_element_error;
pub use passport_element_error::PassportElementError;

mod chat_full_info;
pub use chat_full_info::ChatFullInfo;

mod text_quote;
pub use text_quote::TextQuote;

mod chat_background;
pub use chat_background::ChatBackground;

mod message_auto_delete_timer_changed;
pub use message_auto_delete_timer_changed::MessageAutoDeleteTimerChanged;

mod background_type_chat_theme;
pub use background_type_chat_theme::BackgroundTypeChatTheme;

mod message_entity;
pub use message_entity::MessageEntity;

mod general_forum_topic_unhidden;
pub use general_forum_topic_unhidden::GeneralForumTopicUnhidden;

mod reaction_type_custom_emoji;
pub use reaction_type_custom_emoji::ReactionTypeCustomEmoji;

mod input_media_audio;
pub use input_media_audio::InputMediaAudio;

mod input_message_content;
pub use input_message_content::InputMessageContent;

mod input_location_message_content;
pub use input_location_message_content::InputLocationMessageContent;

mod input_invoice_message_content;
pub use input_invoice_message_content::InputInvoiceMessageContent;

mod pre_checkout_query;
pub use pre_checkout_query::PreCheckoutQuery;

mod sent_web_app_message;
pub use sent_web_app_message::SentWebAppMessage;

mod users_shared;
pub use users_shared::UsersShared;

mod video_chat_scheduled;
pub use video_chat_scheduled::VideoChatScheduled;

mod inline_keyboard_button;
pub use inline_keyboard_button::InlineKeyboardButton;

mod inline_query_result_cached_photo;
pub use inline_query_result_cached_photo::InlineQueryResultCachedPhoto;

mod reaction_type_emoji;
pub use reaction_type_emoji::ReactionTypeEmoji;

mod location;
pub use location::Location;

mod menu_button;
pub use menu_button::MenuButton;

mod chat_permissions;
pub use chat_permissions::ChatPermissions;

mod business_intro;
pub use business_intro::BusinessIntro;

mod video_note;
pub use video_note::VideoNote;

mod dice;
pub use dice::Dice;

mod input_poll_option;
pub use input_poll_option::InputPollOption;

mod venue;
pub use venue::Venue;

mod write_access_allowed;
pub use write_access_allowed::WriteAccessAllowed;

mod bot_command_scope_all_chat_administrators;
pub use bot_command_scope_all_chat_administrators::BotCommandScopeAllChatAdministrators;

mod bot_command_scope_chat_administrators;
pub use bot_command_scope_chat_administrators::BotCommandScopeChatAdministrators;

mod switch_inline_query_chosen_chat;
pub use switch_inline_query_chosen_chat::SwitchInlineQueryChosenChat;

mod menu_button_default;
pub use menu_button_default::MenuButtonDefault;

mod sticker_set;
pub use sticker_set::StickerSet;

mod invoice;
pub use invoice::Invoice;

mod passport_element_error_translation_file;
pub use passport_element_error_translation_file::PassportElementErrorTranslationFile;

mod voice;
pub use voice::Voice;

mod maybe_inaccessible_message;
pub use maybe_inaccessible_message::MaybeInaccessibleMessage;

mod message_origin_user;
pub use message_origin_user::MessageOriginUser;

mod passport_element_error_files;
pub use passport_element_error_files::PassportElementErrorFiles;

mod bot_short_description;
pub use bot_short_description::BotShortDescription;

mod chat_boost_updated;
pub use chat_boost_updated::ChatBoostUpdated;

mod video;
pub use video::Video;

mod callback_game;
pub use callback_game::CallbackGame;

mod encrypted_passport_element;
pub use encrypted_passport_element::EncryptedPassportElement;

mod web_app_data;
pub use web_app_data::WebAppData;

mod video_chat_participants_invited;
pub use video_chat_participants_invited::VideoChatParticipantsInvited;

mod chat_member_updated;
pub use chat_member_updated::ChatMemberUpdated;

mod user_profile_photos;
pub use user_profile_photos::UserProfilePhotos;

mod input_file;
pub use input_file::InputFile;

mod reply_keyboard_markup;
pub use reply_keyboard_markup::ReplyKeyboardMarkup;

mod audio;
pub use audio::Audio;

mod inline_query_result_venue;
pub use inline_query_result_venue::InlineQueryResultVenue;

mod inaccessible_message;
pub use inaccessible_message::InaccessibleMessage;

mod chat_boost_source;
pub use chat_boost_source::ChatBoostSource;

mod shared_user;
pub use shared_user::SharedUser;

mod photo_size;
pub use photo_size::PhotoSize;

mod chat;
pub use chat::Chat;

mod bot_command_scope_default;
pub use bot_command_scope_default::BotCommandScopeDefault;

mod bot_command_scope_all_group_chats;
pub use bot_command_scope_all_group_chats::BotCommandScopeAllGroupChats;

mod message_origin_chat;
pub use message_origin_chat::MessageOriginChat;

mod general_forum_topic_hidden;
pub use general_forum_topic_hidden::GeneralForumTopicHidden;

mod reaction_type;
pub use reaction_type::ReactionType;

mod bot_command_scope_all_private_chats;
pub use bot_command_scope_all_private_chats::BotCommandScopeAllPrivateChats;

mod bot_name;
pub use bot_name::BotName;

mod chat_boost_source_giveaway;
pub use chat_boost_source_giveaway::ChatBoostSourceGiveaway;

mod chat_administrator_rights;
pub use chat_administrator_rights::ChatAdministratorRights;

mod contact;
pub use contact::Contact;

mod bot_description;
pub use bot_description::BotDescription;

mod chat_invite_link;
pub use chat_invite_link::ChatInviteLink;

mod forum_topic_closed;
pub use forum_topic_closed::ForumTopicClosed;

mod chat_member_administrator;
pub use chat_member_administrator::ChatMemberAdministrator;

mod menu_button_commands;
pub use menu_button_commands::MenuButtonCommands;

mod poll_answer;
pub use poll_answer::PollAnswer;

mod proximity_alert_triggered;
pub use proximity_alert_triggered::ProximityAlertTriggered;

mod giveaway_created;
pub use giveaway_created::GiveawayCreated;

mod chat_member_owner;
pub use chat_member_owner::ChatMemberOwner;

mod user;
pub use user::User;

mod message;
pub use message::Message;

mod message_origin_hidden_user;
pub use message_origin_hidden_user::MessageOriginHiddenUser;

mod chat_boost_source_premium;
pub use chat_boost_source_premium::ChatBoostSourcePremium;

mod chat_boost_removed;
pub use chat_boost_removed::ChatBoostRemoved;

mod input_media_photo;
pub use input_media_photo::InputMediaPhoto;

mod input_media_animation;
pub use input_media_animation::InputMediaAnimation;

mod sticker;
pub use sticker::Sticker;

mod background_type_fill;
pub use background_type_fill::BackgroundTypeFill;

mod keyboard_button;
pub use keyboard_button::KeyboardButton;

mod forum_topic_edited;
pub use forum_topic_edited::ForumTopicEdited;

mod input_media_video;
pub use input_media_video::InputMediaVideo;

mod inline_query_result_audio;
pub use inline_query_result_audio::InlineQueryResultAudio;

mod callback_query;
pub use callback_query::CallbackQuery;

mod inline_query_result_document;
pub use inline_query_result_document::InlineQueryResultDocument;

mod document;
pub use document::Document;

mod inline_query_result_cached_sticker;
pub use inline_query_result_cached_sticker::InlineQueryResultCachedSticker;

mod encrypted_credentials;
pub use encrypted_credentials::EncryptedCredentials;

mod chat_boost_added;
pub use chat_boost_added::ChatBoostAdded;

mod inline_query;
pub use inline_query::InlineQuery;

mod inline_query_result_cached_gif;
pub use inline_query_result_cached_gif::InlineQueryResultCachedGif;

mod chat_member;
pub use chat_member::ChatMember;

mod business_connection;
pub use business_connection::BusinessConnection;

mod inline_query_result_article;
pub use inline_query_result_article::InlineQueryResultArticle;

mod shipping_address;
pub use shipping_address::ShippingAddress;

mod passport_element_error_data_field;
pub use passport_element_error_data_field::PassportElementErrorDataField;

mod input_media_document;
pub use input_media_document::InputMediaDocument;

mod passport_element_error_file;
pub use passport_element_error_file::PassportElementErrorFile;

mod background_type_wallpaper;
pub use background_type_wallpaper::BackgroundTypeWallpaper;

mod giveaway_winners;
pub use giveaway_winners::GiveawayWinners;

mod chat_boost;
pub use chat_boost::ChatBoost;

mod business_opening_hours;
pub use business_opening_hours::BusinessOpeningHours;

mod inline_query_result_photo;
pub use inline_query_result_photo::InlineQueryResultPhoto;

mod reaction_count;
pub use reaction_count::ReactionCount;

mod passport_element_error_translation_files;
pub use passport_element_error_translation_files::PassportElementErrorTranslationFiles;

mod input_media;
pub use input_media::InputMedia;

mod passport_file;
pub use passport_file::PassportFile;

mod business_opening_hours_interval;
pub use business_opening_hours_interval::BusinessOpeningHoursInterval;

mod message_reaction_updated;
pub use message_reaction_updated::MessageReactionUpdated;

mod passport_element_error_selfie;
pub use passport_element_error_selfie::PassportElementErrorSelfie;

mod update;
pub use update::Update;

mod background_fill_freeform_gradient;
pub use background_fill_freeform_gradient::BackgroundFillFreeformGradient;

mod background_type_pattern;
pub use background_type_pattern::BackgroundTypePattern;

mod game;
pub use game::Game;

mod video_chat_started;
pub use video_chat_started::VideoChatStarted;

mod keyboard_button_request_users;
pub use keyboard_button_request_users::KeyboardButtonRequestUsers;

mod message_origin;
pub use message_origin::MessageOrigin;

mod inline_keyboard_markup;
pub use inline_keyboard_markup::InlineKeyboardMarkup;

mod forum_topic;
pub use forum_topic::ForumTopic;

mod webhook_info;
pub use webhook_info::WebhookInfo;

mod login_url;
pub use login_url::LoginUrl;

mod game_high_score;
pub use game_high_score::GameHighScore;

mod business_location;
pub use business_location::BusinessLocation;

mod business_messages_deleted;
pub use business_messages_deleted::BusinessMessagesDeleted;

mod inline_query_result_voice;
pub use inline_query_result_voice::InlineQueryResultVoice;

mod inline_query_result_contact;
pub use inline_query_result_contact::InlineQueryResultContact;

mod passport_element_error_unspecified;
pub use passport_element_error_unspecified::PassportElementErrorUnspecified;

mod link_preview_options;
pub use link_preview_options::LinkPreviewOptions;

mod menu_button_web_app;
pub use menu_button_web_app::MenuButtonWebApp;

mod poll_option;
pub use poll_option::PollOption;

mod background_type;
pub use background_type::BackgroundType;

mod chat_member_restricted;
pub use chat_member_restricted::ChatMemberRestricted;

mod chat_location;
pub use chat_location::ChatLocation;

mod user_chat_boosts;
pub use user_chat_boosts::UserChatBoosts;

mod chat_shared;
pub use chat_shared::ChatShared;

mod force_reply;
pub use force_reply::ForceReply;

mod chat_join_request;
pub use chat_join_request::ChatJoinRequest;

mod response_parameters;
pub use response_parameters::ResponseParameters;

mod inline_query_result_gif;
pub use inline_query_result_gif::InlineQueryResultGif;

mod chat_boost_source_gift_code;
pub use chat_boost_source_gift_code::ChatBoostSourceGiftCode;

mod background_fill_solid;
pub use background_fill_solid::BackgroundFillSolid;

mod message_id;
pub use message_id::MessageId;

mod video_chat_ended;
pub use video_chat_ended::VideoChatEnded;

mod giveaway;
pub use giveaway::Giveaway;

mod input_text_message_content;
pub use input_text_message_content::InputTextMessageContent;

mod passport_element_error_reverse_side;
pub use passport_element_error_reverse_side::PassportElementErrorReverseSide;

mod inline_query_result_game;
pub use inline_query_result_game::InlineQueryResultGame;

mod message_origin_channel;
pub use message_origin_channel::MessageOriginChannel;

mod inline_query_result_cached_audio;
pub use inline_query_result_cached_audio::InlineQueryResultCachedAudio;

mod inline_query_result_mpeg_4_gif;
pub use inline_query_result_mpeg_4_gif::InlineQueryResultMpeg4Gif;

mod chat_member_member;
pub use chat_member_member::ChatMemberMember;

mod background_fill;
pub use background_fill::BackgroundFill;

mod inline_query_result_cached_video;
pub use inline_query_result_cached_video::InlineQueryResultCachedVideo;

mod inline_query_results_button;
pub use inline_query_results_button::InlineQueryResultsButton;

mod labeled_price;
pub use labeled_price::LabeledPrice;

mod bot_command_scope_chat_member;
pub use bot_command_scope_chat_member::BotCommandScopeChatMember;

mod shipping_option;
pub use shipping_option::ShippingOption;

mod forum_topic_reopened;
pub use forum_topic_reopened::ForumTopicReopened;

mod passport_data;
pub use passport_data::PassportData;

mod background_fill_gradient;
pub use background_fill_gradient::BackgroundFillGradient;

mod chat_member_left;
pub use chat_member_left::ChatMemberLeft;

mod inline_query_result_cached_document;
pub use inline_query_result_cached_document::InlineQueryResultCachedDocument;

mod chosen_inline_result;
pub use chosen_inline_result::ChosenInlineResult;

mod giveaway_completed;
pub use giveaway_completed::GiveawayCompleted;

mod passport_element_error_front_side;
pub use passport_element_error_front_side::PassportElementErrorFrontSide;

mod birthdate;
pub use birthdate::Birthdate;

mod animation;
pub use animation::Animation;

mod forum_topic_created;
pub use forum_topic_created::ForumTopicCreated;

mod poll;
pub use poll::Poll;

mod keyboard_button_request_chat;
pub use keyboard_button_request_chat::KeyboardButtonRequestChat;

mod reply_keyboard_remove;
pub use reply_keyboard_remove::ReplyKeyboardRemove;

mod order_info;
pub use order_info::OrderInfo;

mod inline_query_result;
pub use inline_query_result::InlineQueryResult;

mod web_app_info;
pub use web_app_info::WebAppInfo;

mod input_venue_message_content;
pub use input_venue_message_content::InputVenueMessageContent;

mod bot_command_scope;
pub use bot_command_scope::BotCommandScope;

mod message_reaction_count_updated;
pub use message_reaction_count_updated::MessageReactionCountUpdated;

mod chat_member_banned;
pub use chat_member_banned::ChatMemberBanned;

mod inline_query_result_cached_mpeg_4_gif;
pub use inline_query_result_cached_mpeg_4_gif::InlineQueryResultCachedMpeg4Gif;

mod inline_query_result_cached_voice;
pub use inline_query_result_cached_voice::InlineQueryResultCachedVoice;

mod successful_payment;
pub use successful_payment::SuccessfulPayment;

mod bot_command_scope_chat;
pub use bot_command_scope_chat::BotCommandScopeChat;

mod keyboard_button_poll_type;
pub use keyboard_button_poll_type::KeyboardButtonPollType;

mod mask_position;
pub use mask_position::MaskPosition;

mod file;
pub use file::File;

mod inline_query_result_location;
pub use inline_query_result_location::InlineQueryResultLocation;

mod input_sticker;
pub use input_sticker::InputSticker;

mod chat_photo;
pub use chat_photo::ChatPhoto;
