use diesel::prelude::*;


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::moderation_bans)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ModerationBans {
    pub ban_id: i64,
    pub target_id: i64,
    pub moderator_id: i64,
    pub ban_reason: Option<String>,
    pub ban_duration: Option<i64>,
}