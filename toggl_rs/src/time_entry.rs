use std::rc::Rc;

use crate::project::{Project, ProjectTrait};
use crate::workspace::Workspace;
use crate::Toggl;
use crate::error::TogglError;
use crate::Query;

#[derive(Debug)]
pub struct TimeEntry {
    id: i64,
    guid: uuid::Uuid,
    workspace: Rc<Workspace>,
    project: Rc<Project>,
    start: chrono::DateTime<chrono::Utc>,
    stop: chrono::DateTime<chrono::Utc>,
    duration: i64,
    description: String,
    duronly: bool,
    at: chrono::DateTime<chrono::Utc>,
    uuid: uuid::Uuid,
}


#[derive(Deserialize,Debug,Serialize)]
struct TimeEntryJSON {
    id: i64,
    guid: uuid::Uuid,
    wid: i64,
    pid: i64,
    start: chrono::DateTime<chrono::Utc>,
    stop: chrono::DateTime<chrono::Utc>,
    duration: i64,
    description: String,
    duronly: bool,
    at: chrono::DateTime<chrono::Utc>,
    uuid: uuid::Uuid,
}

impl From<TimeEntry> for TimeEntryJSON {
    fn from(t: TimeEntry) -> Self {
        TimeEntryJSON {
            id: t.id,
            guid: t.guid,
            wid: t.workspace.id,
            pid: t.project.id,
            start: t.start,
            stop: t.stop,
            duration: t.duration,
            description: t.description,
            duronly: t.duronly,
            at: t.at,
            uuid: t.uuid,
        }
    }
}

#[derive(Serialize, Debug)]
struct StartEntry {
    description: String,
    tags: Vec<String>,
    pid: i64,
}

fn convert(p: &[Rc<Project>], w: &[Rc<Workspace>], tjson: &TimeEntryJSON) -> TimeEntry {
    let workspace = w.iter().find(|ws| ws.id == tjson.wid).expect("Workspaces was not filled correctly").clone();
    let project = p.iter().find(|p| p.id == tjson.pid).expect("Projects was not filled correctly").clone();
    TimeEntry {
        id: tjson.id,
        guid: tjson.guid,
        workspace,
        project,
        start: tjson.start,
        stop: tjson.stop,
        duration: tjson.duration,
        description: tjson.description,
        duronly: tjson.duronly,
        at: tjson.at,
        uuid: tjson.uuid,
    }
}

trait TimeEntryTrait {
    fn get_time_entries(&mut self) -> Result<Vec<TimeEntry>, TogglError>;
    fn get_time_entries_range(&self, start: Option<chrono::DateTime<chrono::Utc>>, end: Option<chrono::DateTime<chrono::Utc>>) -> Result<Vec<TimeEntry>, TogglError>;
    fn start_entry(&self, description: &str, tags: &[String], p: &Project) -> Result<(), TogglError>;
    fn stop_entry(&self, t: &TimeEntry) -> Result<(), TogglError>;
    fn get_entry_details(&self, id: i64) -> Result<TimeEntry, TogglError>;
    fn get_running_entry(&self) -> Result<TimeEntry, TogglError>;
    fn update_entry(&self, t: TimeEntry) -> Result<(), TogglError>;
    fn delete_entry(&self, t: &TimeEntry) -> Result<(), TogglError>;
    fn convert_response(&self, t: &[TimeEntryJSON]) -> Vec<TimeEntry>;
}

impl TimeEntryTrait for Toggl {
    fn get_time_entries(&mut self) -> Result<Vec<TimeEntry>, TogglError> {
        self.get_time_entries_range(None, None)
    }

    fn get_time_entries_range(&self, start: Option<chrono::DateTime<chrono::Utc>>,
        end: Option<chrono::DateTime<chrono::Utc>>) -> Result<Vec<TimeEntry>, TogglError> {
            let url = if let Some(s) = start {
                if let Some(e) = end {
                    format!("https://www.toggl.com/api/v8/time_entries?start_date={}&end_date={}", s, e)
                } else {
                    format!("https://www.toggl.com/api/v8/time_entries?start_date={}", s)
                }
            } else {
                if let Some(e) = end {
                    format!("https://www.toggl.com/api/v8/time_entries?end_date={}", e)
                } else {
                    format!("https://www.toggl.com/api/v8/time_entries")
                }
            };
                    if self.projects.is_none() {
            self.fill_projects();
        }

        let p = self.projects.as_ref().unwrap();

        let res: Vec<TimeEntryJSON> = self.get("https://www.toggl.com/api/v8/time_entries")?;
        Ok(self.convert_response(&res))
    }

    fn start_entry(&self, description: &str, tags: &[String], p: &Project) -> Result<(), TogglError> {
        let t = StartEntry {
            description: description.to_owned(),
            tags: tags.to_owned(),
            pid: p.id
        };
        self.post("https://www.toggl.com/api/v8/time_entries/start", &t)
    }

    fn stop_entry(&self, t: &TimeEntry) -> Result<(), TogglError> {
        self.put::<i64>(&format!("https://www.toggl.com/api/v8/time_entries/{}/stop", t.id), &None)?;
        Ok(())
    }

    fn get_entry_details(&self, id: i64) -> Result<TimeEntry, TogglError> {
        self.get(&format!("https://www.toggl.com/api/v8/time_entries/{}", id))
            .map(|r| self.convert_response(&[r]))
            .map(|v| v[0])
    }

    fn get_running_entry(&self) -> Result<TimeEntry, TogglError> {
        self.get("https://www.toggl.com/api/v8/time_entries/current")
            .map(|r| self.convert_response(&[r]))
            .map(|v| v[0])
    }

    fn update_entry(&self, t: TimeEntry) -> Result<(), TogglError> {
        let entry: TimeEntryJSON = t.into();
        self.put(&format!("https://www.toggl.com/api/v8/time_entries/{}", t.id), &Some(entry))
    }

    fn delete_entry(&self, t: &TimeEntry) -> Result<(), TogglError> {
        self.delete(&format!("https://www.toggl.com/api/v8/time_entries/{}", t.id))
    }

    fn convert_response(&self, res: &[TimeEntryJSON]) -> Vec<TimeEntry> {
        res
        .into_iter()
        .map(|tjson| convert(&self.projects.unwrap_or([].to_vec()), &self.user.workspaces, tjson))
        .collect()
    }
}