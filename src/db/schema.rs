// @generated automatically by Diesel CLI.

diesel::table! {
    core_associations (association_id) {
        association_id -> Nullable<Integer>,
        user_id -> Integer,
        associated_user_id -> Integer,
    }
}

diesel::table! {
    core_users (user_id) {
        user_id -> Nullable<Integer>,
    }
}

diesel::table! {
    message_references (message_reference, entry_id) {
        message_reference -> Nullable<Integer>,
        entry_id -> Nullable<Integer>,
        message_id -> Integer,
        note -> Nullable<Text>,
    }
}

diesel::table! {
    moderation_bans (ban_id) {
        ban_id -> Nullable<Integer>,
        target_id -> Integer,
        moderator_id -> Integer,
        ban_reason -> Nullable<Text>,
        ban_duration -> Nullable<Double>,
    }
}

diesel::table! {
    moderation_kicks (kick_id) {
        kick_id -> Nullable<Integer>,
        target_id -> Integer,
        moderator_id -> Integer,
        kick_reason -> Nullable<Text>,
    }
}

diesel::table! {
    moderation_messages (message_id) {
        message_id -> Nullable<Integer>,
        message_link -> Text,
    }
}

diesel::table! {
    moderation_mutes (mute_id) {
        mute_id -> Nullable<Integer>,
        target_id -> Integer,
        moderator_id -> Integer,
        reason -> Nullable<Text>,
        timestamp -> Double,
        duration -> Nullable<Double>,
    }
}

diesel::table! {
    moderation_notes (note_id) {
        note_id -> Nullable<Integer>,
        target_id -> Integer,
        moderator_id -> Integer,
        note -> Text,
        message_reference -> Nullable<Integer>,
    }
}

diesel::table! {
    moderation_reports (report_id) {
        report_id -> Nullable<Integer>,
        target_id -> Integer,
        moderator_id -> Integer,
        report_title -> Nullable<Text>,
        report_description -> Nullable<Text>,
        timestamp -> Nullable<Double>,
    }
}

diesel::joinable!(message_references -> moderation_messages (message_id));

diesel::allow_tables_to_appear_in_same_query!(
    core_associations,
    core_users,
    message_references,
    moderation_bans,
    moderation_kicks,
    moderation_messages,
    moderation_mutes,
    moderation_notes,
    moderation_reports,
);
