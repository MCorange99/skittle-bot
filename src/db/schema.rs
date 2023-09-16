// @generated automatically by Diesel CLI.

diesel::table! {
    core_associations (association_id) {
        association_id -> Integer,
        user_id -> Integer,
        associated_user_id -> Integer,
    }
}

diesel::table! {
    core_users (user_id) {
        user_id -> Integer,
    }
}

diesel::table! {
    moderation_bans (ban_id) {
        ban_id -> Integer,
        target_id -> Integer,
        moderator_id -> Integer,
        ban_reason -> Nullable<Text>,
        ban_duration -> Nullable<Integer>,
    }
}

diesel::table! {
    moderation_kicks (kick_id) {
        kick_id -> Integer,
        target_id -> Integer,
        moderator_id -> Integer,
        kick_reason -> Text,
    }
}

diesel::table! {
    moderation_message_references (entry_id) {
        entry_id -> Integer,
        message_id -> Integer,
        note -> Nullable<Text>,
    }
}

diesel::table! {
    moderation_messages (message_id) {
        message_id -> Integer,
        message_link -> Text,
    }
}

diesel::table! {
    moderation_mutes (mute_id) {
        mute_id -> Integer,
        target_id -> Integer,
        moderator_id -> Integer,
        reason -> Nullable<Text>,
        timestamp -> Double,
        duration -> Nullable<Double>,
    }
}

diesel::table! {
    moderation_notes (note_id) {
        note_id -> Integer,
        target_id -> Integer,
        moderator_id -> Integer,
        note_text -> Text,
        message_reference -> Nullable<Integer>,
    }
}

diesel::table! {
    moderation_reports (report_id) {
        report_id -> Integer,
        target_id -> Integer,
        moderator_id -> Integer,
        report_title -> Text,
        report_description -> Text,
        timestamp -> Integer,
    }
}

diesel::joinable!(core_associations -> core_users (user_id));
diesel::joinable!(moderation_message_references -> moderation_messages (message_id));
diesel::joinable!(moderation_notes -> moderation_message_references (message_reference));

diesel::allow_tables_to_appear_in_same_query!(
    core_associations,
    core_users,
    moderation_bans,
    moderation_kicks,
    moderation_message_references,
    moderation_messages,
    moderation_mutes,
    moderation_notes,
    moderation_reports,
);
