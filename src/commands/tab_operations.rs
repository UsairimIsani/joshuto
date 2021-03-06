use std::path;

use crate::commands::{JoshutoCommand, JoshutoRunnable, Quit, TabSwitch};
use crate::context::JoshutoContext;
use crate::error::JoshutoResult;
use crate::tab::JoshutoTab;
use crate::ui::TuiBackend;
use crate::util::load_child::LoadChild;

use crate::HOME_DIR;

#[derive(Clone, Debug)]
pub struct NewTab;

impl NewTab {
    pub fn new() -> Self {
        NewTab
    }
    pub const fn command() -> &'static str {
        "new_tab"
    }

    pub fn new_tab(context: &mut JoshutoContext) -> JoshutoResult<()> {
        /* start the new tab in $HOME or root */
        let curr_path = match HOME_DIR.as_ref() {
            Some(s) => s.clone(),
            None => path::PathBuf::from("/"),
        };

        let tab = JoshutoTab::new(curr_path, &context.config_t.sort_option)?;
        context.tabs.push(tab);
        context.curr_tab_index = context.tabs.len() - 1;
        TabSwitch::tab_switch(context.curr_tab_index, context)?;
        LoadChild::load_child(context)?;
        Ok(())
    }
}

impl JoshutoCommand for NewTab {}

impl std::fmt::Display for NewTab {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for NewTab {
    fn execute(&self, context: &mut JoshutoContext, _: &mut TuiBackend) -> JoshutoResult<()> {
        Self::new_tab(context)
    }
}

#[derive(Clone, Debug)]
pub struct CloseTab;

impl CloseTab {
    pub fn new() -> Self {
        CloseTab
    }
    pub const fn command() -> &'static str {
        "close_tab"
    }

    pub fn close_tab(context: &mut JoshutoContext) -> JoshutoResult<()> {
        if context.tabs.len() <= 1 {
            return Quit::quit(context);
        }

        let _ = context.tabs.remove(context.curr_tab_index);
        if context.curr_tab_index > 0 {
            context.curr_tab_index -= 1;
        }
        TabSwitch::tab_switch(context.curr_tab_index, context)?;
        Ok(())
    }
}

impl JoshutoCommand for CloseTab {}

impl std::fmt::Display for CloseTab {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for CloseTab {
    fn execute(&self, context: &mut JoshutoContext, _: &mut TuiBackend) -> JoshutoResult<()> {
        Self::close_tab(context)
    }
}
