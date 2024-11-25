use std::{collections::HashMap, sync::LazyLock};

use crate::{
    msgpack::{deserialize::Deserialize, Value},
    presence::types::ActivityButton,
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

        let log_level = input
            .get("log_level")
            .and_then(|v| v.as_uinteger())
            .ok_or("Missing or invalid 'log_level' field")?
            .try_into()
            .map_err(|_| "Invalid log level")?;

        let viewing_text = input
            .remove("viewing_text")
            .and_then(|v| v.take_str())
            .ok_or("Missing or invalid 'viewing_text' field")?
            .to_string();

        let editing_text = input
            .remove("editing_text")
            .and_then(|v| v.take_str())
            .ok_or("Missing or invalid 'editing_text' field")?
            .to_string();

        let file_browser_text = input
            .remove("file_browser_text")
            .and_then(|v| v.take_str())
            .ok_or("Missing or invalid 'file_browser_text' field")?
            .to_string();

        let plugin_manager_text = input
            .remove("plugin_manager_text")
            .and_then(|v| v.take_str())
            .ok_or("Missing or invalid 'plugin_manager_text' field")?
            .to_string();

        let lsp_manager_text = input
            .remove("lsp_manager_text")
            .and_then(|v| v.take_str())
            .ok_or("Missing or invalid 'lsp_manager_text' field")?
            .to_string();

        let vcs_text = input
            .remove("vcs_text")
            .and_then(|v| v.take_str())
            .ok_or("Missing or invalid 'vcs_text' field")?
            .to_string();

        let workspace_text = input
            .remove("workspace_text")
            .and_then(|v| v.take_str())
            .ok_or("Missing or invalid 'workspace_text' field")?
            .to_string();

        let workspace = input
            .remove("workspace")
            .and_then(|v| v.take_str())
            .ok_or("Missing or invalid 'workspace' field")?
            .to_string();

        let editor_image = input
            .remove("editor_image")
            .and_then(|v| v.take_str())
            .ok_or("Missing or invalid 'editor_image' field")?
            .to_string();

        let editor_tooltip = input
            .remove("editor_tooltip")
            .and_then(|v| v.take_str())
            .ok_or("Missing or invalid 'editor_tooltip' field")?
            .to_string();

        let idle_text = input
            .remove("idle_text")
            .and_then(|v| v.take_str())
            .ok_or("Missing or invalid 'idle_text' field")?
            .to_string();

        let idle_tooltip = input
            .remove("idle_tooltip")
            .and_then(|v| v.take_str())
            .ok_or("Missing or invalid 'idle_tooltip' field")?
            .to_string();

        let workspace_blacklist = input
            .remove("workspace_blacklist")
            .and_then(|v| v.take_array())
            .ok_or("Missing or invalid 'workspace_blacklist' field")?
            .into_iter()
            .map(|v| {
                v.take_str()
                    .ok_or("Invalid workspace blacklist entry")
                    .map(String::from)
            })
            .collect::<Result<Vec<String>, _>>()?;

        let swap_fields = input
            .get("swap_fields")
            .and_then(|v| v.as_bool())
            .ok_or("Missing or invalid 'swap_fields' field")?;

        let swap_icons = input
            .get("swap_icons")
            .and_then(|v| v.as_bool())
            .ok_or("Missing or invalid 'swap_icons' field")?;

        let mut buttons = input
            .remove("buttons")
            .and_then(|v| v.take_array())
            .ok_or("Missing or invalid 'buttons' field")?
            .into_iter()
            .map(ActivityButton::deserialize)
            .collect::<crate::Result<Vec<_>>>()?;
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
