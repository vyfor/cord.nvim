pub mod ipc;
pub mod json;
pub mod mappings;
pub mod rpc;

use std::{
    ffi::{c_char, CStr},
    time::UNIX_EPOCH,
};

use mappings::{
    file_browser::get_file_browser, language::get_language,
    plugin_manager::get_plugin_manager,
};
use rpc::activity::{ActivityAssets, ActivityButton};

use crate::{
    ipc::client::{Connection, RichClient},
    rpc::packet::{Activity, Packet},
};

const GITHUB_ASSETS_URL: &str =
    "http://raw.githubusercontent.com/vyfor/cord.nvim/master/assets";

static mut RICH_CLIENT: Option<RichClient> = None;
static mut CLIENT_IMAGE: String = String::new();
static mut CWD: Option<String> = None;
static mut START_TIME: Option<u128> = None;
static mut EDITOR_TOOLTIP: String = String::new();
static mut IDLE_TEXT: String = String::new();
static mut IDLE_TOOLTIP: String = String::new();
static mut VIEWING_TEXT: String = String::new();
static mut EDITING_TEXT: String = String::new();
static mut FILE_BROWSER_TEXT: String = String::new();
static mut PLUGIN_MANAGER_TEXT: String = String::new();
static mut WORKSPACE_TEXT: String = String::new();
static mut BUTTONS: Vec<ActivityButton> = Vec::new();
static mut INITIALIZED: bool = false;

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
) {
    unsafe {
        if INITIALIZED {
            return;
        }
        INITIALIZED = true;
        let client_id = match ptr_to_string(client).as_str() {
            "vim" => {
                CLIENT_IMAGE = format!("{}/editor/vim.png", GITHUB_ASSETS_URL);
                1219918645770059796
            }
            "neovim" => {
                CLIENT_IMAGE =
                    format!("{}/editor/neovim.png", GITHUB_ASSETS_URL);
                1219918880005165137
            }
            "lunarvim" => {
                CLIENT_IMAGE =
                    format!("{}/editor/lunarvim.png", GITHUB_ASSETS_URL);
                1220295374087000104
            }
            "nvchad" => {
                CLIENT_IMAGE =
                    format!("{}/editor/nvchad.png", GITHUB_ASSETS_URL);
                1220296082861326378
            }
            id => {
                let id = id.parse::<u64>().expect("Invalid client ID");
                CLIENT_IMAGE = ptr_to_string(image);
                id
            }
        };

        EDITOR_TOOLTIP = ptr_to_string(editor_tooltip);
        IDLE_TEXT = ptr_to_string(idle_text);
        IDLE_TOOLTIP = ptr_to_string(idle_tooltip);
        VIEWING_TEXT = ptr_to_string(viewing_text);
        EDITING_TEXT = ptr_to_string(editing_text);
        FILE_BROWSER_TEXT = ptr_to_string(file_browser_text);
        PLUGIN_MANAGER_TEXT = ptr_to_string(plugin_manager_text);
        WORKSPACE_TEXT = ptr_to_string(workspace_text);

        std::thread::spawn(move || {
            if let Ok(mut client) = RichClient::connect(client_id) {
                client
                    .handshake()
                    .expect("Failed to handshake with Rich Client");
                client.read().expect("Failed to read from Rich Client");

                RICH_CLIENT = Some(client);
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
        if let Some(client) = RICH_CLIENT.as_mut() {
            let presence_details: String;
            let presence_large_image: String;
            let presence_large_text: String;
            match ptr_to_string(filetype).as_str() {
                "Cord.idle" => {
                    if IDLE_TEXT.is_empty() {
                        return false;
                    }
                    presence_details = IDLE_TEXT.to_string();
                    presence_large_image =
                        format!("{}/editor/idle.png?v=1", GITHUB_ASSETS_URL);
                    presence_large_text = IDLE_TOOLTIP.to_string();
                }
                "netrw" | "dirvish" | "TelescopePrompt" | "neo-tree"
                | "oil" | "NvimTree" | "minifiles" => {
                    if FILE_BROWSER_TEXT.is_empty() {
                        return false;
                    }
                    let filetype = ptr_to_string(filetype);
                    let file_browser = get_file_browser(&filetype);
                    presence_details =
                        FILE_BROWSER_TEXT.replace("{}", file_browser.1);
                    presence_large_image = format!(
                        "{}/file_browser/{}.png?v=1",
                        GITHUB_ASSETS_URL, file_browser.0
                    );
                    presence_large_text = file_browser.1.to_string();
                }
                "lazy" | "packer" | "pckr" => {
                    if PLUGIN_MANAGER_TEXT.is_empty() {
                        return false;
                    }
                    let filetype = ptr_to_string(filetype);
                    let plugin_manager = get_plugin_manager(&filetype);
                    presence_details =
                        PLUGIN_MANAGER_TEXT.replace("{}", plugin_manager.1);
                    presence_large_image = format!(
                        "{}/plugin_manager/{}.png?v=1",
                        GITHUB_ASSETS_URL, plugin_manager.0
                    );
                    presence_large_text = plugin_manager.1.to_string();
                }
                _ => {
                    if ptr_to_string(filename).is_empty() {
                        if !ptr_to_string(filetype).is_empty() {
                            return false;
                        }
                        let details = if is_read_only {
                            VIEWING_TEXT.replace("{}", "a new file")
                        } else {
                            EDITING_TEXT.replace("{}", "a new file")
                        };
                        presence_details = if !cursor_position.is_null() {
                            format!(
                                "{}:{}",
                                details,
                                ptr_to_string(cursor_position)
                            )
                        } else {
                            details
                        };
                        presence_large_image = format!(
                            "{}/language/text.png?v=1",
                            GITHUB_ASSETS_URL
                        );
                        presence_large_text = "New buffer".to_string();
                    } else {
                        let filetype = ptr_to_string(filetype);
                        let filename = ptr_to_string(filename);
                        let language = get_language(&filetype, &filename);
                        let details = if is_read_only {
                            VIEWING_TEXT.replace("{}", &filename)
                        } else {
                            EDITING_TEXT.replace("{}", &filename)
                        };
                        presence_details = if !cursor_position.is_null() {
                            format!(
                                "{}:{}",
                                details,
                                ptr_to_string(cursor_position)
                            )
                        } else {
                            details
                        };
                        presence_large_image = format!(
                            "{}/language/{}.png?v=1",
                            GITHUB_ASSETS_URL, language.0
                        );
                        presence_large_text = language.1.to_string();
                    }
                }
            };
            let mut activity = Activity {
                details: Some(presence_details),
                ..Default::default()
            };
            if let Some(cwd) = CWD.as_ref() {
                if !WORKSPACE_TEXT.is_empty() {
                    activity.state = Some(if problem_count != -1 {
                        format!(
                            "{} - {} problems",
                            WORKSPACE_TEXT.replace("{}", &cwd),
                            problem_count
                        )
                    } else {
                        WORKSPACE_TEXT.replace("{}", &cwd)
                    })
                }
            }
            activity.assets = Some(ActivityAssets {
                large_image: Some(presence_large_image),
                large_text: Some(presence_large_text.to_string()),
                small_image: Some(CLIENT_IMAGE.clone()),
                small_text: if CLIENT_IMAGE.is_empty() {
                    None
                } else {
                    Some(EDITOR_TOOLTIP.clone())
                },
            });
            if let Some(presence_start_time) = START_TIME {
                activity.timestamp = Some(presence_start_time);
            }
            if !BUTTONS.is_empty() {
                activity.buttons = Some(BUTTONS.to_vec());
            }
            match client.update(&Packet {
                pid: std::process::id(),
                activity: Some(activity),
            }) {
                Ok(_) => true,
                Err(_) => false,
            }
        } else {
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn clear_presence() {
    unsafe {
        if !INITIALIZED {
            return;
        }
        if let Some(client) = RICH_CLIENT.as_mut() {
            client.clear().expect("Failed to clear presence");
        }
    }
}

#[no_mangle]
pub extern "C" fn disconnect() {
    unsafe {
        if !INITIALIZED {
            return;
        }
        if let Some(mut client) = RICH_CLIENT.take() {
            client.close().expect("Failed to close connection");
            INITIALIZED = false;
        }
    }
}

#[no_mangle]
pub extern "C" fn set_cwd(value: *const c_char) {
    unsafe {
        CWD = Some(ptr_to_string(value));
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

#[inline]
fn ptr_to_string(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    let c_str = unsafe { CStr::from_ptr(ptr) };

    c_str.to_string_lossy().into_owned()
}
