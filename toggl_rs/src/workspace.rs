#[derive(Debug, Deserialize, PartialEq, Eq, Serialize)]
/// Main Struct to store workspaces.
pub struct Workspace {
    pub id: i64,
    pub name: String,
}
