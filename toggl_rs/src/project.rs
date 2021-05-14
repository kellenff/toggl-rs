use std::rc::Rc;

use crate::Query;
use crate::Toggl;

pub type Projects = Vec<Rc<Project>>;

#[derive(Deserialize, Debug, Eq, PartialEq, Serialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub billable: bool,
    pub active: bool,
    pub cid: i64,
}

pub trait ProjectTrait {
    fn fill_projects(&mut self);
}

impl ProjectTrait for Toggl {
    fn fill_projects(&mut self) {
        self.projects = self
            .user
            .workspaces
            .iter()
            .flat_map(|w| {
                let url = format!("https://www.toggl.com/api/v8/workspaces/{}/projects", w.id);
                let res: Vec<Project> = self.get(&url).expect("Error in querying");
                res.into_iter().map(Rc::new)
            })
            .collect();
    }
}
