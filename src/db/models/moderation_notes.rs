use diesel::prelude::*;


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::moderation_notes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ModerationNotes {
    pub note_id: i64,
    pub target_id: i64,
    pub moderator_id: i64,
    pub note: String,
    pub message_reference: Option<i64>
}