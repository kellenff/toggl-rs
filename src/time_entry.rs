use crate::project::ProjectID;
use crate::workspace::WorkspaceID;
use crate::Toggl;

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


trait TimeEntryTrait {
    fn get_time_entries(&self) -> Vec<TimeEntry>;
}

impl TimeEntryTrait for Toggl {
    fn get_time_entries(&self) -> Vec<TimeEntry> {
        vec![]
    }
}