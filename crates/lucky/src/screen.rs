use config::Config;
use std::{cell::RefCell, rc::Rc};

use crate::screen_manager::Position;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Client {
    pub frame: xcb::x::Window,
    pub window: xcb::x::Window,
    pub workspace: u8,
    pub visible: bool,
}

#[derive(Default, Debug, Clone)]
pub enum WorkspaceLayout {
    #[default]
    Tall,
}

#[derive(Debug)]
pub struct Workspace {
    id: u8,
    layout: WorkspaceLayout,
    clients: Vec<xcb::x::Window>,
    focused_client: Option<xcb::x::Window>,
}

impl Workspace {
    pub fn new(id: u8) -> Self {
        Workspace {
            id,
            layout: Default::default(),
            clients: vec![],
            focused_client: None,
        }
    }

    pub fn layout(&self) -> &WorkspaceLayout {
        &self.layout
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn new_client(&mut self, client: xcb::x::Window) {
        self.clients.push(client)
    }

    pub fn clients(&self) -> &[xcb::x::Window] {
        &self.clients
    }

    pub fn clients_mut(&mut self) -> &mut [xcb::x::Window] {
        &mut self.clients
    }

    pub fn set_focused_client(&mut self, client: Option<xcb::x::Window>) {
        self.focused_client = client
    }

    pub fn remove_client(&mut self, client: xcb::x::Window) {
        self.clients.retain(|i| !i.eq(&client));
    }
}

#[derive(Debug)]
pub struct Screen {
    position: Position,
    active_workspace: u8,
    workspaces: Vec<Workspace>,
}

impl Screen {
    pub fn new(config: &Rc<RefCell<Config>>, position: Position) -> Self {
        Screen {
            position,
            active_workspace: 1,
            workspaces: (0..config.borrow().workspaces())
                .map(Workspace::new)
                .collect(),
        }
    }

    pub fn focused_client(&self) -> Option<xcb::x::Window> {
        self.workspaces[self.active_workspace as usize].focused_client
    }

    pub fn workspaces_mut(&mut self) -> &mut [Workspace] {
        &mut self.workspaces
    }

    pub fn active_workspace(&self) -> &Workspace {
        &self.workspaces[self.active_workspace as usize]
    }

    pub fn active_workspace_mut(&mut self) -> &mut Workspace {
        &mut self.workspaces[self.active_workspace as usize]
    }

    pub fn position(&self) -> &Position {
        &self.position
    }
}
