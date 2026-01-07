use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

use crate::ipc::discord::client::RichClient;
use crate::presence::activity::Activity;
use crate::presence::packet::Packet;
use crate::types::config::{SyncConfig, SyncMode};
use crate::util::pad_activity_field;

#[derive(Clone)]
enum PendingOp {
    Update(Activity),
    Clear,
}

pub struct ActivityManager {
    pub client: Arc<RwLock<RichClient>>,
    last_activity: Arc<RwLock<Option<Activity>>>,
    last_update: Arc<RwLock<Instant>>,
    last_periodic_sync: Arc<RwLock<Instant>>,
    config: Arc<RwLock<SyncConfig>>,
    pending_op: Arc<RwLock<Option<PendingOp>>>,
    first_update: Arc<AtomicBool>,
}

impl ActivityManager {
    pub fn new(client_id: u64, pipe_paths: Vec<String>) -> Self {
        let client =
            Arc::new(RwLock::new(RichClient::new(client_id, pipe_paths)));
        let last_activity = Arc::new(RwLock::new(None));
        let last_update = Arc::new(RwLock::new(Instant::now()));
        let last_periodic_sync = Arc::new(RwLock::new(Instant::now()));
        let config = Arc::new(RwLock::new(SyncConfig::default()));
        let pending_op = Arc::new(RwLock::new(None));
        let first_update = Arc::new(AtomicBool::new(true));

        let manager = Self {
            client,
            last_activity,
            last_update,
            last_periodic_sync,
            config,
            pending_op,
            first_update,
        };

        manager.start_loop();

        manager
    }

    fn start_loop(&self) {
        let client = self.client.clone();
        let last_activity = self.last_activity.clone();
        let last_update = self.last_update.clone();
        let last_periodic_sync = self.last_periodic_sync.clone();
        let config = self.config.clone();
        let pending_op = self.pending_op.clone();

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(500));

                let config_guard = config.read().unwrap();
                let enabled = config_guard.enabled;
                if !enabled {
                    continue;
                }

                let mode = config_guard.mode.clone();
                let interval = Duration::from_millis(config_guard.interval);
                let pad_enabled = config_guard.pad;
                let reset_on_update = config_guard.reset_on_update;
                drop(config_guard);

                let now = Instant::now();

                match mode {
                    SyncMode::Periodic => {
                        let last_sync = if reset_on_update {
                            last_update.read().unwrap()
                        } else {
                            last_periodic_sync.read().unwrap()
                        };

                        if now.duration_since(*last_sync) >= interval {
                            drop(last_sync);
                            let activity_opt = last_activity.read().unwrap();
                            if let Some(activity) = activity_opt.as_ref() {
                                let client_guard = client.read().unwrap();
                                let mut padded_activity = activity.clone();
                                if pad_enabled {
                                    pad_activity_field(
                                        &mut padded_activity.details,
                                    );
                                    pad_activity_field(
                                        &mut padded_activity.state,
                                    );
                                }
                                let packet = Packet::new(
                                    client_guard.pid,
                                    Some(&padded_activity),
                                );
                                let _ = client_guard.update(&packet);

                                if reset_on_update {
                                    *last_update.write().unwrap() = now;
                                } else {
                                    *last_periodic_sync.write().unwrap() = now;
                                }
                            }
                        }
                    }
                    SyncMode::Defer => {
                        let mut last_update_lock = last_update.write().unwrap();
                        if now.duration_since(*last_update_lock) >= interval {
                            let mut pending_opt = pending_op.write().unwrap();
                            if let Some(op) = pending_opt.take() {
                                let client_guard = client.read().unwrap();
                                match op {
                                    PendingOp::Update(mut activity) => {
                                        if pad_enabled {
                                            pad_activity_field(
                                                &mut activity.details,
                                            );
                                            pad_activity_field(
                                                &mut activity.state,
                                            );
                                        }
                                        let packet = Packet::new(
                                            client_guard.pid,
                                            Some(&activity),
                                        );
                                        let _ = client_guard.update(&packet);
                                    }
                                    PendingOp::Clear => {
                                        let _ = client_guard.clear();
                                    }
                                }
                                *last_update_lock = now;
                            }
                        }
                    }
                }
            }
        });
    }

    pub fn set_config(&self, new_config: SyncConfig) {
        *self.config.write().unwrap() = new_config;
    }

    pub fn update(&self, activity: Activity) -> crate::Result<()> {
        let config = self.config.read().unwrap();

        match (config.enabled, &config.mode) {
            (true, SyncMode::Defer) => {
                let mut last_update = self.last_update.write().unwrap();
                let interval = Duration::from_millis(config.interval);
                let is_first = self.first_update.swap(false, Ordering::Relaxed);

                if is_first
                    || Instant::now().duration_since(*last_update) >= interval
                {
                    let client = self.client.read().unwrap();
                    let mut padded_activity = activity.clone();
                    if config.pad {
                        pad_activity_field(&mut padded_activity.details);
                        pad_activity_field(&mut padded_activity.state);
                    }
                    let packet =
                        Packet::new(client.pid, Some(&padded_activity));
                    client.update(&packet)?;
                    *last_update = Instant::now();
                    *self.pending_op.write().unwrap() = None;
                } else {
                    *self.pending_op.write().unwrap() =
                        Some(PendingOp::Update(activity.clone()));
                }
                *self.last_activity.write().unwrap() = Some(activity);
            }
            (true, SyncMode::Periodic) => {
                let client = self.client.read().unwrap();
                let mut padded_activity = activity.clone();
                if config.pad {
                    pad_activity_field(&mut padded_activity.details);
                    pad_activity_field(&mut padded_activity.state);
                }
                let packet = Packet::new(client.pid, Some(&padded_activity));
                client.update(&packet)?;
                *self.last_update.write().unwrap() = Instant::now();
                *self.last_activity.write().unwrap() = Some(activity);
            }
            (false, _) => {
                let client = self.client.read().unwrap();
                let mut padded_activity = activity.clone();
                if config.pad {
                    pad_activity_field(&mut padded_activity.details);
                    pad_activity_field(&mut padded_activity.state);
                }
                let packet = Packet::new(client.pid, Some(&padded_activity));
                client.update(&packet)?;
            }
        }
        Ok(())
    }

    pub fn clear(&self) -> crate::Result<()> {
        let config = self.config.read().unwrap();

        match (config.enabled, &config.mode) {
            (true, SyncMode::Defer) => {
                let mut last_update = self.last_update.write().unwrap();
                let interval = Duration::from_millis(config.interval);
                let is_first = self.first_update.swap(false, Ordering::Relaxed);

                if is_first
                    || Instant::now().duration_since(*last_update) >= interval
                {
                    self.client.read().unwrap().clear()?;
                    *last_update = Instant::now();
                    *self.pending_op.write().unwrap() = None;
                } else {
                    *self.pending_op.write().unwrap() = Some(PendingOp::Clear);
                }
                *self.last_activity.write().unwrap() = None;
            }
            _ => {
                self.client.read().unwrap().clear()?;
                *self.last_activity.write().unwrap() = None;
                *self.last_update.write().unwrap() = Instant::now();
            }
        }
        Ok(())
    }
}
