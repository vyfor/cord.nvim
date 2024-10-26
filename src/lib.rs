#![allow(clippy::missing_safety_doc)]

mod ipc;
mod json;
mod mappings;
mod rpc;
mod util;

use mappings::Filetype;
use rpc::activity::ActivityButton;
use std::{ffi::c_char, time::UNIX_EPOCH};
use util::utils::{
    build_activity, build_presence, find_workspace, get_asset, ptr_to_string,
    validate_buttons,
};
use util::{logger, types::AssetType};

use crate::{
    ipc::client::{Connection, RichClient},
    rpc::packet::Packet,
};

type LogCallback = extern "C" fn(*const c_char, i32);
type DisconnectCallback = extern "C" fn();
static mut PID: u32 = 0;
static mut INITIALIZED: bool = false;
static mut START_TIME: Option<u128> = None;
static mut CONFIG: Option<Config> = None;
static mut CALLBACKS: Option<LuaCallbacks> = None;

struct LuaCallbacks {
    log_callback: LogCallback,
    disconnect_callback: DisconnectCallback,
}

pub struct Config {
    is_custom_client: bool,
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
    vcs_text: String,
    workspace_text: String,
    workspace: String,
    workspace_blacklist: Vec<String>,
    buttons: Vec<ActivityButton>,
    swap_fields: bool,
    swap_icons: bool,
    init_buttons: InitButtons,
}

#[derive(Default)]
struct InitButtons {
    first_label: String,
    first_url: String,
    second_label: String,
    second_url: String,
}

#[repr(C)]
pub struct Buttons {
    pub first_label: *const c_char,
    pub first_url: *const c_char,
    pub second_label: *const c_char,
    pub second_url: *const c_char,
}

#[repr(C)]
pub struct Callbacks {
    pub log_callback: LogCallback,
    pub disconnect_callback: DisconnectCallback,
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
    pub vcs_text: *const c_char,
    pub workspace_text: *const c_char,
    pub workspace_blacklist: *const *const c_char,
    pub workspace_blacklist_len: i32,
    pub initial_path: *const c_char,
    pub swap_fields: bool,
    pub swap_icons: bool,
    pub log_level: u8,
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
pub unsafe extern "C" fn is_connected() -> bool {
    INITIALIZED
}

#[no_mangle]
pub unsafe extern "C" fn init(
    args_ptr: *const InitArgs,
    buttons_ptr: *const Buttons,
    callbacks_ptr: *const Callbacks,
) -> i8 {
    if INITIALIZED {
        return 0;
    }

    let args = &*args_ptr;
    let callbacks = &*callbacks_ptr;

    logger::init(args.log_level);
    if !callbacks_ptr.is_null() {
        CALLBACKS = Some(LuaCallbacks {
            log_callback: callbacks.log_callback,
            disconnect_callback: callbacks.disconnect_callback,
        });
    }

    let mut is_custom_client = false;
    let (client_id, mut editor_image) = match ptr_to_string(args.client)
        .as_str()
    {
        "vim" => (1219918645770059796, get_asset("editor", "vim")),
        "neovim" => (1219918880005165137, get_asset("editor", "neovim")),
        "lunarvim" => (1220295374087000104, get_asset("editor", "lunarvim")),
        "nvchad" => (1220296082861326378, get_asset("editor", "nvchad")),
        "astronvim" => (1230866983977746532, get_asset("editor", "astronvim")),
        id => {
            if let Ok(id) = id.parse::<u64>() {
                is_custom_client = true;
                (id, ptr_to_string(args.image))
            } else {
                error!("Invalid client ID provided");
                return -1;
            }
        }
    };

    if !is_custom_client {
        if !args.image.is_null() {
            editor_image = ptr_to_string(args.image);
        }
    } else if args.image.is_null() {
        editor_image = get_asset("editor", "neovim");
    }

    let editor_tooltip = ptr_to_string(args.editor_tooltip);
    let idle_text = ptr_to_string(args.idle_text);
    let idle_tooltip = ptr_to_string(args.idle_tooltip);
    let viewing_text = ptr_to_string(args.viewing_text);
    let editing_text = ptr_to_string(args.editing_text);
    let file_browser_text = ptr_to_string(args.file_browser_text);
    let plugin_manager_text = ptr_to_string(args.plugin_manager_text);
    let lsp_manager_text = ptr_to_string(args.lsp_manager_text);
    let vcs_text = ptr_to_string(args.vcs_text);
    let workspace_text = ptr_to_string(args.workspace_text);
    let swap_fields = args.swap_fields;
    let swap_icons = args.swap_icons;
    let workspace = find_workspace(&ptr_to_string(args.initial_path));

    let (buttons, init_buttons) = if buttons_ptr.is_null() {
        (Vec::new(), InitButtons::default())
    } else {
        let buttons = &*buttons_ptr;
        let (first_label, first_url, second_label, second_url) = (
            ptr_to_string(buttons.first_label),
            ptr_to_string(buttons.first_url),
            ptr_to_string(buttons.second_label),
            ptr_to_string(buttons.second_url),
        );

        (
            validate_buttons(
                first_label.clone(),
                first_url.clone(),
                second_label.clone(),
                second_url.clone(),
                workspace.to_str().unwrap(),
            ),
            InitButtons {
                first_label,
                first_url,
                second_label,
                second_url,
            },
        )
    };

    let workspace =
        workspace.file_name().unwrap().to_string_lossy().to_string();
    let ws = workspace.clone();

    let workspace_blacklist = if args.workspace_blacklist.is_null() {
        Vec::new()
    } else {
        let workspace_blacklist = &*args.workspace_blacklist;
        std::slice::from_raw_parts(
            workspace_blacklist,
            args.workspace_blacklist_len as usize,
        )
        .iter()
        .map(|s| ptr_to_string(s.to_owned()))
        .collect::<Vec<String>>()
    };
    let ws_blacklist = workspace_blacklist.clone();

    if let Ok(mut rich_client) = RichClient::connect(client_id) {
        debug!("Awaiting connection...");
        if rich_client.handshake().is_err() {
            error!("Failed to perform handshake with Discord");
            return -1;
        }

        if rich_client.read().is_err() {
            error!("Failed to read data from Discord");
            return -1;
        }
        info!("Connected to Discord");
        PID = std::process::id();

        CONFIG = Some(Config {
            is_custom_client,
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
            vcs_text,
            workspace_text,
            workspace,
            workspace_blacklist,
            buttons,
            swap_fields,
            swap_icons,
            init_buttons,
        });
        INITIALIZED = true;
    } else {
        error!("Failed to establish connection with Discord. Is the Discord client running?");
        return -1;
    };

    if ws_blacklist.contains(&ws) {
        warning!("Workspace '{ws}' is found in the blacklist. Hiding presence");
        return 1;
    }

    0
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
            config.swap_icons,
        );

        if config
            .rich_client
            .update(&Packet::new(PID, Some(activity)))
            .is_err()
        {
            cord_disconnect();
            error!("Failed to update rich presence: write operation failed");
            false
        } else {
            true
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn update_presence_with_assets(
    name: *const c_char,
    default_name: *const c_char,
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
        let default_name = ptr_to_string(default_name);
        let mut icon = ptr_to_string(icon);
        let mut tooltip = ptr_to_string(tooltip);
        let cursor_position = if !args.cursor_position.is_null() {
            Some(ptr_to_string(args.cursor_position))
        } else {
            None
        };

        let ft = mappings::get_by_filetype_or_none(&filetype, &filename);

        let (details, large_image, large_text) = match ft {
            Some(Filetype::Language(default_icon, default_tooltip)) => {
                let filename = if !filename.is_empty() {
                    filename.clone()
                } else if !name.is_empty() {
                    name
                } else if default_name == "Cord.new" {
                    "a new file".to_owned()
                } else {
                    format!("{} file", filetype)
                };
                let mut details = if args.is_read_only {
                    config.viewing_text.replace("{}", &filename)
                } else {
                    config.editing_text.replace("{}", &filename)
                };
                if let Some(pos) = cursor_position {
                    details = format!("{details}:{pos}");
                }

                if icon.is_empty() {
                    icon = get_asset("language", default_icon);
                }
                if tooltip.is_empty() {
                    default_tooltip.clone_into(&mut tooltip);
                }

                (details, icon, tooltip)
            }
            Some(Filetype::FileBrowser(default_icon, default_tooltip)) => {
                if icon.is_empty() {
                    icon = get_asset("file_browser", default_icon);
                }
                if tooltip.is_empty() {
                    default_tooltip.clone_into(&mut tooltip);
                }
                let name = if name.is_empty() {
                    default_tooltip
                } else {
                    &name
                };

                (config.file_browser_text.replace("{}", name), icon, tooltip)
            }
            Some(Filetype::PluginManager(default_icon, default_tooltip)) => {
                if icon.is_empty() {
                    icon = get_asset("plugin_manager", default_icon);
                }
                if tooltip.is_empty() {
                    default_tooltip.clone_into(&mut tooltip);
                }
                let name = if name.is_empty() {
                    default_tooltip
                } else {
                    &name
                };

                (
                    config.plugin_manager_text.replace("{}", name),
                    icon,
                    tooltip,
                )
            }
            Some(Filetype::Lsp(default_icon, default_tooltip)) => {
                if icon.is_empty() {
                    icon = get_asset("lsp", default_icon);
                }
                if tooltip.is_empty() {
                    default_tooltip.clone_into(&mut tooltip);
                }
                let name = if name.is_empty() {
                    default_tooltip
                } else {
                    &name
                };

                (config.lsp_manager_text.replace("{}", name), icon, tooltip)
            }
            Some(Filetype::Vcs(default_icon, default_tooltip)) => {
                if icon.is_empty() {
                    icon = get_asset("vcs", default_icon);
                }
                if tooltip.is_empty() {
                    default_tooltip.clone_into(&mut tooltip);
                }
                let name = if name.is_empty() {
                    default_tooltip
                } else {
                    &name
                };

                (config.vcs_text.replace("{}", name), icon, tooltip)
            }
            _ => match AssetType::from(asset_type) {
                Some(AssetType::Language) => {
                    if icon.is_empty() {
                        filetype.clone_into(&mut icon);
                    }

                    let filename = if !filename.is_empty() {
                        filename.clone()
                    } else if !name.is_empty() {
                        name.clone()
                    } else if default_name == "Cord.new" {
                        "a new file".to_owned()
                    } else {
                        format!("{} file", filetype)
                    };
                    let mut details = if args.is_read_only {
                        config.viewing_text.replace("{}", &filename)
                    } else {
                        config.editing_text.replace("{}", &filename)
                    };

                    if let Some(pos) = cursor_position {
                        details = format!("{details}:{pos}");
                    }

                    if tooltip.is_empty() {
                        tooltip = name;
                    }

                    (details, icon, tooltip)
                }
                Some(AssetType::FileBrowser) => {
                    if icon.is_empty() {
                        filetype.clone_into(&mut icon);
                    }
                    if tooltip.is_empty() {
                        name.clone_into(&mut tooltip);
                    }
                    let name = if name.is_empty() { &filetype } else { &name };

                    (
                        config.file_browser_text.replace("{}", name),
                        icon,
                        tooltip,
                    )
                }
                Some(AssetType::PluginManager) => {
                    if icon.is_empty() {
                        filetype.clone_into(&mut icon);
                    }
                    if tooltip.is_empty() {
                        name.clone_into(&mut tooltip);
                    }
                    let name = if name.is_empty() { &filetype } else { &name };

                    (
                        config.plugin_manager_text.replace("{}", name),
                        icon,
                        tooltip,
                    )
                }
                Some(AssetType::Lsp) => {
                    if icon.is_empty() {
                        filetype.clone_into(&mut icon);
                    }
                    if tooltip.is_empty() {
                        name.clone_into(&mut tooltip);
                    }
                    let name = if name.is_empty() { &filetype } else { &name };

                    (config.lsp_manager_text.replace("{}", name), icon, tooltip)
                }
                Some(AssetType::Vcs) => {
                    if icon.is_empty() {
                        filetype.clone_into(&mut icon);
                    }
                    if tooltip.is_empty() {
                        name.clone_into(&mut tooltip);
                    }
                    let name = if name.is_empty() { &filetype } else { &name };

                    (config.vcs_text.replace("{}", name), icon, tooltip)
                }
                _ => {
                    return false;
                }
            },
        };

        let large_image = if !(config.is_custom_client
            || large_image.starts_with("http://")
            || large_image.starts_with("https://"))
        {
            match mappings::get_by_filetype_or_none(&large_image, &filename) {
                Some(Filetype::Language(icon, _)) => {
                    get_asset("language", icon)
                }
                Some(Filetype::FileBrowser(icon, _)) => {
                    get_asset("file_browser", icon)
                }
                Some(Filetype::PluginManager(icon, _)) => {
                    get_asset("plugin_manager", icon)
                }
                Some(Filetype::Lsp(icon, _)) => get_asset("lsp", icon),
                Some(Filetype::Vcs(icon, _)) => get_asset("vcs", icon),
                _ => get_asset("language", &large_image),
            }
        } else {
            large_image.to_owned()
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
            config.swap_icons,
        );

        if config
            .rich_client
            .update(&Packet::new(PID, Some(activity)))
            .is_err()
        {
            cord_disconnect();
            error!("Failed to update rich presence: write operation failed");
            false
        } else {
            true
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn clear_presence() {
    if !INITIALIZED {
        return;
    }

    if let Some(config) = CONFIG.as_mut() {
        config.rich_client.clear().unwrap_or_else(|e| {
            error!("Failed to clear rich presence: {e}");
        });
    }
}

#[no_mangle]
pub unsafe extern "C" fn disconnect() {
    if !INITIALIZED {
        return;
    }

    if let Some(mut config) = CONFIG.take() {
        info!("Shutting down connection");
        config.rich_client.close();
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
    debug!("Set start time to {START_TIME:?}");
}

#[no_mangle]
pub unsafe extern "C" fn set_workspace(value: *mut c_char) -> bool {
    if let Some(config) = CONFIG.as_mut() {
        config.workspace = ptr_to_string(value);

        if config.workspace_blacklist.contains(&config.workspace) {
            return false;
        }

        return true;
    }

    true
}

#[no_mangle]
pub unsafe extern "C" fn update_workspace(value: *mut c_char) -> bool {
    if let Some(config) = CONFIG.as_mut() {
        let workspace = find_workspace(&ptr_to_string(value));
        if let Some(file_name) = workspace.file_name() {
            config.workspace = file_name.to_string_lossy().to_string();

            config.buttons = validate_buttons(
                config.init_buttons.first_label.clone(),
                config.init_buttons.first_url.clone(),
                config.init_buttons.second_label.clone(),
                config.init_buttons.second_url.clone(),
                &workspace.to_string_lossy(),
            );

            if config.workspace_blacklist.contains(&config.workspace) {
                return false;
            }

            return true;
        }
    }

    true
}

pub fn cord_disconnect() {
    unsafe {
        if let Some(callbacks) = CALLBACKS.as_ref() {
            (callbacks.disconnect_callback)();
        }
    }
}
