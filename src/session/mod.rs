use std::collections::HashMap;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::presence::types::Activity;
use crate::types::config::PluginConfig;

pub struct Session {
    #[allow(dead_code)]
    pub id: u32,
    pub workspace: Option<String>,
    pub timestamp: Option<u64>,
    pub last_activity: Option<Activity>,
    pub config: Option<PluginConfig>,
}

impl Session {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            workspace: None,
            timestamp: None,
            last_activity: None,
            config: None,
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

    pub fn get_config(&self) -> Option<&PluginConfig> {
        self.config.as_ref()
    }
}

pub struct SessionRef<'a>(RwLockReadGuard<'a, HashMap<u32, Session>>);

impl<'a> std::ops::Deref for SessionRef<'a> {
    type Target = Session;

    fn deref(&self) -> &Self::Target {
        self.0.values().next().unwrap()
    }
}

pub struct SessionRefMut<'a>(RwLockWriteGuard<'a, HashMap<u32, Session>>);

impl<'a> std::ops::Deref for SessionRefMut<'a> {
    type Target = Session;

    fn deref(&self) -> &Self::Target {
        self.0.values().next().unwrap()
    }
}

impl<'a> std::ops::DerefMut for SessionRefMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.values_mut().next().unwrap()
    }
}

#[derive(Default)]
pub struct SessionManager {
    sessions: RwLock<HashMap<u32, Session>>,
    default_config: Option<PluginConfig>,
}

impl SessionManager {
    pub fn create_session(&self, id: u32) {
        let mut sessions = self.sessions.write().unwrap();
        let session = Session::new(id);
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

    pub fn set_default_config(&mut self, config: PluginConfig) {
        self.default_config = Some(config);
    }

    pub fn get_default_config(&self) -> Option<&PluginConfig> {
        self.default_config.as_ref()
    }
}
