use std::borrow::Cow;

use reedline::{Prompt, PromptHistorySearchStatus};
use time::OffsetDateTime;

pub struct CustomPrompt {
    prompt: String,
}

impl CustomPrompt {
    pub fn new(prompt: String) -> Self {
        Self { prompt }
    }

    pub fn update_prompt(&mut self, prompt: String) {
        self.prompt = prompt;
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
