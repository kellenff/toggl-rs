#[derive(Deserialize, Debug, Serialize)]
pub struct Project {
    id: String,
    name: String,
    billable: bool,
    active: bool,
}