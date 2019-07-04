use crate::Toggl;
use crate::project::Project;
use crate::user::User;
use crate::error::TogglError;

pub type WorkspaceID = i64;

#[derive(Deserialize, Debug, Serialize)]
pub struct Workspace {
    id: WorkspaceID,
    name: String,
}

fn get_projects(u: Toggl, w: Workspace) -> Result<Vec<Project>,TogglError> {
    let url = format!("https://www.toggl.com/api/v8/workspaces/{}/projects", w.id);
    let mut resp = u
        .client
        .get(&url)
        .basic_auth(u.api_token, Some("api_token"))
        .send()?;
    Ok(resp
    .json()?)
}