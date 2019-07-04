use std::rc::Rc;

use crate::workspace::Workspace;

#[derive(Debug)]
pub struct User {
    pub fullname: String,
    pub workspaces: Vec<Rc<Workspace>>,
}
