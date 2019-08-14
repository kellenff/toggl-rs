use crate::project::Project;
use crate::workspace::Workspace;
use std::cmp::Ordering;
use std::rc::Rc;

/// The base type for all returned data
#[derive(Deserialize, Serialize, Debug)]
pub struct Return<T> {
    pub data: T,
}

/// The Main struct for the timeentry.
/// Will have a Rc to the project and workspace it belongs to.
#[derive(Clone, Debug, Eq)]
pub struct TimeEntry {
    pub id: i64,
    pub guid: uuid::Uuid,
    pub workspace: Rc<Workspace>,
    pub project: Option<Rc<Project>>,
    pub start: chrono::DateTime<chrono::Utc>,
    pub stop: Option<chrono::DateTime<chrono::Utc>>,
    pub duration: i64,
    pub description: Option<String>,
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

/// Compares if the project id and the possible project id are the same, if `tjsonid` is None, we return false
fn project_cmp(p: &Project, tjsonid: Option<i64>) -> bool {
    tjsonid.map(|v| v == p.id).unwrap_or(false)
}

impl From<(&Vec<Rc<Project>>, &Vec<Rc<Workspace>>, &TimeEntryInner)> for TimeEntry {
    fn from(value: (&Vec<Rc<Project>>, &Vec<Rc<Workspace>>, &TimeEntryInner)) -> TimeEntry {
        let p = value.0;
        let w = value.1;
        let tjson = value.2;
        let workspace = w
            .iter()
            .find(|ws| ws.id == tjson.wid)
            .expect("Workspaces was not filled correctly")
            .clone();
        let project = p.iter().find(|p| project_cmp(p, tjson.pid)).cloned();
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
}

/// The Inner Type for the return from StartEntryCall
#[derive(Deserialize, Debug)]
pub struct StartEntryReturnInner {
    id: i64,
    pid: Option<i64>,
    wid: i64,
    billable: bool,
    start: chrono::DateTime<chrono::Utc>,
    tags: Option<Vec<String>>,
    duration: i64,
    description: Option<String>,
}

/// TimeEntry format that comes from the json api. Notice that it includes ids and not workspace/projects.
#[derive(Deserialize, Debug, Serialize)]
pub struct TimeEntryInner {
    pub id: i64,
    pub guid: uuid::Uuid,
    /// Workspace id
    pub wid: i64,
    /// Project id
    pub pid: Option<i64>,
    /// Start time, will be parsed into Utc
    pub start: chrono::DateTime<chrono::Utc>,
    /// End time (optional), will be parsed into Utc
    pub stop: Option<chrono::DateTime<chrono::Utc>>,
    pub duration: i64,
    pub description: Option<String>,
    pub duronly: bool,
    pub at: chrono::DateTime<chrono::Utc>,
}

pub type StartEntryReturn = Return<StartEntryReturnInner>;

//yes they seem to be the same
pub type StopEntryReturn = Return<StartEntryReturnInner>;
pub type TimeEntryReturn = Return<Option<TimeEntryInner>>;
pub type TimeEntryRangeReturn = Vec<TimeEntryInner>;
pub type DeleteEntryReturn = Vec<i64>;

#[derive(Serialize, Debug)]
pub struct TimeEntryUpdate {
    time_entry: TimeEntryInner,
}

impl From<TimeEntry> for TimeEntryUpdate {
    fn from(t: TimeEntry) -> Self {
        TimeEntryUpdate {
            time_entry: TimeEntryInner {
                id: t.id,
                guid: t.guid,
                wid: t.workspace.id,
                pid: t.project.map(|v| v.id),
                start: t.start,
                stop: t.stop,
                duration: t.duration,
                description: t.description,
                duronly: t.duronly,
                at: t.at,
            },
        }
    }
}
