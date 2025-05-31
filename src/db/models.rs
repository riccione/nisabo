use chrono::{NaiveDateTime, Utc};

#[derive(Debug)]
pub struct Note {
    pub id: i32,
    pub name: String,
    pub content: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Debug)]
pub struct NoteIdName {
    pub id: i32,
    pub name: String,
}

#[derive(Debug)]
pub enum LinkType {
    Related,
    Parent,
}

#[derive(Debug)]
pub struct NoteLink {
    pub id: i32,
    pub source_note_id: i32,
    pub target_note_id: i32,
    pub link_type: LinkType, // parent or related
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl std::str::FromStr for LinkType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "related" => Ok(LinkType::Related),
            "parent" => Ok(LinkType::Parent),
            _ => Err(()),
        }
    }
}
