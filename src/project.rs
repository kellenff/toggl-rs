pub type ProjectID = i64;

#[derive(Deserialize, Debug, Serialize)]
pub struct Project {
    id: ProjectID,
    name: String,
    billable: bool,
    active: bool,
}