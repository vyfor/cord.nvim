pub mod ipc;
pub mod mappings;
pub mod parser;
pub mod rpc;

use std::{
    ffi::{c_char, CStr},
    time::UNIX_EPOCH,
};

use mappings::{
    file_browser::FILE_BROWSERS, language::LANGUAGES,
    plugin_manager::PLUGIN_MANAGERS,
};
use rpc::activity::{ActivityAssets, ActivityButton};

use crate::{
    ipc::client::{Connection, RichClient},
    mappings::{file_browser, language, plugin_manager},
    rpc::packet::{Activity, Packet},
};

const GITHUB_ASSETS_URL: &str =
    "https://raw.githubusercontent.com/vyfor/cord.nvim/master/assets";

static mut RICH_CLIENT: Option<RichClient> = None;
static mut CLIENT_IMAGE: String = String::new();
static mut CWD: Option<String> = None;
static mut START_TIME: Option<u128> = None;
static mut REPOSITORY_URL: Option<String> = None;
static mut EDITOR_TOOLTIP: String = String::new();
static mut IDLE_TEXT: String = String::new();
static mut IDLE_TOOLTIP: String = String::new();
static mut VIEWING_TEXT: String = String::new();
static mut EDITING_TEXT: String = String::new();
static mut FILE_BROWSER_TEXT: String = String::new();
static mut PLUGIN_MANAGER_TEXT: String = String::new();
static mut WORKSPACE_TEXT: String = String::new();
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
        language::init();
        file_browser::init();
        plugin_manager::init();
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
                        format!("{}/editor/idle.png", GITHUB_ASSETS_URL);
                    presence_large_text = IDLE_TOOLTIP.to_string();
                }
                "netrw" | "dirvish" | "TelescopePrompt" => {
                    if FILE_BROWSER_TEXT.is_empty() {
                        return false;
                    }
                    let file_browser = FILE_BROWSERS
                        .as_ref()
                        .unwrap()
                        .get(ptr_to_string(filetype).as_str())
                        .unwrap_or(&("", ""))
                        .to_owned();
                    presence_details =
                        FILE_BROWSER_TEXT.replace("{}", file_browser.1);
                    presence_large_image = format!(
                        "{}/file_browser/{}.png",
                        GITHUB_ASSETS_URL, file_browser.0
                    );
                    presence_large_text = file_browser.1.to_string();
                }
                "lazy" | "packer" => {
                    if PLUGIN_MANAGER_TEXT.is_empty() {
                        return false;
                    }
                    let plugin_manager = PLUGIN_MANAGERS
                        .as_ref()
                        .unwrap()
                        .get(ptr_to_string(filetype).as_str())
                        .unwrap_or(&("", ""))
                        .to_owned();
                    presence_details =
                        PLUGIN_MANAGER_TEXT.replace("{}", plugin_manager.1);
                    presence_large_image = format!(
                        "{}/plugin_manager/{}.png",
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
                            &VIEWING_TEXT
                        } else {
                            &EDITING_TEXT
                        }
                        .replace("{}", "a new file");
                        presence_details = if !cursor_position.is_null() {
                            format!(
                                "{}:{}",
                                details,
                                ptr_to_string(cursor_position)
                            )
                        } else {
                            details
                        };
                        presence_large_image =
                            format!("{}/language/text.png", GITHUB_ASSETS_URL);
                        presence_large_text = "New buffer".to_string();
                    } else {
                        let file = ptr_to_string(filetype);
                        let language = LANGUAGES
                            .as_ref()
                            .unwrap()
                            .get(file.as_str())
                            .unwrap_or(&("text", &file))
                            .to_owned();
                        let details = if is_read_only {
                            &VIEWING_TEXT
                        } else {
                            &EDITING_TEXT
                        }
                        .replace("{}", &ptr_to_string(filename));
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
                            "{}/language/{}.png",
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
            if let Some(cwd) = &CWD {
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
            if let Some(repository_url) = REPOSITORY_URL.clone() {
                activity.buttons = Some(vec![ActivityButton {
                    label: "View Repository".to_string(),
                    url: repository_url,
                }]);
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
        if let Some(mut client) = RICH_CLIENT.take() {
            if let Err(e) = client.clear() {
                eprintln!("Failed to clear presence: {}", e);
            }
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
            if let Err(e) = client.close() {
                eprintln!("Failed to disconnect: {}", e);
            }
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
pub extern "C" fn set_repository_url(value: *const c_char) {
    unsafe {
        REPOSITORY_URL = Some(ptr_to_string(value));
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
