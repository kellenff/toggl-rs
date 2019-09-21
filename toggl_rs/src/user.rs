use crate::auth::InitResponse;
use crate::workspace::Workspace;
use std::rc::Rc;

#[derive(Debug)]
pub struct User {
    pub fullname: String,
    pub workspaces: Vec<Rc<Workspace>>,
}

impl From<InitResponse> for User {
    fn from(i: InitResponse) -> User {
        User {
            fullname: i.data.fullname,
            workspaces: i.data.workspaces.into_iter().map(Rc::new).collect(),
        }
    }
}
