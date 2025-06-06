use chrono::{NaiveDateTime};

#[derive(Debug)]
pub struct Note {
    pub id: i64,
    pub name: String,
    pub content: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NoteIdName {
    pub id: i64,
    pub name: String,
    pub children: Vec<NoteIdName>,
    pub has_parent: bool,
}

#[derive(Debug)]
pub enum LinkType {
    Related,
    Parent,
}

#[derive(Debug)]
pub struct NoteLink {
    pub id: i64,
    pub source_note_id: i64,
    pub target_note_id: i64,
    pub link_type: LinkType, // parent or related
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug)]
pub struct NoteLinkIds {
    pub source_note_id: i64,
    pub target_note_id: i64,
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

impl ToString for LinkType {
    fn to_string(&self) -> String {
        match self {
            LinkType::Related => "related".to_string(),
            LinkType::Parent => "parent".to_string(),
        }
    }
}
