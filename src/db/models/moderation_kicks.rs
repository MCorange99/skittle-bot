use diesel::prelude::*;


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::moderation_kicks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ModerationKicks {
    pub kick_id: i64,
    pub target_id: i64,
    pub moderator_id: i64,
    pub kick_reason: Option<String>
}