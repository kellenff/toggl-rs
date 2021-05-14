use crate::error::TogglError;

use crate::project::Project;
use crate::types::{
    DeleteEntryReturn, StartEntryReturn, StopEntryReturn, TimeEntry, TimeEntryRange,
    TimeEntryRangeSlice, TimeEntryReturn, TimeEntryUpdate, TimeEntryWrapper,
};
use crate::Query;
use crate::Toggl;

#[derive(Serialize, Debug)]
struct StartEntry {
    time_entry: StartTimeEntry,
}

#[derive(Serialize, Debug)]
struct StartTimeEntry {
    description: Option<String>,
    tags: Vec<String>,
    pid: Option<i64>,
    created_with: String,
}

/// Main Trait for working with time entries on the toggl struct.
pub trait TimeEntryExt {
    /// Get all time entries from the api.
    fn get_time_entries(&self) -> Result<Vec<TimeEntry>, TogglError>;

    /// Get all time entries from the specified range (both are optional arguments)
    fn get_time_entries_range(
        &self,
        start: Option<chrono::DateTime<chrono::Utc>>,
        end: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Vec<TimeEntry>, TogglError>;

    /// Starts a time entry with the description, tags and a given project.
    fn start_entry<T: AsRef<Project>>(
        &self,
        description: Option<String>,
        tags: &[String],
        p: Option<T>,
    ) -> Result<(), TogglError>;

    /// Stops the supplied time entry. While we technically only look at the id, this is not guaranteed by updates in the api
    fn stop_entry(&self, t: &TimeEntry) -> Result<(), TogglError>;

    /// Returns the time entry for the given id
    fn get_entry_details(&self, id: i64) -> Result<Option<TimeEntry>, TogglError>;

    /// Returns the currently running entry (i.e., a time entry that has no end time) or returns None if it does not exist
    fn get_running_entry(&self) -> Result<Option<TimeEntry>, TogglError>;

    /// Update the time entry with all values that in the time entry. Notice that we need move semantics here.
    /// # Example
    /// ```no_run
    /// use toggl_rs::Toggl;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     use toggl_rs::time_entry::TimeEntryExt;
    /// let t = Toggl::init("api_token")?;
    ///
    ///     let mut entry = t.get_running_entry()?.unwrap();
    ///     entry.description = Some("test2".to_string());
    ///     t.update_entry(entry)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    fn update_entry(&self, t: TimeEntry) -> Result<(), TogglError>;

    /// Deletes the entry.
    fn delete_entry(&self, t: &TimeEntry) -> Result<(), TogglError>;
}

trait TimeEntryTrait {
    /// Converts an array of TimeEntryReturn to Vector of TimeEntry discarding any elements where the data of Return<TimeEntryInner> is None
    fn convert_response(&self, t: TimeEntryRangeSlice) -> Vec<TimeEntry>;

    fn convert_single(&self, res: &TimeEntryReturn) -> Option<TimeEntry>;
}

impl TimeEntryExt for Toggl {
    fn get_time_entries(&self) -> Result<Vec<TimeEntry>, TogglError> {
        self.get_time_entries_range(None, None)
    }

    fn get_time_entries_range(
        &self,
        start: Option<chrono::DateTime<chrono::Utc>>,
        end: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Vec<TimeEntry>, TogglError> {
        let mut entries = Vec::new();
        if let Some(s) = start {
            entries.push(("start_date", s.to_rfc3339()));
        }
        if let Some(e) = end {
            entries.push(("end_date", e.to_rfc3339()));
        }

        let url =
            reqwest::Url::parse_with_params("https://www.toggl.com/api/v8/time_entries", entries)
                .expect("Error in parsing URL");

        let res: TimeEntryRange = self.get(url)?;
        Ok(self.convert_response(res.as_slice()))
    }

    fn start_entry<T: AsRef<Project>>(
        &self,
        description: Option<String>,
        tags: &[String],
        p: Option<T>,
    ) -> Result<(), TogglError> {
        let t = StartEntry {
            time_entry: StartTimeEntry {
                description,
                tags: tags.to_owned(),
                pid: p.map(|v| v.as_ref().id),
                created_with: "toggl-rs".to_string(),
            },
        };
        self.post::<&str, StartEntry, StartEntryReturn>(
            "https://www.toggl.com/api/v8/time_entries/start",
            &t,
        )?;
        Ok(())
    }

    fn stop_entry(&self, t: &TimeEntry) -> Result<(), TogglError> {
        self.get::<&str, StopEntryReturn>(&format!(
            "https://www.toggl.com/api/v8/time_entries/{}/stop",
            t.id
        ))?;
        Ok(())
    }

    fn get_entry_details(&self, id: i64) -> Result<Option<TimeEntry>, TogglError> {
        self.get::<&str, TimeEntryReturn>(&format!(
            "https://www.toggl.com/api/v8/time_entries/{}",
            id
        ))
        .map(|r| self.convert_single(&r))
    }

    fn get_running_entry(&self) -> Result<Option<TimeEntry>, TogglError> {
        self.get("https://www.toggl.com/api/v8/time_entries/current")
            .map(|r| self.convert_single(&r))
    }

    fn update_entry(&self, t: TimeEntry) -> Result<(), TogglError> {
        let id = t.id;
        let entry: TimeEntryUpdate = t.into();
        self.put::<&str, TimeEntryUpdate, TimeEntryReturn>(
            &format!("https://www.toggl.com/api/v8/time_entries/{}", id),
            &entry,
        )?;
        Ok(())
    }

    fn delete_entry(&self, t: &TimeEntry) -> Result<(), TogglError> {
        self.delete::<&str, DeleteEntryReturn>(&format!(
            "https://www.toggl.com/api/v8/time_entries/{}",
            t.id
        ))?;
        Ok(())
    }
}

impl TimeEntryTrait for Toggl {
    fn convert_response(&self, res: TimeEntryRangeSlice) -> Vec<TimeEntry> {
        res.iter()
            .map(|tjson| {
                TimeEntryWrapper::new(
                    self.clients.as_ref(),
                    self.projects.as_ref(),
                    &self.user.workspaces,
                    tjson,
                )
                .into()
            })
            .collect()
    }

    fn convert_single(&self, res: &TimeEntryReturn) -> Option<TimeEntry> {
        res.data.as_ref().map(|ref t| {
            TimeEntryWrapper::new(&self.clients, &self.projects, &self.user.workspaces, t).into()
        })
    }
}
