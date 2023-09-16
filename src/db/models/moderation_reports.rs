use diesel::prelude::*;

#[allow(non_camel_case_types)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::moderation_reports)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct moderation_reports {
    pub report_id: i64,
    pub target_id: i64,
    pub moderator_id: i64,
    pub report_title: String,
    pub report_description: String,
    pub timestamp: i64,
}