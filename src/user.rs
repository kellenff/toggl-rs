use crate::project::Project;
use crate::workspace::Workspace;

#[derive(Debug,Serialize,Deserialize)]
pub struct User {
    fullname: String,
    workspaces: Vec<Workspace>,
}