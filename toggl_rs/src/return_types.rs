use crate::project::Project;
use crate::workspace::Workspace;
use std::rc::Rc;
use std::cmp::Ordering;

/// The base type for all returned data
#[derive(Deserialize, Debug)]
pub struct Return<T> {
    pub data: T,
}

#[derive(Debug, Eq)]
pub struct TimeEntry {
    pub id: i64,
    pub guid: uuid::Uuid,
    pub workspace: Rc<Workspace>,
    pub project: Rc<Project>,
    pub start: chrono::DateTime<chrono::Utc>,
    pub stop: Option<chrono::DateTime<chrono::Utc>>,
    pub duration: i64,
    pub description: String,
    pub duronly: bool,
    pub at: chrono::DateTime<chrono::Utc>,
}

impl PartialEq for TimeEntry {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for TimeEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.start.cmp(&other.start))
    }
}

impl Ord for TimeEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl From<TimeEntry> for TimeEntryReturn {
    fn from(t: TimeEntry) -> Self {
        TimeEntryReturn {
            data: Some(TimeEntryInner {
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
            }),
        }
    }
}

pub fn convert(p: &[Rc<Project>], w: &[Rc<Workspace>], tjson: &TimeEntryInner) -> TimeEntry {
    let workspace = w
        .iter()
        .find(|ws| ws.id == tjson.wid)
        .expect("Workspaces was not filled correctly")
        .clone();
    let project = p
        .iter()
        .find(|p| p.id == tjson.pid)
        .expect("Projects was not filled correctly")
        .clone();
    TimeEntry {
        id: tjson.id,
        guid: tjson.guid,
        workspace,
        project,
        start: tjson.start,
        stop: tjson.stop,
        duration: tjson.duration,
        description: tjson.description.clone(),
        duronly: tjson.duronly,
        at: tjson.at,
    }
}

/// The Inner Type for the return from StartEntryCall
#[derive(Deserialize, Debug)]
pub struct StartEntryReturnInner {
    id: i64,
    pid: i64,
    wid: i64,
    billable: bool,
    start: chrono::DateTime<chrono::Utc>,
    tags: Option<Vec<String>>,
    duration: i64,
    description: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct TimeEntryInner {
    pub id: i64,
    pub guid: uuid::Uuid,
    pub wid: i64,
    pub pid: i64,
    pub start: chrono::DateTime<chrono::Utc>,
    pub stop: Option<chrono::DateTime<chrono::Utc>>,
    pub duration: i64,
    pub description: String,
    pub duronly: bool,
    pub at: chrono::DateTime<chrono::Utc>,
}

pub type StartEntryReturn = Return<StartEntryReturnInner>;

//yes they seem to be the same
pub type StopEntryReturn = Return<StartEntryReturnInner>;
pub type TimeEntryReturn = Return<Option<TimeEntryInner>>;
pub type TimeEntryRangeReturn = Vec<TimeEntryInner>;