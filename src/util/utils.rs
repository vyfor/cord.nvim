#![allow(clippy::too_many_arguments)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::activity::types::ActivityButton;

pub const GITHUB_ASSETS_URL: &str =
    "http://raw.githubusercontent.com/vyfor/cord.nvim/master/assets";

// Increment when modifying an existing icon
const ASSETS_VERSION: &str = "16";
const VCS_MARKERS: [&str; 3] = [".git", ".svn", ".hg"];

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
pub fn find_git_repository(workspace_path: &str) -> Option<String> {
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
                let is_origin = trimmed[8..].trim_start().starts_with("\"origin\"]");

                if !is_origin && remote_url.is_some() {
                    continue;
                }

                let formatted_url = if repo_url.starts_with("http") {
                    repo_url
                        .strip_suffix(".git")
                        .map(|url| url.to_string())
                        .unwrap_or_else(|| repo_url.to_string())
                } else if let Some((_protocol, repo_url)) = repo_url.split_once('@') {
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
