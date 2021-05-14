use crate::client::Client;
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
    pub client: Option<Rc<Client>>,
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

/// Compares if the client id and the possible client id are the same, if `tjsonid` is None, we return false
fn client_cmp(c: &Client, tjsonid: Option<i64>) -> bool {
    tjsonid.map(|v| v == c.id).unwrap_or(false)
}

impl From<TimeEntryWrapper<'_>> for TimeEntry {
    fn from(value: TimeEntryWrapper) -> TimeEntry {
        let workspace = value
            .workspaces
            .iter()
            .find(|ws| ws.id == value.tjson.wid)
            .expect("Workspaces was not filled correctly")
            .clone();
        let project = value
            .projects
            .iter()
            .find(|p| project_cmp(p, value.tjson.pid))
            .cloned();
        let client = value
            .clients
            .iter()
            .find(|c| client_cmp(c, project.as_ref().map(|v| v.cid)))
            .cloned();
        TimeEntry {
            id: value.tjson.id,
            guid: value.tjson.guid,
            workspace,
            client,
            project,
            start: value.tjson.start,
            stop: value.tjson.stop,
            duration: value.tjson.duration,
            description: value.tjson.description.clone(),
            duronly: value.tjson.duronly,
            at: value.tjson.at,
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
pub type TimeEntryRange = Vec<TimeEntryInner>;
pub type TimeEntryRangeSlice<'a> = &'a [TimeEntryInner];
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

// Wrapper containing everything needed to construct a TimeEntry
pub struct TimeEntryWrapper<'a> {
    clients: &'a [Rc<Client>],
    projects: &'a [Rc<Project>],
    workspaces: &'a [Rc<Workspace>],
    tjson: &'a TimeEntryInner,
}

impl<'a> TimeEntryWrapper<'a> {
    pub fn new(
        clients: &'a [Rc<Client>],
        projects: &'a [Rc<Project>],
        workspaces: &'a [Rc<Workspace>],
        tjson: &'a TimeEntryInner,
    ) -> Self {
        TimeEntryWrapper {
            clients,
            projects,
            workspaces,
            tjson,
        }
    }
}
