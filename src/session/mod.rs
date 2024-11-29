#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::ipc::pipe::platform::client::PipeClient;
use crate::presence::types::Activity;
use crate::types::config::PluginConfig;

pub struct Session {
    pub workspace: Option<String>,
    pub timestamp: Option<u64>,
    pub last_activity: Option<Activity>,
    pub last_updated: u128,
    pub config: Option<PluginConfig>,
    pub pipe_client: Option<PipeClient>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            workspace: None,
            timestamp: None,
            last_activity: None,
            last_updated: 0,
            config: None,
            pipe_client: None,
        }
    }

    pub fn set_workspace(&mut self, workspace: String) {
        self.workspace = Some(workspace);
    }

    pub fn set_timestamp(&mut self, timestamp: u64) {
        self.timestamp = Some(timestamp);
    }

    pub fn set_last_activity(&mut self, activity: Activity) {
        self.last_activity = Some(activity);
    }

    pub fn set_config(&mut self, config: PluginConfig) {
        self.config = Some(config);
    }

    pub fn set_pipe_client(&mut self, client: PipeClient) {
        self.pipe_client = Some(client);
    }

    pub fn get_config(&self) -> Option<&PluginConfig> {
        self.config.as_ref()
    }

    pub fn get_pipe_client(&self) -> Option<&PipeClient> {
        self.pipe_client.as_ref()
    }

    pub fn get_pipe_client_mut(&mut self) -> Option<&mut PipeClient> {
        self.pipe_client.as_mut()
    }
}

pub struct SessionRef<'a> {
    sessions: RwLockReadGuard<'a, HashMap<u32, Session>>,
    id: u32,
}

impl std::ops::Deref for SessionRef<'_> {
    type Target = Session;

    fn deref(&self) -> &Self::Target {
        self.sessions.get(&self.id).unwrap()
    }
}

pub struct SessionRefMut<'a> {
    sessions: RwLockWriteGuard<'a, HashMap<u32, Session>>,
    id: u32,
}

impl std::ops::Deref for SessionRefMut<'_> {
    type Target = Session;

    fn deref(&self) -> &Self::Target {
        self.sessions.get(&self.id).unwrap()
    }
}

impl std::ops::DerefMut for SessionRefMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.sessions.get_mut(&self.id).unwrap()
    }
}

#[derive(Default)]
pub struct SessionManager {
    pub sessions: RwLock<HashMap<u32, Session>>,
}

impl SessionManager {
    pub fn create_session(&self, id: u32, client: PipeClient) {
        let mut sessions = self.sessions.write().unwrap();
        let mut session = Session::new();
        session.set_pipe_client(client);
        sessions.insert(id, session);
    }

    pub fn remove_session(&self, id: u32) {
        let mut sessions = self.sessions.write().unwrap();
        sessions.remove(&id);
    }

    pub fn get_session(&self, id: u32) -> Option<SessionRef<'_>> {
        let sessions = self.sessions.read().unwrap();
        if sessions.contains_key(&id) {
            Some(SessionRef { sessions, id })
        } else {
            None
        }
    }

    pub fn get_session_mut(&self, id: u32) -> Option<SessionRefMut<'_>> {
        let sessions = self.sessions.write().unwrap();
        if sessions.contains_key(&id) {
            Some(SessionRefMut { sessions, id })
        } else {
            None
        }
    }
}
