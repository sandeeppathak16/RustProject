use chrono::NaiveDate;
use diesel::prelude::*;

use crate::schema::{users, notes, tags, note_tags};

#[derive(Debug, Queryable, Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(User))]
#[diesel(table_name = notes)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub tags: String,
    pub body: String,
    pub created: NaiveDate,
    pub updated: NaiveDate,
    pub user_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = notes)]
pub struct NewNote {
    pub title: String,
    pub tags: String,
    pub body: String,
    pub created: NaiveDate,
    pub updated: NaiveDate,
    pub user_id: i32,
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = notes)]
pub struct UpdateNote {
    pub title: Option<String>,
    pub tags: Option<String>,
    pub body: Option<String>,
    pub updated: Option<NaiveDate>,
}

#[derive(Debug, Queryable, Identifiable)]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = tags)]
pub struct NewTag {
    pub name: String,
}

#[derive(
    Debug,
    Queryable,
    Insertable,
    Identifiable,
    Associations,
)]
#[diesel(primary_key(note_id, tag_id))]
#[diesel(belongs_to(Note))]
#[diesel(belongs_to(Tag))]
#[diesel(table_name = note_tags)]
pub struct NoteTag {
    pub note_id: i32,
    pub tag_id: i32,
}