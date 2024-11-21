use std::{collections::HashMap, sync::LazyLock};

use crate::{
    json::deserialize::{DValue, Deserialize},
    presence::types::ActivityButton,
    util::utils::find_git_repository,
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

#[derive(Debug, Clone)]
pub struct Config {
    pub log_level: u8,
    pub timestamp: Option<u128>,
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
}

impl Deserialize for Config {
    fn deserialize<'a>(input: &HashMap<&'a str, DValue<'a>>) -> Result<Self, String> {
        let log_level = input
            .get("log_level")
            .and_then(|v| v.as_number())
            .ok_or("Missing or invalid 'log_level' field")? as u8;

        let viewing_text = input
            .get("viewing_text")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'viewing_text' field")?
            .to_string();

        let editing_text = input
            .get("editing_text")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'editing_text' field")?
            .to_string();

        let file_browser_text = input
            .get("file_browser_text")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'file_browser_text' field")?
            .to_string();

        let plugin_manager_text = input
            .get("plugin_manager_text")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'plugin_manager_text' field")?
            .to_string();

        let lsp_manager_text = input
            .get("lsp_manager_text")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'lsp_manager_text' field")?
            .to_string();

        let vcs_text = input
            .get("vcs_text")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'vcs_text' field")?
            .to_string();

        let workspace_text = input
            .get("workspace_text")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'workspace_text' field")?
            .to_string();

        let workspace = input
            .get("workspace")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'workspace' field")?
            .to_string();

        let editor_image = input
            .get("editor_image")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'editor_image' field")?
            .to_string();

        let editor_tooltip = input
            .get("editor_tooltip")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'editor_tooltip' field")?
            .to_string();

        let idle_text = input
            .get("idle_text")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'idle_text' field")?
            .to_string();

        let idle_tooltip = input
            .get("idle_tooltip")
            .and_then(|v| v.as_str())
            .ok_or("Missing or invalid 'idle_tooltip' field")?
            .to_string();

        let workspace_blacklist = input
            .get("workspace_blacklist")
            .and_then(|v| v.as_array())
            .ok_or("Missing or invalid 'workspace_blacklist' field")?
            .iter()
            .map(|v| {
                v.as_str()
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
            .get("buttons")
            .and_then(|v| v.as_array())
            .ok_or("Missing or invalid 'buttons' field")?
            .iter()
            .map(|v| {
                v.as_map()
                    .ok_or("Invalid button entry".to_string())
                    .and_then(ActivityButton::deserialize)
            })
            .collect::<Result<Vec<_>, _>>()?;
        buttons = validate_buttons(buttons, &workspace);

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

pub fn validate_buttons(mut buttons: Vec<ActivityButton>, workspace: &str) -> Vec<ActivityButton> {
    if buttons.iter().any(|b| b.url == "git") {
        if let Some(repository) = find_git_repository(workspace) {
            for button in &mut buttons {
                if button.url == "git" {
                    button.url.clone_from(&repository);
                }
            }
        } else {
            buttons.retain(|b| b.url != "git");
        }
    }

    buttons.retain(|b| !b.label.is_empty() && !b.url.is_empty() && b.url.starts_with("http"));
    buttons.truncate(2);
    buttons
}
