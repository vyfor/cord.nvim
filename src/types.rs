use std::{collections::HashMap, sync::LazyLock};

use crate::{
    get_field,
    msgpack::{deserialize::Deserialize, Value},
    presence::types::ActivityButton,
    remove_field,
    util::{logger::LogLevel, utils::find_git_repository},
};

pub static CLIENT_IDS: LazyLock<HashMap<&str, u64>> = LazyLock::new(|| {
    HashMap::from([
        ("vim", 1219918645770059796),
        ("neovim", 1219918880005165137),
        ("lunarvim", 1220295374087000104),
        ("nvchad", 1220296082861326378),
        ("astronvim", 1230866983977746532),
    ])
});

#[derive(Debug)]
pub struct Config {
    pub log_level: LogLevel,
    pub viewing_text: String,
    pub editing_text: String,
    pub file_browser_text: String,
    pub plugin_manager_text: String,
    pub lsp_manager_text: String,
    pub vcs_text: String,
    pub workspace_text: String,
    pub workspace: String,
    pub editor_image: String,
    pub editor_tooltip: String,
    pub idle_text: String,
    pub idle_tooltip: String,
    pub workspace_blacklist: Vec<String>,
    pub swap_fields: bool,
    pub swap_icons: bool,
    pub buttons: Vec<ActivityButton>,
    pub timestamp: Option<u128>,
}

impl Deserialize for Config {
    fn deserialize<'a>(input: Value) -> crate::Result<Self> {
        let mut input = input.take_map().ok_or("Invalid config")?;

        let log_level = get_field!(input, "log_level", |v| v.as_uinteger())
            .try_into()
            .map_err(|_| "Invalid log level")?;
        let viewing_text = remove_field!(input, "viewing_text", |v| v.take_string());
        let editing_text = remove_field!(input, "editing_text", |v| v.take_string());
        let file_browser_text = remove_field!(input, "file_browser_text", |v| v.take_string());
        let plugin_manager_text = remove_field!(input, "plugin_manager_text", |v| v.take_string());
        let lsp_manager_text = remove_field!(input, "lsp_manager_text", |v| v.take_string());
        let vcs_text = remove_field!(input, "vcs_text", |v| v.take_string());
        let workspace_text = remove_field!(input, "workspace_text", |v| v.take_string());
        let workspace = remove_field!(input, "workspace", |v| v.take_string());
        let editor_image = remove_field!(input, "editor_image", |v| v.take_string());
        let editor_tooltip = remove_field!(input, "editor_tooltip", |v| v.take_string());
        let idle_text = remove_field!(input, "idle_text", |v| v.take_string());
        let idle_tooltip = remove_field!(input, "idle_tooltip", |v| v.take_string());
        let workspace_blacklist = remove_field!(input, "workspace_blacklist", |v| v.take_array())
            .into_iter()
            .map(|v| {
                v.take_string()
                    .ok_or("Invalid workspace blacklist entry")
                    .map(String::from)
            })
            .collect::<Result<Vec<String>, _>>()?;
        let swap_fields = get_field!(input, "swap_fields", |v| v.as_bool());
        let swap_icons = get_field!(input, "swap_icons", |v| v.as_bool());
        let mut buttons = input
            .remove("buttons")
            .and_then(|v| v.take_array())
            .map(|arr| {
                arr.into_iter()
                    .filter_map(|button| ActivityButton::deserialize(button).ok())
                    .collect()
            })
            .unwrap_or_default();

        validate_buttons(&mut buttons, &workspace);

        Ok(Config {
            log_level,
            viewing_text,
            editing_text,
            file_browser_text,
            plugin_manager_text,
            lsp_manager_text,
            vcs_text,
            workspace_text,
            workspace,
            editor_image,
            editor_tooltip,
            idle_text,
            idle_tooltip,
            workspace_blacklist,
            swap_fields,
            swap_icons,
            buttons,
            timestamp: None,
        })
    }
}

pub fn validate_buttons(buttons: &mut Vec<ActivityButton>, workspace: &str) {
    buttons.truncate(2);

    if buttons.iter().any(|b| b.url == "git") {
        if let Some(repository) = find_git_repository(workspace) {
            buttons
                .iter_mut()
                .filter(|b| b.url == "git")
                .for_each(|button| button.url = repository.clone());
        } else {
            buttons.retain(|b| b.url != "git");
        }
    }

    buttons.retain(|b| !b.label.is_empty() && !b.url.is_empty() && b.url.starts_with("http"));
}
