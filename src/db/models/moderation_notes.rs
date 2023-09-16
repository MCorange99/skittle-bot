use diesel::prelude::*;

#[allow(non_camel_case_types)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::moderation_notes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct moderation_notes {
    pub note_id: i64,
    pub target_id: i64,
    pub moderator_id: i64,
    pub note_text: String
}