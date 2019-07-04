use crate::project::ProjectID;
use crate::workspace::WorkspaceID;

#[derive(Deserialize,Debug,Serialize)]
struct TimeEntry {
    id: i64,
    guid: uuid::Uuid,
    wid: WorkspaceID,
    pid: ProjectID,
    start: chrono::DateTime<chrono::Utc>,
    stop: chrono::DateTime<chrono::Utc>,
    duration: i64,
    description: String,
    duronly: bool,
    at: chrono::DateTime<chrono::Utc>,
    uuid: uuid::Uuid,
}

pub fn get_time_entries(t: &Toggl, from: Option<chrono::DateTime<chrono::Utc>>, to: Option<chrono::DateTime<chrono::Utc>>) -> Vec<TimeEntry> {
    vec![]
}

pub fn get_populated_time_entries(t: &Toggl, w: &Vec<Workspace>) -> () {

}