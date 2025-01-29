use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Serialize, Deserialize};
use std::fmt;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Tag{
    HIGH,
    LOW,
    MEDIUM
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Match the enum variants and output the desired string
        match *self {
            Tag::HIGH => write!(f, "HIGH"),
            Tag::LOW => write!(f, "LOW"),
            Tag::MEDIUM => write!(f, "MEDIUm"),
        }
    }
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum State{
    PENDING,
    ONGOING,
    DONE
}
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Match the enum variants and output the desired string
        match *self {
            State::PENDING => write!(f, "PENDING"),
            State::ONGOING => write!(f, "ONGOING"),
            State::DONE => write!(f, "DONE"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo{
    #[serde(rename = "_id", skip_serializing_if ="Option::is_none")]
    pub id : Option<ObjectId>,
    pub title : String,
    pub position : Option<i32>,
    pub description : String,
    pub tag : Tag,
    pub state : State,
    pub date : Option<DateTime>,
    pub created_at: Option<DateTime>
}
