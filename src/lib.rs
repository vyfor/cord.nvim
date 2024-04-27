mod ipc;
mod json;
mod mappings;
mod rpc;
mod types;
mod utils;

use rpc::activity::ActivityButton;
use std::{
    ffi::{c_char, CString},
    ptr::null,
    time::UNIX_EPOCH,
};
use types::AssetType;
use utils::{
    build_activity, build_presence, find_workspace, ptr_to_string,
    validate_buttons, GITHUB_ASSETS_URL,
};

use crate::{
    ipc::client::{Connection, RichClient},
    rpc::packet::Packet,
};

static mut INITIALIZED: bool = false;
static mut START_TIME: Option<u128> = None;
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
    workspace: String,
    buttons: Vec<ActivityButton>,
    swap_fields: bool,
}

#[repr(C)]
pub struct Buttons {
    pub first_label: *const c_char,
    pub first_url: *const c_char,
    pub second_label: *const c_char,
    pub second_url: *const c_char,
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
    initial_path: *const c_char,
    buttons_ptr: *const Buttons,
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
            "astronvim" => (
                1230866983977746532,
                format!("{}/editor/astronvim.png", GITHUB_ASSETS_URL),
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
        let workspace = find_workspace(&ptr_to_string(initial_path));

        let buttons = if buttons_ptr.is_null() {
            Vec::new()
        } else {
            let buttons = &*buttons_ptr;
            validate_buttons(
                ptr_to_string(buttons.first_label),
                ptr_to_string(buttons.first_url),
                ptr_to_string(buttons.second_label),
                ptr_to_string(buttons.second_url),
                workspace.to_str().unwrap(),
            )
        };

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
                    workspace: workspace
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .to_string(),
                    buttons: buttons,
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

            let (details, large_image, large_text) = if filetype == "Cord.idle"
            {
                if config.idle_text.is_empty() {
                    return false;
                }

                (
                    config.idle_text.clone(),
                    Some(format!("{}/editor/idle.png?v=5", GITHUB_ASSETS_URL)),
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

            let activity = build_activity(
                config,
                details,
                large_image,
                large_text,
                problem_count,
                START_TIME.as_ref(),
                config.swap_fields,
            );

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
pub extern "C" fn update_presence_with_assets(
    filename: *const c_char,
    filetype: *const c_char,
    name: *const c_char,
    icon: *const c_char,
    tooltip: *const c_char,
    asset_type: i32,
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
            let name = ptr_to_string(name);
            let mut icon = ptr_to_string(icon);
            let mut tooltip = ptr_to_string(tooltip);
            let cursor_position = if !cursor_position.is_null() {
                Some(ptr_to_string(cursor_position))
            } else {
                None
            };

            let (details, large_image, large_text) =
                match AssetType::from(asset_type) {
                    Some(AssetType::Language) => {
                        let filename = if !filename.is_empty() {
                            &filename
                        } else {
                            if !name.is_empty() && name != "Cord.new" {
                                &name
                            } else {
                                "a new file"
                            }
                        };
                        let details = if is_read_only {
                            config.viewing_text.replace("{}", filename)
                        } else {
                            config.editing_text.replace("{}", filename)
                        };
                        let details = cursor_position
                            .map_or(details.clone(), |pos| {
                                format!("{}:{}", details, pos)
                            });

                        if icon.is_empty() || tooltip.is_empty() {
                            if let Some((default_icon, default_tooltip)) =
                                mappings::language::get(&filetype, filename)
                            {
                                if icon.is_empty() {
                                    icon = format!(
                                        "{}/language/{}.png?v=5",
                                        GITHUB_ASSETS_URL, default_icon
                                    );
                                }
                                if tooltip.is_empty() {
                                    tooltip = default_tooltip.to_string();
                                }
                            } else {
                                if icon.is_empty() {
                                    return false;
                                }
                                if tooltip.is_empty() {
                                    tooltip = name;
                                }
                            }
                        }

                        (details, icon, tooltip)
                    }
                    Some(AssetType::FileBrowser) => {
                        let details =
                            config.file_browser_text.replace("{}", &name);

                        if icon.is_empty() || tooltip.is_empty() {
                            if let Some((default_icon, default_tooltip)) =
                                mappings::file_browser::get(&filetype)
                            {
                                if icon.is_empty() {
                                    icon = format!(
                                        "{}/file_browser/{}.png?v=5",
                                        GITHUB_ASSETS_URL, default_icon
                                    );
                                }
                                if tooltip.is_empty() {
                                    tooltip = default_tooltip.to_string();
                                }
                            } else {
                                if icon.is_empty() {
                                    return false;
                                }
                                if tooltip.is_empty() {
                                    tooltip = name;
                                }
                            }
                        }

                        (details, icon, tooltip)
                    }
                    Some(AssetType::PluginManager) => {
                        let details =
                            config.plugin_manager_text.replace("{}", &name);

                        if icon.is_empty() || tooltip.is_empty() {
                            if let Some((default_icon, default_tooltip)) =
                                mappings::plugin_manager::get(&filetype)
                            {
                                if icon.is_empty() {
                                    icon = format!(
                                        "{}/plugin_manager/{}.png?v=5",
                                        GITHUB_ASSETS_URL, default_icon
                                    )
                                }
                                if tooltip.is_empty() {
                                    tooltip = default_tooltip.to_string()
                                }
                            } else {
                                if icon.is_empty() {
                                    return false;
                                }
                                if tooltip.is_empty() {
                                    tooltip = name;
                                }
                            }
                        }

                        (details, icon, tooltip)
                    }
                    None => return false,
                };

            let activity = build_activity(
                config,
                details,
                Some(large_image),
                large_text,
                problem_count,
                START_TIME.as_ref(),
                config.swap_fields,
            );

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

#[no_mangle]
pub extern "C" fn update_workspace(value: *mut c_char) -> *const c_char {
    unsafe {
        let mut ws = String::new();
        if let Some(config) = CONFIG.as_mut() {
            if let Some(workspace) =
                find_workspace(&ptr_to_string(value)).file_name()
            {
                let workspace = workspace.to_string_lossy().to_string();
                ws = workspace.clone();
                config.workspace = workspace;
            }
        }

        CString::new(ws).unwrap().into_raw() as *const c_char
    }
}

#[no_mangle]
pub extern "C" fn get_workspace() -> *const c_char {
    unsafe {
        if let Some(config) = CONFIG.as_ref() {
            CString::new(config.workspace.clone()).unwrap().into_raw()
                as *const c_char
        } else {
            null()
        }
    }
}
