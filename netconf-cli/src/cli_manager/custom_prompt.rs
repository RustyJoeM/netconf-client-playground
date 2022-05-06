use std::borrow::Cow;

use reedline::{Prompt, PromptHistorySearchStatus};
use time::OffsetDateTime;

pub struct CustomPrompt {
    prompt: Cow<'static, str>,
}

const BASE_PROMPT: &str = "netconf-cli";

impl CustomPrompt {
    pub fn new() -> Self {
        Self {
            prompt: BASE_PROMPT.into(),
        }
    }

    pub fn set_sub_mode(&mut self, sub_mode: Option<String>) {
        self.prompt = match sub_mode {
            Some(s) => format!("{}({})", BASE_PROMPT, s).into(),
            None => BASE_PROMPT.into(),
        };
    }
}

impl Prompt for CustomPrompt {
    fn render_prompt_left(&self) -> std::borrow::Cow<str> {
        Cow::Borrowed(&self.prompt)
    }

    fn render_prompt_right(&self) -> std::borrow::Cow<str> {
        let time = OffsetDateTime::now_utc();
        Cow::Owned(format!("{}", time))
    }

    fn render_prompt_indicator(
        &self,
        _prompt_mode: reedline::PromptEditMode,
    ) -> std::borrow::Cow<str> {
        // TODO - customize per prompt mode?
        Cow::Borrowed("$ ")
    }

    fn render_prompt_multiline_indicator(&self) -> std::borrow::Cow<str> {
        Cow::Borrowed("| ")
    }

    fn render_prompt_history_search_indicator(
        &self,
        history_search: reedline::PromptHistorySearch,
    ) -> std::borrow::Cow<str> {
        // NOTE/TODO - copied from DefaultPrompt implementation of `reedline` crate!
        let prefix = match history_search.status {
            PromptHistorySearchStatus::Passing => "",
            PromptHistorySearchStatus::Failing => "failing ",
        };
        Cow::Owned(format!(
            "({}reverse-search: {}) ",
            prefix, history_search.term
        ))
    }
}
