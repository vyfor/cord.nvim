use std::{
    borrow::{Borrow, Cow},
    ops::Deref,
};

use crate::{
    mappings::Filetype,
    types::Config,
    util::{types::AssetType, utils::get_asset},
};

use super::types::Activity;

#[derive(Debug, Clone)]
pub struct CustomAssetContext {
    pub ty: AssetType,
    pub name: String,
    pub icon: String,
    pub tooltip: String,
}

#[derive(Debug, Clone)]
pub struct ActivityContext {
    pub filename: String,
    pub filetype: String,
    pub is_read_only: bool,
    pub cursor_position: Option<(i32, i32)>,
    pub problem_count: i32,
    pub custom_asset: Option<CustomAssetContext>,
    pub resolved_type: Option<Filetype>,
}

impl CustomAssetContext {
    pub fn new(asset_type: AssetType, asset_name: String, icon: String, tooltip: String) -> Self {
        Self {
            ty: asset_type,
            name: asset_name,
            icon,
            tooltip,
        }
    }
}

impl ActivityContext {
    pub fn new(
        filename: String,
        filetype: String,
        is_read_only: bool,
        cursor_position: Option<(i32, i32)>,
        problem_count: i32,
    ) -> Self {
        let mut ctx = Self {
            filename,
            filetype,
            is_read_only,
            cursor_position,
            problem_count,
            resolved_type: None,
            custom_asset: None,
        };
        ctx.resolve_filetype();

        ctx
    }

    pub fn resolve_filetype(&mut self) -> bool {
        let filetype_str = self.filetype.as_str();
        let filename_str = self.filename.as_str();

        let resolved = if self.custom_asset.is_some() {
            crate::mappings::get_by_filetype_or_none(filetype_str, filename_str)
        } else {
            Some(crate::mappings::get_by_filetype(filetype_str, filename_str))
        };

        self.resolved_type = resolved;
        true
    }

    pub fn update_custom_asset(&mut self, custom: CustomAssetContext) {
        self.custom_asset = Some(custom);
    }

    pub fn get_effective_name(&self) -> Cow<str> {
        if let Some(custom) = &self.custom_asset {
            if !custom.name.is_empty() {
                return Cow::Borrowed(custom.name.as_str());
            }
        }

        if let Some(asset) = &self.custom_asset {
            match asset.ty {
                AssetType::Language => {
                    return if !self.filename.is_empty() {
                        Cow::Borrowed(self.filename.as_str())
                    } else if !asset.name.is_empty() {
                        Cow::Borrowed(asset.name.as_str())
                    } else if self.filetype == "Cord.new" {
                        Cow::Borrowed("a new file")
                    } else {
                        Cow::Owned(format!("{} file", self.filetype))
                    }
                }
                _ => {
                    if let Some(ft) = &self.resolved_type {
                        return match ft {
                            Filetype::Language(name, _)
                            | Filetype::FileBrowser(name, _)
                            | Filetype::PluginManager(name, _)
                            | Filetype::Lsp(name, _)
                            | Filetype::Vcs(name, _) => Cow::Borrowed(name),
                        };
                    }
                }
            }
        }

        Cow::Borrowed(self.filename.as_str())
    }

    pub fn get_effective_icon(&self) -> String {
        if let Some(custom) = &self.custom_asset {
            if !custom.icon.is_empty() {
                return custom.icon.clone();
            }
        }

        if let Some(ft) = &self.resolved_type {
            match ft {
                Filetype::Language(icon, _) => get_asset("language", icon),
                Filetype::FileBrowser(icon, _) => get_asset("file_browser", icon),
                Filetype::PluginManager(icon, _) => get_asset("plugin_manager", icon),
                Filetype::Lsp(icon, _) => get_asset("lsp", icon),
                Filetype::Vcs(icon, _) => get_asset("vcs", icon),
            }
        } else {
            get_asset("language", "text")
        }
    }

    pub fn get_effective_tooltip(&self) -> &str {
        if let Some(custom) = &self.custom_asset {
            if !custom.tooltip.is_empty() {
                return &custom.tooltip;
            }
        }

        if let Some(ft) = &self.resolved_type {
            match ft {
                Filetype::Language(_, tooltip)
                | Filetype::FileBrowser(_, tooltip)
                | Filetype::PluginManager(_, tooltip)
                | Filetype::Lsp(_, tooltip)
                | Filetype::Vcs(_, tooltip) => tooltip,
            }
        } else {
            &self.filetype
        }
    }

    fn build_idle_activity(&self, config: &Config) -> Activity {
        let state = self.build_workspace_state(config, -1);
        let large_image = get_asset("editor", "idle");

        Activity {
            details: Some(config.idle_text.clone()),
            state,
            large_image: Some(large_image),
            large_text: Some(config.idle_tooltip.clone()),
            small_image: None,
            small_text: None,
            timestamp: config.timestamp,
            buttons: (!config.buttons.is_empty()).then(|| config.buttons.clone()),
        }
    }

    fn build_details(&self, config: &Config) -> String {
        let filename = self.get_effective_name();
        let filename = filename.deref();

        let details = match self.resolved_type.as_ref().unwrap() {
            Filetype::Language(_, _) => {
                let mut details = if self.is_read_only {
                    config.viewing_text.replace("{}", filename)
                } else {
                    config.editing_text.replace("{}", filename)
                };

                if let Some((line, char)) = self.cursor_position {
                    details = details + ":" + &line.to_string() + ":" + &char.to_string();
                }

                details
            }
            Filetype::FileBrowser(_, _) => config
                .file_browser_text
                .replace("{}", self.get_effective_name().borrow()),
            Filetype::PluginManager(_, _) => config
                .plugin_manager_text
                .replace("{}", self.get_effective_name().borrow()),
            Filetype::Lsp(_, _) => config
                .lsp_manager_text
                .replace("{}", self.get_effective_name().borrow()),
            Filetype::Vcs(_, _) => config
                .vcs_text
                .replace("{}", self.get_effective_name().borrow()),
        };

        details
    }

    fn build_workspace_state(&self, config: &Config, problem_count: i32) -> Option<String> {
        if !config.workspace_text.is_empty() {
            Some(if problem_count != -1 {
                let replaced = config.workspace_text.replace("{}", &config.workspace);
                format!("{} - {} problems", replaced, problem_count)
            } else {
                config.workspace_text.replace("{}", &config.workspace)
            })
        } else {
            None
        }
    }

    fn build_editor_text(&self, config: &Config) -> Option<String> {
        if !config.editor_tooltip.is_empty() {
            Some(config.editor_tooltip.clone())
        } else {
            None
        }
    }

    fn swap_fields(
        &self,
        details: String,
        state: Option<String>,
        swap: bool,
    ) -> (String, Option<String>) {
        if swap {
            (state.unwrap_or_default(), Some(details))
        } else {
            (details, state)
        }
    }

    fn swap_images(
        &self,
        config: &Config,
        large_image: Option<String>,
        large_text: Option<String>,
        swap: bool,
    ) -> (
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
    ) {
        if swap {
            (
                large_image,
                large_text,
                Some(config.editor_image.clone()),
                self.build_editor_text(config),
            )
        } else {
            (
                Some(config.editor_image.clone()),
                self.build_editor_text(config),
                large_image,
                large_text,
            )
        }
    }

    pub fn build(&self, config: &Config) -> Activity {
        if self.filetype == "Cord.idle" {
            return self.build_idle_activity(config);
        }

        let details = self.build_details(config);
        let state = self.build_workspace_state(config, self.problem_count);

        let large_image = Some(self.get_effective_icon());
        let large_text = Some(self.get_effective_tooltip()).map(|s| s.to_owned());

        let (small_image, small_text, large_image, large_text) =
            self.swap_images(config, large_image, large_text, config.swap_icons);
        let (details, state) = self.swap_fields(details, state, config.swap_fields);

        Activity {
            details: Some(details),
            state: state.map(|s| s.to_owned()),
            large_image: large_image.map(|s| s.to_owned()),
            large_text,
            small_image,
            small_text,
            timestamp: config.timestamp,
            buttons: (!config.buttons.is_empty()).then(|| config.buttons.clone()),
        }
    }
}
