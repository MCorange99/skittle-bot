use diesel::prelude::*;

#[allow(non_camel_case_types)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::moderation_kicks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct moderation_kicks {
    pub kick_id: i64,
    pub target_id: i64,
    pub moderator_id: i64,
    pub kick_reason: Option<String>
}