#[derive(Deserialize, Debug)]
pub struct StartEntryReturn {
    data: StartEntryReturnInner,
}

#[derive(Deserialize, Debug)]
struct StartEntryReturnInner {
    id: i64,
    pid: i64,
    wid: i64,
    billable: bool,
    start: chrono::DateTime<chrono::Utc>,
    duration: i64,
    description: String,
}
