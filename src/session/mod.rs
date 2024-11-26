use std::collections::HashMap;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::ipc::pipe::platform::client::PipeClient;
use crate::presence::types::Activity;
use crate::types::config::PluginConfig;

pub struct Session {
    #[allow(dead_code)]
    pub id: u32,
    pub workspace: Option<String>,
    pub timestamp: Option<u64>,
    pub last_activity: Option<Activity>,
    pub config: Option<PluginConfig>,
    pub pipe_client: Option<PipeClient>,
}

impl Session {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            workspace: None,
            timestamp: None,
            last_activity: None,
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

pub struct SessionRef<'a>(RwLockReadGuard<'a, HashMap<u32, Session>>);

impl std::ops::Deref for SessionRef<'_> {
    type Target = Session;

    fn deref(&self) -> &Self::Target {
        self.0.values().next().unwrap()
    }
}

pub struct SessionRefMut<'a>(RwLockWriteGuard<'a, HashMap<u32, Session>>);

impl std::ops::Deref for SessionRefMut<'_> {
    type Target = Session;

    fn deref(&self) -> &Self::Target {
        self.0.values().next().unwrap()
    }
}

impl std::ops::DerefMut for SessionRefMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.values_mut().next().unwrap()
    }
}

#[derive(Default)]
pub struct SessionManager {
    pub sessions: RwLock<HashMap<u32, Session>>,
}

impl SessionManager {
    pub fn create_session(&self, id: u32, client: PipeClient) {
        let mut sessions = self.sessions.write().unwrap();
        let mut session = Session::new(id);
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
            Some(SessionRef(sessions))
        } else {
            None
        }
    }

    pub fn get_session_mut(&self, id: u32) -> Option<SessionRefMut<'_>> {
        let sessions = self.sessions.write().unwrap();
        if sessions.contains_key(&id) {
            Some(SessionRefMut(sessions))
        } else {
            None
        }
    }
}
