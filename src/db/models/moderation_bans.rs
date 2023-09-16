use diesel::prelude::*;

#[allow(non_camel_case_types)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::moderation_bans)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct moderation_bans {
    pub ban_id: i64,
    pub target_id: i64,
    pub moderator_id: i64,
    pub ban_reason: Option<String>,
    pub ban_duration: i64,
}