#![allow(clippy::missing_safety_doc)]

mod ipc;
mod json;
mod mappings;
mod rpc;
mod util;

use rpc::activity::ActivityButton;
use std::{
    ffi::{c_char, CString},
    ptr::null,
    time::UNIX_EPOCH,
};
use util::types::AssetType;
use util::utils::{
    build_activity, build_presence, find_workspace, get_asset, ptr_to_string,
    validate_buttons,
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
    lsp_manager_text: String,
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

#[repr(C)]
pub struct InitArgs {
    pub client: *const c_char,
    pub image: *const c_char,
    pub editor_tooltip: *const c_char,
    pub idle_text: *const c_char,
    pub idle_tooltip: *const c_char,
    pub viewing_text: *const c_char,
    pub editing_text: *const c_char,
    pub file_browser_text: *const c_char,
    pub plugin_manager_text: *const c_char,
    pub lsp_manager_text: *const c_char,
    pub workspace_text: *const c_char,
    pub initial_path: *const c_char,
    pub swap_fields: bool,
}

#[repr(C)]
pub struct PresenceArgs {
    pub filename: *const c_char,
    pub filetype: *const c_char,
    pub cursor_position: *const c_char,
    pub problem_count: i32,
    pub is_read_only: bool,
}

#[no_mangle]
pub unsafe extern "C" fn init(
    args_ptr: *const InitArgs,
    buttons_ptr: *const Buttons,
) {
    if INITIALIZED {
        return;
    }

    let args = &*args_ptr;

    let (client_id, editor_image) = match ptr_to_string(args.client).as_str() {
        "vim" => (1219918645770059796, get_asset("editor", "vim")),
        "neovim" => (1219918880005165137, get_asset("editor", "neovim")),
        "lunarvim" => (1220295374087000104, get_asset("editor", "lunarvim")),
        "nvchad" => (1220296082861326378, get_asset("editor", "nvchad")),
        "astronvim" => (1230866983977746532, get_asset("editor", "astronvim")),
        id => (
            id.parse::<u64>().expect("Invalid client ID"),
            ptr_to_string(args.image),
        ),
    };

    let editor_tooltip = ptr_to_string(args.editor_tooltip);
    let idle_text = ptr_to_string(args.idle_text);
    let idle_tooltip = ptr_to_string(args.idle_tooltip);
    let viewing_text = ptr_to_string(args.viewing_text);
    let editing_text = ptr_to_string(args.editing_text);
    let file_browser_text = ptr_to_string(args.file_browser_text);
    let plugin_manager_text = ptr_to_string(args.plugin_manager_text);
    let lsp_manager_text = ptr_to_string(args.lsp_manager_text);
    let workspace_text = ptr_to_string(args.workspace_text);
    let swap_fields = args.swap_fields;
    let workspace = find_workspace(&ptr_to_string(args.initial_path));

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
        if let Ok(mut rich_client) = RichClient::connect(client_id) {
            rich_client
                .handshake()
                .expect("Failed to handshake with Rich Client");
            rich_client.read().expect("Failed to read from Rich Client");

            let workspace =
                workspace.file_name().unwrap().to_string_lossy().to_string();
            CONFIG = Some(Config {
                rich_client,
                editor_image,
                editor_tooltip,
                idle_text,
                idle_tooltip,
                viewing_text,
                editing_text,
                file_browser_text,
                plugin_manager_text,
                lsp_manager_text,
                workspace_text,
                workspace,
                buttons,
                swap_fields,
            });
            INITIALIZED = true;
        };
    });
}

#[no_mangle]
pub unsafe extern "C" fn update_presence(
    args_ptr: *const PresenceArgs,
) -> bool {
    if !INITIALIZED {
        return false;
    }

    CONFIG.as_mut().map_or(false, |config| {
        let args = &*args_ptr;
        let filename = ptr_to_string(args.filename);
        let filetype = ptr_to_string(args.filetype);
        let cursor_position = if !args.cursor_position.is_null() {
            Some(ptr_to_string(args.cursor_position))
        } else {
            None
        };

        let (details, large_image, large_text) = if filetype == "Cord.idle" {
            if config.idle_text.is_empty() {
                return false;
            }

            (
                config.idle_text.clone(),
                Some(get_asset("editor", "idle")),
                config.idle_tooltip.clone(),
            )
        } else {
            build_presence(
                config,
                &filename,
                &filetype,
                args.is_read_only,
                cursor_position.as_deref(),
            )
        };

        let activity = build_activity(
            config,
            &filetype,
            details,
            large_image,
            large_text,
            args.problem_count,
            START_TIME.as_ref(),
            config.swap_fields,
        );

        config
            .rich_client
            .update(&Packet::new(std::process::id(), Some(activity)))
            .is_ok()
    })
}

#[no_mangle]
pub unsafe extern "C" fn update_presence_with_assets(
    name: *const c_char,
    icon: *const c_char,
    tooltip: *const c_char,
    asset_type: i32,
    args_ptr: *const PresenceArgs,
) -> bool {
    if !INITIALIZED {
        return false;
    }

    CONFIG.as_mut().map_or(false, |config| {
        let args = &*args_ptr;
        let filename = ptr_to_string(args.filename);
        let filetype = ptr_to_string(args.filetype);
        let name = ptr_to_string(name);
        let mut icon = ptr_to_string(icon);
        let mut tooltip = ptr_to_string(tooltip);
        let cursor_position = if !args.cursor_position.is_null() {
            Some(ptr_to_string(args.cursor_position))
        } else {
            None
        };

        let (details, large_image, large_text) =
            match AssetType::from(asset_type) {
                Some(AssetType::Language) => {
                    let filename = if !filename.is_empty() {
                        &filename
                    } else if !name.is_empty() && name != "Cord.new" {
                        &name
                    } else {
                        "a new file"
                    };
                    let details = if args.is_read_only {
                        config.viewing_text.replace("{}", filename)
                    } else {
                        config.editing_text.replace("{}", filename)
                    };
                    let details = cursor_position
                        .map_or(details.clone(), |pos| {
                            format!("{details}:{pos}")
                        });

                    if icon.is_empty() || tooltip.is_empty() {
                        if let Some((default_icon, default_tooltip)) =
                            mappings::language::get(&filetype, filename)
                        {
                            if icon.is_empty() {
                                icon = get_asset("language", default_icon);
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
                    let details = config.file_browser_text.replace("{}", &name);

                    if icon.is_empty() || tooltip.is_empty() {
                        if let Some((default_icon, default_tooltip)) =
                            mappings::file_browser::get(&filetype)
                        {
                            if icon.is_empty() {
                                icon = get_asset("file_browser", default_icon);
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
                                icon =
                                    get_asset("plugin_manager", default_icon);
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
                Some(AssetType::Lsp) => {
                    let details = config.lsp_manager_text.replace("{}", &name);

                    if icon.is_empty() || tooltip.is_empty() {
                        if let Some((default_icon, default_tooltip)) =
                            mappings::lsp_manager::get(&filetype)
                        {
                            if icon.is_empty() {
                                icon = get_asset("lsp_manager", default_icon);
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
                None => return false,
            };

        let activity = build_activity(
            config,
            &filetype,
            details,
            Some(large_image),
            large_text,
            args.problem_count,
            START_TIME.as_ref(),
            config.swap_fields,
        );

        config
            .rich_client
            .update(&Packet::new(std::process::id(), Some(activity)))
            .is_ok()
    })
}

#[no_mangle]
pub unsafe extern "C" fn clear_presence() {
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

#[no_mangle]
pub unsafe extern "C" fn disconnect() {
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

#[no_mangle]
pub unsafe extern "C" fn update_time() {
    START_TIME = Some(
        std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis(),
    );
}

#[no_mangle]
pub unsafe extern "C" fn update_workspace(value: *mut c_char) -> *const c_char {
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

#[no_mangle]
pub unsafe extern "C" fn get_workspace() -> *const c_char {
    if let Some(config) = CONFIG.as_ref() {
        CString::new(config.workspace.clone()).unwrap().into_raw()
            as *const c_char
    } else {
        null()
    }
}
