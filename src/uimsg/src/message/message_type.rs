use serde::*;
use uuid::Uuid;


#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum MessageType {
    ObjectCreated(Uuid, String, String),
    ObjectRemoved(Uuid, Uuid),
    ValueUpdated(Uuid, String, String),
    SetupScrollView(Uuid, String),
    SetupListView(Uuid, String),
    AddNode(String),
    RemoveNode(String),
    SetRoot(String)
}