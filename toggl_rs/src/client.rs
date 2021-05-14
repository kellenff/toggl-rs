use std::rc::Rc;

use crate::Query;
use crate::Toggl;

pub type Clients = Vec<Rc<Client>>;

#[derive(Deserialize, Debug, Eq, PartialEq, Serialize)]
pub struct Client {
    pub id: i64,
    pub name: String,
}

pub trait ClientTrait {
    fn fill_clients(&mut self);
}

impl ClientTrait for Toggl {
    fn fill_clients(&mut self) {
        self.clients = self
            .user
            .workspaces
            .iter()
            .flat_map(|w| {
                let url = format!("https://api.track.toggl.com/api/v8/workspaces/{}/clients", w.id);
                let res: Vec<Client> = self.get(&url).expect("Error in querying");
                res.into_iter().map(Rc::new)
            })
            .collect();
    }
}
