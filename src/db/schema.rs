// @generated automatically by Diesel CLI.

diesel::table! {
    core_associations (association_id) {
        association_id -> Nullable<BigInt>,
        user_id -> BigInt,
        associated_user_id -> BigInt,
    }
}

diesel::table! {
    core_users (user_id) {
        user_id -> Nullable<BigInt>,
        user_name -> Text,
        user_nickname -> Text,
        user_created_at -> BigInt,
        user_role -> BigInt,
    }
}

diesel::table! {
    moderation_bans (ban_id) {
        ban_id -> Nullable<BigInt>,
        target_id -> BigInt,
        moderator_id -> BigInt,
        ban_reason -> Nullable<Text>,
        ban_duration -> Nullable<BigInt>,
    }
}

diesel::table! {
    moderation_kicks (kick_id) {
        kick_id -> Nullable<BigInt>,
        target_id -> BigInt,
        moderator_id -> BigInt,
        kick_reason -> Nullable<Text>,
    }
}

diesel::table! {
    moderation_message_references (message_reference, entry_id) {
        message_reference -> Nullable<BigInt>,
        entry_id -> Nullable<BigInt>,
        message_id -> BigInt,
        note -> Nullable<Text>,
    }
}

diesel::table! {
    moderation_messages (message_id) {
        message_id -> Nullable<BigInt>,
        message_link -> Text,
    }
}

diesel::table! {
    moderation_mutes (mute_id) {
        mute_id -> Nullable<BigInt>,
        target_id -> BigInt,
        moderator_id -> BigInt,
        reason -> Nullable<Text>,
        timestamp -> Double,
        duration -> Nullable<Double>,
    }
}

diesel::table! {
    moderation_notes (note_id) {
        note_id -> Nullable<BigInt>,
        target_id -> BigInt,
        moderator_id -> BigInt,
        note -> Text,
        message_reference -> Nullable<BigInt>,
    }
}

diesel::table! {
    moderation_reports (report_id) {
        report_id -> Nullable<BigInt>,
        target_id -> BigInt,
        moderator_id -> BigInt,
        report_title -> Nullable<Text>,
        report_description -> Nullable<Text>,
        timestamp -> Nullable<Double>,
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
