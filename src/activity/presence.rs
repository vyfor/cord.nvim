use std::{borrow::Cow, ops::Deref};

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
pub struct PresenceContext<'a> {
    pub filename: String,
    pub filetype: String,
    pub resolved_type: Option<Filetype<'a>>,
    pub custom_asset: Option<CustomAssetContext>,
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

impl<'a> PresenceContext<'a> {
    pub fn new(filename: String, filetype: String) -> Self {
        Self {
            filename,
            filetype,
            resolved_type: None,
            custom_asset: None,
        }
    }

    pub fn resolve_filetype(&'a mut self) -> bool {
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

    pub fn get_effective_name(&'a self) -> Cow<'a, str> {
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

    pub fn get_effective_tooltip(&'a self) -> &'a str {
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
                | Filetype::Vcs(_, tooltip) => &tooltip,
            }
        } else {
            &self.filetype
        }
    }

    fn build_idle_presence(&'a self, config: &'a Config) -> Option<Activity> {
        let state = self.build_workspace_state(config, -1);
        let large_image = get_asset("editor", "idle");

        Some(Activity {
            details: Some(config.idle_text.clone()),
            state,
            large_image: Some(large_image),
            large_text: Some(config.idle_tooltip.clone()),
            small_image: None,
            small_text: None,
            timestamp: config.timestamp,
            buttons: (!config.buttons.is_empty()).then(|| config.buttons.clone()),
        })
    }

    fn build_details(
        &self,
        config: &'a Config,
        is_read_only: bool,
        cursor_position: Option<&str>,
    ) -> String {
        let filename = self.get_effective_name();
        let filename = filename.deref();

        let mut details = if is_read_only {
            config.viewing_text.replace("{}", filename)
        } else {
            config.editing_text.replace("{}", filename)
        };

        if let Some(pos) = cursor_position {
            details = format!("{}:{}", details, pos);
        }

        details
    }

    fn build_workspace_state(&self, config: &'a Config, problem_count: i32) -> Option<String> {
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
        config: &'a Config,
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

    pub fn build(
        &'a self,
        config: &'a Config,
        is_read_only: bool,
        cursor_position: Option<&str>,
        problem_count: i32,
    ) -> Option<Activity> {
        if self.filetype == "Cord.idle" {
            return self.build_idle_presence(config);
        }

        let details = self.build_details(config, is_read_only, cursor_position);
        let state = self.build_workspace_state(config, problem_count);

        let large_image = Some(self.get_effective_icon());
        let large_text = Some(self.get_effective_tooltip()).map(|s| s.to_owned());

        let (small_image, small_text, large_image, large_text) =
            self.swap_images(config, large_image, large_text, config.swap_icons);
        let (details, state) = self.swap_fields(details, state, config.swap_fields);

        Some(Activity {
            details: Some(details),
            state: state.map(|s| s.to_owned()),
            large_image: large_image.map(|s| s.to_owned()),
            large_text,
            small_image,
            small_text,
            timestamp: config.timestamp,
            buttons: (!config.buttons.is_empty()).then(|| config.buttons.clone()),
        })
    }
}