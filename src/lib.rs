mod ipc;
mod json;
mod mappings;
mod rpc;
mod utils;

use rpc::activity::{ActivityAssets, ActivityButton};
use std::{ffi::c_char, time::UNIX_EPOCH};
use utils::{build_presence, get_presence_state, ptr_to_string};

use crate::{
    ipc::client::{Connection, RichClient},
    rpc::packet::{Activity, Packet},
};

const GITHUB_ASSETS_URL: &str =
    "http://raw.githubusercontent.com/vyfor/cord.nvim/master/assets";

static mut INITIALIZED: bool = false;
static mut CWD: String = String::new();
static mut START_TIME: Option<u128> = None;
static mut BUTTONS: Vec<ActivityButton> = Vec::new();
static mut CONFIG: Option<Config> = None;

struct Config {
    rich_client: RichClient,
    editor_image: String,
    editor_tooltip: String,
    idle_text: String,
    idle_tooltip: String,
    viewing_text: String,
    editing_text: String,
    file_browser_text: String,
    plugin_manager_text: String,
    workspace_text: String,
    swap_fields: bool,
}

#[no_mangle]
pub extern "C" fn init(
    client: *const c_char,
    image: *const c_char,
    editor_tooltip: *const c_char,
    idle_text: *const c_char,
    idle_tooltip: *const c_char,
    viewing_text: *const c_char,
    editing_text: *const c_char,
    file_browser_text: *const c_char,
    plugin_manager_text: *const c_char,
    workspace_text: *const c_char,
    swap_fields: bool,
) {
    unsafe {
        if INITIALIZED {
            return;
        }

        let (client_id, client_image) = match ptr_to_string(client).as_str() {
            "vim" => (
                1219918645770059796,
                format!("{}/editor/vim.png", GITHUB_ASSETS_URL),
            ),
            "neovim" => (
                1219918880005165137,
                format!("{}/editor/neovim.png", GITHUB_ASSETS_URL),
            ),
            "lunarvim" => (
                1220295374087000104,
                format!("{}/editor/lunarvim.png", GITHUB_ASSETS_URL),
            ),
            "nvchad" => (
                1220296082861326378,
                format!("{}/editor/nvchad.png", GITHUB_ASSETS_URL),
            ),
            id => (
                id.parse::<u64>().expect("Invalid client ID"),
                ptr_to_string(image),
            ),
        };

        let editor_tooltip = ptr_to_string(editor_tooltip);
        let idle_text = ptr_to_string(idle_text);
        let idle_tooltip = ptr_to_string(idle_tooltip);
        let viewing_text = ptr_to_string(viewing_text);
        let editing_text = ptr_to_string(editing_text);
        let file_browser_text = ptr_to_string(file_browser_text);
        let plugin_manager_text = ptr_to_string(plugin_manager_text);
        let workspace_text = ptr_to_string(workspace_text);

        std::thread::spawn(move || {
            if let Ok(mut client) = RichClient::connect(client_id) {
                client
                    .handshake()
                    .expect("Failed to handshake with Rich Client");
                client.read().expect("Failed to read from Rich Client");

                CONFIG = Some(Config {
                    rich_client: client,
                    editor_image: client_image,
                    editor_tooltip: editor_tooltip,
                    idle_text: idle_text,
                    idle_tooltip: idle_tooltip,
                    viewing_text: viewing_text,
                    editing_text: editing_text,
                    file_browser_text: file_browser_text,
                    plugin_manager_text: plugin_manager_text,
                    workspace_text: workspace_text,
                    swap_fields: swap_fields,
                });
                INITIALIZED = true;
            };
        });
    }
}

#[no_mangle]
pub extern "C" fn update_presence(
    filename: *const c_char,
    filetype: *const c_char,
    is_read_only: bool,
    cursor_position: *const c_char,
    problem_count: i32,
) -> bool {
    unsafe {
        if !INITIALIZED {
            return false;
        }

        CONFIG.as_mut().map_or(false, |config| {
            let filename = ptr_to_string(filename);
            let filetype = ptr_to_string(filetype);
            let cursor_position = if !cursor_position.is_null() {
                Some(ptr_to_string(cursor_position))
            } else {
                None
            };

            let (presence_details, presence_large_image, presence_large_text) =
                if filetype.as_str() == "Cord.idle" {
                    if config.idle_text.is_empty() {
                        return false;
                    }

                    (
                        config.idle_text.clone(),
                        format!("{}/editor/idle.png?v=1", GITHUB_ASSETS_URL),
                        config.idle_tooltip.clone(),
                    )
                } else {
                    build_presence(
                        &config,
                        &filename,
                        &filetype,
                        is_read_only,
                        cursor_position.as_deref(),
                    )
                };

            let mut activity = Activity::default();

            if config.swap_fields {
                activity.state = Some(presence_details);
                activity.details =
                    get_presence_state(&config, CWD.as_ref(), problem_count);
            } else {
                activity.state =
                    get_presence_state(&config, CWD.as_ref(), problem_count);
                activity.details = Some(presence_details);
            }
            activity.assets = Some(ActivityAssets {
                large_image: Some(presence_large_image),
                large_text: Some(if presence_large_text.len() < 2 {
                    format!("{:<2}", presence_large_text)
                } else {
                    presence_large_text
                }),
                small_image: Some(config.editor_image.clone()),
                small_text: if config.editor_tooltip.is_empty() {
                    None
                } else {
                    Some(config.editor_tooltip.clone())
                },
            });
            START_TIME.as_ref().map(|start_time| {
                activity.timestamp = Some(start_time.clone());
            });
            if !BUTTONS.is_empty() {
                activity.buttons = Some(BUTTONS.clone());
            }

            match config
                .rich_client
                .update(&Packet::new(std::process::id(), Some(activity)))
            {
                Ok(_) => true,
                Err(_) => false,
            }
        })
    }
}

#[no_mangle]
pub extern "C" fn clear_presence() {
    unsafe {
        if !INITIALIZED {
            return;
        }

        if let Some(config) = CONFIG.as_mut() {
            config
                .rich_client
                .clear()
                .expect("Failed to clear presence");
        }
    }
}

#[no_mangle]
pub extern "C" fn disconnect() {
    unsafe {
        if !INITIALIZED {
            return;
        }

        if let Some(mut config) = CONFIG.take() {
            config
                .rich_client
                .close()
                .expect("Failed to close connection");
            INITIALIZED = false;
        }
    }
}

#[no_mangle]
pub extern "C" fn set_cwd(value: *const c_char) {
    unsafe {
        CWD = ptr_to_string(value);
    }
}

#[no_mangle]
pub extern "C" fn set_buttons(
    first_label: *const c_char,
    first_url: *const c_char,
    second_label: *const c_char,
    second_url: *const c_char,
) {
    unsafe {
        BUTTONS.clear();
        let first_label = ptr_to_string(first_label);
        let first_url = ptr_to_string(first_url);
        if !first_label.is_empty() && !first_url.is_empty() {
            BUTTONS.push(ActivityButton {
                label: first_label,
                url: first_url,
            });
        }
        let second_label = ptr_to_string(second_label);
        let second_url = ptr_to_string(second_url);
        if !second_label.is_empty() && !second_url.is_empty() {
            BUTTONS.push(ActivityButton {
                label: second_label,
                url: second_url,
            })
        }
    }
}

#[no_mangle]
pub extern "C" fn update_time() {
    unsafe {
        START_TIME = Some(
            std::time::SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
        );
    }
}
