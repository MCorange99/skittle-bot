use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::moderation_mutes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]

pub struct ModerationMutes {
    pub mute_id: i64,
    pub target_id: i64,
    pub moderator_id: i64,
    pub reason: Option<String>,
    pub timestamp: i64,
    pub duration: Option<i64>
}