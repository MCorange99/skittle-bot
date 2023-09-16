use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::moderation_mutes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[allow(non_camel_case_types)]
pub struct moderation_mutes {
    pub mute_id: i64,
    pub target_id: i64,
    pub moderator_id: i64,
    pub reason: Option<String>,
    pub timestamp: i64,
    #[allow(trivial_bounds)]
    pub duration: Option<i64>
}