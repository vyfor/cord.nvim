#![allow(dead_code)]

use crate::{
    get_field,
    msgpack::{deserialize::Deserialize, Value},
    presence::types::ActivityButton,
    remove_field,
    util::{
        logger::LogLevel,
        types::AssetType,
        utils::{find_git_repository, get_asset},
    },
};

#[derive(Debug, Clone)]
pub struct PluginConfig {
    pub log_level: LogLevel,
    pub editor: EditorConfig,
    pub display: DisplayConfig,
    pub lsp: LspConfig,
    pub idle: IdleConfig,
    pub text: TextConfig,
    pub buttons: Vec<ActivityButton>,
    pub assets: Vec<Asset>,
}

#[derive(Debug, Clone)]
pub struct EditorConfig {
    pub image: Option<String>,
    pub tooltip: String,
}

#[derive(Debug, Clone)]
pub struct DisplayConfig {
    pub show_time: bool,
    pub show_repository: bool,
    pub show_cursor_position: bool,
    pub swap_fields: bool,
    pub swap_images: bool,
    pub workspace_blacklist: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct LspConfig {
    pub show_problem_count: bool,
}

#[derive(Debug, Clone)]
pub struct IdleConfig {
    pub text: String,
    pub tooltip: String,
}

#[derive(Debug, Clone)]
pub struct TextConfig {
    pub viewing: String,
    pub editing: String,
    pub file_browser: String,
    pub plugin_manager: String,
    pub lsp_manager: String,
    pub vcs: String,
    pub workspace: String,
}

#[derive(Debug, Clone)]
pub struct Asset {
    pub name: String,
    pub icon: String,
    pub tooltip: String,
    pub ty: AssetType,
}

impl Deserialize for PluginConfig {
    fn deserialize<'a>(input: Value) -> crate::Result<Self> {
        let mut input = input.take_map().ok_or("Invalid config")?;

        let log_level = get_field!(input, "log_level", |v| v.as_uinteger())
            .try_into()
            .map_err(|_| "Invalid log level")?;
        let editor = remove_field!(input, "editor", |v| EditorConfig::deserialize(v).ok());
        let display = remove_field!(input, "display", |v| DisplayConfig::deserialize(v).ok());
        let lsp = remove_field!(input, "lsp", |v| LspConfig::deserialize(v).ok());
        let idle = remove_field!(input, "idle", |v| IdleConfig::deserialize(v).ok());
        let text = remove_field!(input, "text", |v| TextConfig::deserialize(v).ok());
        let buttons = input
            .remove("buttons")
            .and_then(|v| v.take_array())
            .map(|arr| {
                arr.into_iter()
                    .filter_map(|button| ActivityButton::deserialize(button).ok())
                    .collect()
            })
            .unwrap_or_default();
        let assets = input
            .remove("assets")
            .and_then(|v| v.take_array())
            .map(|arr| {
                arr.into_iter()
                    .filter_map(|asset| Asset::deserialize(asset).ok())
                    .collect()
            })
            .unwrap_or_default();

        Ok(PluginConfig {
            log_level,
            editor,
            display,
            lsp,
            idle,
            text,
            buttons,
            assets,
        })
    }
}

impl Deserialize for EditorConfig {
    fn deserialize(input: Value) -> crate::Result<Self> {
        let mut input = input.take_map().ok_or("Invalid editor config")?;

        let image = input
            .remove("image")
            .and_then(|v| v.take_string().take_if(|v| !v.is_empty()));
        let tooltip = remove_field!(input, "tooltip", |v| v.take_string());

        Ok(EditorConfig { image, tooltip })
    }
}

impl Deserialize for DisplayConfig {
    fn deserialize(input: Value) -> crate::Result<Self> {
        let input = input.take_map().ok_or("Invalid display config")?;

        let show_time = get_field!(input, "show_time", |v| v.as_bool());
        let show_repository = get_field!(input, "show_repository", |v| v.as_bool());
        let show_cursor_position = get_field!(input, "show_cursor_position", |v| v.as_bool());
        let swap_fields = get_field!(input, "swap_fields", |v| v.as_bool());
        let swap_images = get_field!(input, "swap_images", |v| v.as_bool());

        Ok(DisplayConfig {
            show_time,
            show_repository,
            show_cursor_position,
            swap_fields,
            swap_images,
            workspace_blacklist: Vec::new(),
        })
    }
}

impl Deserialize for LspConfig {
    fn deserialize(input: Value) -> crate::Result<Self> {
        let input = input.take_map().ok_or("Invalid lsp config")?;

        let show_problem_count = get_field!(input, "show_problem_count", |v| v.as_bool());

        Ok(LspConfig { show_problem_count })
    }
}

impl Deserialize for IdleConfig {
    fn deserialize(input: Value) -> crate::Result<Self> {
        let mut input = input.take_map().ok_or("Invalid idle config")?;

        let text = remove_field!(input, "text", |v| v.take_string());
        let tooltip = remove_field!(input, "tooltip", |v| v.take_string());

        Ok(IdleConfig { text, tooltip })
    }
}

impl Deserialize for TextConfig {
    fn deserialize(input: Value) -> crate::Result<Self> {
        let mut input = input.take_map().ok_or("Invalid text config")?;

        let viewing = remove_field!(input, "viewing", |v| v.take_string());
        let editing = remove_field!(input, "editing", |v| v.take_string());
        let file_browser = remove_field!(input, "file_browser", |v| v.take_string());
        let plugin_manager = remove_field!(input, "plugin_manager", |v| v.take_string());
        let lsp_manager = remove_field!(input, "lsp_manager", |v| v.take_string());
        let vcs = remove_field!(input, "vcs", |v| v.take_string());
        let workspace = remove_field!(input, "workspace", |v| v.take_string());

        Ok(TextConfig {
            viewing,
            editing,
            file_browser,
            plugin_manager,
            lsp_manager,
            vcs,
            workspace,
        })
    }
}

impl Deserialize for Asset {
    fn deserialize(input: Value) -> crate::Result<Self> {
        let mut input = input.take_map().ok_or("Invalid asset")?;

        let name = remove_field!(input, "name", |v| v.take_string());
        let icon = remove_field!(input, "icon", |v| v.take_string());
        let tooltip = remove_field!(input, "tooltip", |v| v.take_string());
        let ty = get_field!(input, "type", |v| v.as_uinteger())
            .try_into()
            .map_err(|_| "Invalid asset type")?;

        Ok(Asset {
            name,
            icon,
            tooltip,
            ty,
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
pub fn validate_image(image: &mut Option<String>, is_custom_client: bool) -> String {
    match (image.take(), is_custom_client) {
        (Some(img), false) => img,
        (Some(img), true) => img,
        (None, _) => get_asset("editor", "neovim"),
    }
}
