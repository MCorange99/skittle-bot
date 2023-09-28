use diesel::prelude::*;


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::moderation_message_references)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ModerationMessageReferences {
    pub entry_id: i64,
    pub message_id: i64,
    pub note: String,
}