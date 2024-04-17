use std::ffi::{c_char, CStr};

use crate::{
    mappings::{get_by_filetype, Filetype},
    Config, GITHUB_ASSETS_URL,
};

#[inline(always)]
pub fn ptr_to_string(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    let c_str = unsafe { CStr::from_ptr(ptr) };

    c_str.to_string_lossy().into_owned()
}

#[inline(always)]
pub fn build_presence(
    config: &Config,
    filename: &str,
    filetype: &str,
    is_read_only: bool,
    cursor_position: Option<&str>,
) -> (String, String, String) {
    match get_by_filetype(filetype, filename) {
        Filetype::Language(icon, tooltip) => language_presence(
            config,
            filename,
            filetype,
            is_read_only,
            cursor_position,
            icon,
            tooltip,
        ),
        Filetype::FileBrowser(icon, tooltip) => {
            file_browser_presence(config, tooltip, icon)
        }
        Filetype::PluginManager(icon, tooltip) => {
            plugin_manager_presence(config, tooltip, icon)
        }
    }
}
#[inline(always)]
pub fn get_presence_state(
    config: &Config,
    cwd: &str,
    problem_count: i32,
) -> Option<String> {
    if !cwd.is_empty() && !config.workspace_text.is_empty() {
        Some(if problem_count != -1 {
            format!(
                "{} - {} problems",
                config.workspace_text.replace("{}", cwd),
                problem_count
            )
        } else {
            config.workspace_text.replace("{}", cwd)
        })
    } else {
        None
    }
}

#[inline(always)]
fn language_presence(
    config: &Config,
    filename: &str,
    filetype: &str,
    is_read_only: bool,
    cursor_position: Option<&str>,
    icon: &str,
    tooltip: &str,
) -> (String, String, String) {
    let details = if is_read_only {
        config.viewing_text.replace("{}", filename)
    } else {
        config.editing_text.replace("{}", filename)
    };
    let details = if filename.is_empty() && filetype.is_empty() {
        "Editing a new file".to_string()
    } else {
        details
    };
    let presence_details = cursor_position
        .map_or(details.clone(), |pos| format!("{}:{}", details, pos));
    let presence_large_image = format!(
        "{}/language/{}.png?v=2",
        GITHUB_ASSETS_URL,
        if filename.is_empty() && filetype.is_empty() {
            "text"
        } else {
            icon
        }
    );
    let presence_large_text = tooltip.to_string();

    (presence_details, presence_large_image, presence_large_text)
}

#[inline(always)]
fn file_browser_presence(
    config: &Config,
    tooltip: &str,
    icon: &str,
) -> (String, String, String) {
    let presence_details = config.file_browser_text.replace("{}", tooltip);
    let presence_large_image =
        format!("{}/file_browser/{}.png?v=2", GITHUB_ASSETS_URL, icon);
    let presence_large_text = tooltip.to_string();

    (presence_details, presence_large_image, presence_large_text)
}

#[inline(always)]
fn plugin_manager_presence(
    config: &Config,
    tooltip: &str,
    icon: &str,
) -> (String, String, String) {
    let presence_details = config.plugin_manager_text.replace("{}", tooltip);
    let presence_large_image =
        format!("{}/plugin_manager/{}.png?v=2", GITHUB_ASSETS_URL, icon);
    let presence_large_text = tooltip.to_string();

    (presence_details, presence_large_image, presence_large_text)
}
