// @generated automatically by Diesel CLI.

diesel::table! {
    core_associations (association_id) {
        association_id -> BigInt,
        user_id -> BigInt,
        associated_user_id -> BigInt,
    }
}

diesel::table! {
    core_users (user_id) {
        user_id -> BigInt,
        user_role -> BigInt,
    }
}

diesel::table! {
    moderation_bans (ban_id) {
        ban_id -> BigInt,
        target_id -> BigInt,
        moderator_id -> BigInt,
        ban_reason -> Nullable<Text>,
        ban_duration -> Nullable<BigInt>,
    }
}

diesel::table! {
    moderation_kicks (kick_id) {
        kick_id -> BigInt,
        target_id -> BigInt,
        moderator_id -> BigInt,
        kick_reason -> Nullable<Text>,
    }
}

diesel::table! {
    moderation_message_references (message_reference, entry_id) {
        message_reference -> BigInt,
        entry_id -> BigInt,
        message_id -> BigInt,
        note -> Text,
    }
}

diesel::table! {
    moderation_messages (message_id) {
        message_id -> BigInt,
        message_link -> Text,
    }
}

diesel::table! {
    moderation_mutes (mute_id) {
        mute_id -> BigInt,
        target_id -> BigInt,
        moderator_id -> BigInt,
        reason -> Nullable<Text>,
        timestamp -> BigInt,
        duration -> Nullable<BigInt>,
    }
}

diesel::table! {
    moderation_notes (note_id) {
        note_id -> BigInt,
        target_id -> BigInt,
        moderator_id -> BigInt,
        note -> Text,
        message_reference -> Nullable<BigInt>,
    }
}

diesel::table! {
    moderation_reports (report_id) {
        report_id -> BigInt,
        target_id -> BigInt,
        moderator_id -> BigInt,
        report_title -> Text,
        report_description -> Text,
        timestamp -> BigInt,
    }
}

diesel::joinable!(moderation_message_references -> moderation_messages (message_id));

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
