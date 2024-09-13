#![allow(clippy::too_many_arguments)]

use std::{
    ffi::{c_char, CStr},
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::{
    mappings::{get_by_filetype, Filetype},
    rpc::{
        activity::{ActivityAssets, ActivityButton},
        packet::Activity,
    },
    Config,
};

pub const GITHUB_ASSETS_URL: &str =
    "http://raw.githubusercontent.com/vyfor/cord.nvim/master/assets";
// Increment when modifying an existing icon
const ASSETS_VERSION: &str = "15";
const VCS_MARKERS: [&str; 3] = [".git", ".svn", ".hg"];

#[inline(always)]
pub fn ptr_to_string(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }

    let string: String;
    unsafe {
        let c_str = CStr::from_ptr(ptr);
        string = c_str.to_string_lossy().to_string();
    }

    string
}

#[inline(always)]
pub fn get_asset(path: &str, file: &str) -> String {
    format!("{GITHUB_ASSETS_URL}/{path}/{file}.png?v={ASSETS_VERSION}")
}

#[inline(always)]
pub fn find_workspace(initial_path: &str) -> PathBuf {
    let mut curr_dir = PathBuf::from(initial_path);

    while !curr_dir.as_os_str().is_empty() {
        for dir in VCS_MARKERS {
            let marker_path = curr_dir.join(dir);
            if marker_path.is_dir() {
                return curr_dir;
            }
        }

        curr_dir = match curr_dir.parent() {
            Some(parent) => parent.to_path_buf(),
            None => break,
        };
        if curr_dir.parent() == Some(&curr_dir) {
            break;
        }
    }

    PathBuf::from(initial_path)
}

#[inline(always)]
pub fn validate_buttons(
    first_label: String,
    mut first_url: String,
    second_label: String,
    mut second_url: String,
    workspace: &str,
) -> Vec<ActivityButton> {
    let mut buttons = Vec::with_capacity(2);

    if first_url == "git" || second_url == "git" {
        if let Some(repository) = find_git_repository(workspace) {
            if first_url == "git" {
                first_url.clone_from(&repository)
            }
            if second_url == "git" {
                second_url = repository;
            }
        }
    }

    if !first_label.is_empty()
        && !first_url.is_empty()
        && first_url.starts_with("http")
    {
        buttons.push(ActivityButton {
            label: first_label,
            url: first_url,
        });
    }

    if !second_label.is_empty()
        && !second_url.is_empty()
        && second_url.starts_with("http")
    {
        buttons.push(ActivityButton {
            label: second_label,
            url: second_url,
        });
    }

    buttons
}

#[inline(always)]
pub fn build_activity(
    config: &Config,
    filetype: &str,
    details: String,
    large_image: Option<String>,
    large_text: String,
    problem_count: i32,
    timestamp: Option<&u128>,
    swap_fields: bool,
    swap_icons: bool,
) -> Activity {
    let (state, details) = if filetype == "Cord.idle" {
        (Some(details), None)
    } else if swap_fields {
        (
            Some(details),
            get_presence_state(config, &config.workspace, problem_count),
        )
    } else {
        (
            get_presence_state(config, &config.workspace, problem_count),
            Some(details),
        )
    };

    let small_text = if !config.editor_tooltip.is_empty() {
        if config.editor_tooltip.len() < 2 {
            Some(format!("{:<2}", config.editor_tooltip))
        } else {
            Some(config.editor_tooltip.clone())
        }
    } else {
        None
    };
    let large_text = if large_text.len() < 2 {
        Some(format!("{:<2}", large_text))
    } else {
        Some(large_text)
    };

    let ((large_image, large_text), (small_image, small_text)) =
        if large_image.is_some() {
            if swap_icons {
                (
                    (Some(config.editor_image.clone()), small_text),
                    (large_image, large_text),
                )
            } else {
                (
                    (large_image, large_text),
                    (Some(config.editor_image.clone()), small_text),
                )
            }
        } else {
            (
                (Some(config.editor_image.clone()), small_text),
                (None, None),
            )
        };

    Activity {
        state,
        details,
        assets: Some(ActivityAssets {
            small_image,
            small_text,
            large_image,
            large_text,
        }),
        timestamp: timestamp.copied(),
        buttons: (!config.buttons.is_empty()).then(|| config.buttons.clone()),
    }
}

#[inline(always)]
pub fn build_presence(
    config: &Config,
    filename: &str,
    filetype: &str,
    is_read_only: bool,
    cursor_position: Option<&str>,
) -> (String, Option<String>, String) {
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
            let (details, icon, tooltip) =
                file_browser_presence(config, tooltip, icon);
            (details, Some(icon), tooltip)
        }
        Filetype::PluginManager(icon, tooltip) => {
            let (details, icon, tooltip) =
                plugin_manager_presence(config, tooltip, icon);
            (details, Some(icon), tooltip)
        }
        Filetype::Lsp(icon, tooltip) => {
            let (details, icon, tooltip) =
                lsp_manager_presence(config, tooltip, icon);
            (details, Some(icon), tooltip)
        }
        Filetype::Vcs(icon, tooltip) => {
            let (details, icon, tooltip) = vcs_presence(config, tooltip, icon);
            (details, Some(icon), tooltip)
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
                "{} - {problem_count} problems",
                config.workspace_text.replace("{}", cwd),
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
    mut filename: &str,
    filetype: &str,
    is_read_only: bool,
    cursor_position: Option<&str>,
    icon: &str,
    tooltip: &str,
) -> (String, Option<String>, String) {
    if filename.is_empty() {
        filename = "a new file";
    }
    let details = if is_read_only {
        config.viewing_text.replace("{}", filename)
    } else {
        config.editing_text.replace("{}", filename)
    };
    let presence_details = cursor_position
        .map_or(details.clone(), |pos| format!("{details}:{pos}"));
    let presence_large_image = if filetype == "Cord.new" {
        None
    } else {
        Some(get_asset("language", icon))
    };
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
    let presence_large_image = get_asset("file_browser", icon);
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
    let presence_large_image = get_asset("plugin_manager", icon);
    let presence_large_text = tooltip.to_string();

    (presence_details, presence_large_image, presence_large_text)
}

#[inline(always)]
fn lsp_manager_presence(
    config: &Config,
    tooltip: &str,
    icon: &str,
) -> (String, String, String) {
    let presence_details = config.lsp_manager_text.replace("{}", tooltip);
    let presence_large_image = get_asset("lsp_manager", icon);
    let presence_large_text = tooltip.to_string();

    (presence_details, presence_large_image, presence_large_text)
}

#[inline(always)]
fn vcs_presence(
    config: &Config,
    tooltip: &str,
    icon: &str,
) -> (String, String, String) {
    let presence_details = config.vcs_text.replace("{}", tooltip);
    let presence_large_image = get_asset("vcs", icon);
    let presence_large_text = tooltip.to_string();

    (presence_details, presence_large_image, presence_large_text)
}

#[inline(always)]
fn find_git_repository(workspace_path: &str) -> Option<String> {
    let config_path = format!("{workspace_path}/.git/config");

    let file = match File::open(config_path) {
        Ok(file) => file,
        Err(_) => return None,
    };
    let reader = BufReader::new(file);

    let mut prev_line = String::new();
    let mut remote_url = None;
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => continue,
        };

        let trimmed = prev_line.trim_start();
        if !prev_line.is_empty() && trimmed.starts_with("[remote") {
            if let Some(repo_url) = line.trim().strip_prefix("url = ") {
                let is_origin =
                    trimmed[8..].trim_start().starts_with("\"origin\"]");

                if !is_origin && remote_url.is_some() {
                    continue;
                }

                let formatted_url = if repo_url.starts_with("http") {
                    repo_url
                        .strip_suffix(".git")
                        .map(|url| url.to_string())
                        .unwrap_or_else(|| repo_url.to_string())
                } else if let Some((_protocol, repo_url)) =
                    repo_url.split_once('@')
                {
                    let repo_url = repo_url.replacen(':', "/", 1);
                    format!(
                        "https://{}",
                        repo_url.strip_suffix(".git").unwrap_or(&repo_url)
                    )
                } else {
                    continue;
                };

                if is_origin {
                    return Some(formatted_url);
                } else if remote_url.is_none() {
                    remote_url = Some(formatted_url);
                }
            }
        }

        prev_line = line;
    }

    remote_url
}
