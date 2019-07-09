use std::rc::Rc;

use crate::error::TogglError;

use crate::project::{Project, ProjectTrait};
use crate::return_types::{convert, StartEntryReturn, StopEntryReturn, TimeEntry, TimeEntryReturn};
use crate::workspace::Workspace;
use crate::Query;
use crate::Toggl;


#[derive(Serialize, Debug)]
struct StartEntry {
    time_entry: StartTimeEntry,
}

#[derive(Serialize, Debug)]
struct StartTimeEntry {
    description: String,
    tags: Vec<String>,
    pid: i64,
    created_with: String,
}

pub trait TimeEntryExt {
    fn get_time_entries(&mut self) -> Result<Vec<TimeEntry>, TogglError>;
    fn get_time_entries_range(
        &mut self,
        start: Option<chrono::DateTime<chrono::Utc>>,
        end: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Vec<TimeEntry>, TogglError>;
    fn start_entry(
        &self,
        description: &str,
        tags: &[String],
        p: &Project,
    ) -> Result<(), TogglError>;
    fn stop_entry(&self, t: &TimeEntry) -> Result<(), TogglError>;
    fn get_entry_details(&self, id: i64) -> Result<TimeEntry, TogglError>;
    fn get_running_entry(&self) -> Result<Option<TimeEntry>, TogglError>;
    fn update_entry(&self, t: TimeEntry) -> Result<(), TogglError>;
    fn delete_entry(&self, t: &TimeEntry) -> Result<(), TogglError>;
}

trait TimeEntryTrait {
    /// Converts an array of TimeEntryReturn to Vector of TimeEntry discarding any elements where the data of Return<TimeEntryInner> is None
    fn convert_response(&self, t: &[TimeEntryReturn]) -> Vec<TimeEntry>;
}

impl TimeEntryExt for Toggl {
    fn get_time_entries(&mut self) -> Result<Vec<TimeEntry>, TogglError> {
        self.get_time_entries_range(None, None)
    }

    fn get_time_entries_range(
        &mut self,
        start: Option<chrono::DateTime<chrono::Utc>>,
        end: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Vec<TimeEntry>, TogglError> {
        let url = if let Some(s) = start {
            if let Some(e) = end {
                format!(
                    "https://www.toggl.com/api/v8/time_entries?start_date={}&end_date={}",
                    s, e
                )
            } else {
                format!("https://www.toggl.com/api/v8/time_entries?start_date={}", s)
            }
        } else if let Some(e) = end {
            format!("https://www.toggl.com/api/v8/time_entries?end_date={}", e)
        } else {
            "https://www.toggl.com/api/v8/time_entries".to_string()
        };
        if self.projects.is_none() {
            self.fill_projects();
        }
        let res: Vec<TimeEntryReturn> = self.get(&url)?;
        Ok(self.convert_response(&res))
    }

    /// This starts the entry with the `description` and the tags given by `tags` in the project `project`. It automatically parses the return values to see if we have a valid return and the operation was successful.
    /// This automatically stops the current running entry (serverside).
    fn start_entry(
        &self,
        description: &str,
        tags: &[String],
        p: &Project,
    ) -> Result<(), TogglError> {
        let t = StartEntry {
            time_entry: StartTimeEntry {
                description: description.to_owned(),
                tags: tags.to_owned(),
                pid: p.id,
                created_with: "toggl-rs".to_string(),
            },
        };
        self.post::<StartEntry, StartEntryReturn>(
            "https://www.toggl.com/api/v8/time_entries/start",
            &t,
        )?;
        Ok(())
    }

    /// Stops the given entry
    fn stop_entry(&self, t: &TimeEntry) -> Result<(), TogglError> {
        self.put::<i64, StopEntryReturn>(
            &format!("https://www.toggl.com/api/v8/time_entries/{}/stop", t.id),
            &None,
        )?;
        Ok(())
    }

    fn get_entry_details(&self, id: i64) -> Result<TimeEntry, TogglError> {
        self.get(&format!("https://www.toggl.com/api/v8/time_entries/{}", id))
            .map(|r| self.convert_response(&[r]))
            .map(|mut v| v.swap_remove(0)) //this makes the borrowchecker work
    }

    /// Returns the current running entry or None
    /// Throws an error if there was a problem with the api
    fn get_running_entry(&self) -> Result<Option<TimeEntry>, TogglError> {
        let res: Result<TimeEntryReturn, crate::error::TogglError> =
            self.get("https://www.toggl.com/api/v8/time_entries/current");
        //panic!("{:?}", res);

        self.get("https://www.toggl.com/api/v8/time_entries/current")
            .map(|r| self.convert_response(&[r]))
            .map(|mut v| v.pop())
    }

    fn update_entry(&self, t: TimeEntry) -> Result<(), TogglError> {
        panic!("not yet implemented");
        let entry: TimeEntryReturn = t.into();
        //self.put(
        //    &format!("https://www.toggl.com/api/v8/time_entries/{}", entry.data.id),
        //    &Some(entry))
    }

    fn delete_entry(&self, t: &TimeEntry) -> Result<(), TogglError> {
        panic!("not yet implemented");
        self.delete(&format!(
            "https://www.toggl.com/api/v8/time_entries/{}",
            t.id
        ))
    }

}

impl TimeEntryTrait for Toggl {
    fn convert_response(&self, res: &[TimeEntryReturn]) -> Vec<TimeEntry> {
        res.iter()
            .filter_map(|tjson| {
                if let Some(ref t) = tjson.data {
                    Some(convert(
                        self.projects.as_ref().unwrap_or(&[].to_vec()),
                        &self.user.workspaces,
                        t,
                    ))
                } else {
                    None
                }
            })
            .collect()
    }
}
