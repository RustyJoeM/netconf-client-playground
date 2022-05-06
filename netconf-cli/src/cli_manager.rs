mod cli_config;

mod custom_completer;
mod custom_hinter;

mod custom_prompt;
use custom_prompt::CustomPrompt;

use netconf_client::NetconfSession;
use reedline::{FileBackedHistory, Reedline, Signal};

pub use self::cli_config::{CliConfig, DumpXmlFormat};

const HISTORY_FILE: &str = "netconf-cli-history.txt";

pub struct CliManager {
    editor: Reedline,
    prompt: CustomPrompt,
    config: CliConfig,
    pending_session: Option<NetconfSession>,
}

pub trait CliManagerCommandApi {
    fn config(&self) -> &CliConfig;
    fn config_mut(&mut self) -> &mut CliConfig;
    fn prompt_mut(&mut self) -> &mut CustomPrompt;
    fn pending_session(&self) -> &Option<NetconfSession>;
    fn pending_session_mut(&mut self) -> &mut Option<NetconfSession>;
    fn set_pending_session(&mut self, session: Option<NetconfSession>);
}

impl CliManagerCommandApi for CliManager {
    fn config(&self) -> &CliConfig {
        &self.config
    }

    fn config_mut(&mut self) -> &mut CliConfig {
        &mut self.config
    }

    fn prompt_mut(&mut self) -> &mut CustomPrompt {
        &mut self.prompt
    }

    fn pending_session(&self) -> &Option<NetconfSession> {
        &self.pending_session
    }

    fn pending_session_mut(&mut self) -> &mut Option<NetconfSession> {
        &mut self.pending_session
    }

    fn set_pending_session(&mut self, session: Option<NetconfSession>) {
        self.pending_session = session;
        let new_prompt = self.pending_session.as_ref().map(|session| {
            format!(
                "session-id:{}@{}",
                session.session_id().unwrap_or(0),
                session.target_string()
            )
        });
        self.prompt_mut().set_sub_mode(new_prompt);
    }
}

impl CliManager {
    pub fn new() -> anyhow::Result<Self> {
        let editor_history = Box::new(FileBackedHistory::with_file(5, HISTORY_FILE.into())?);

        // let commands = vec![
        //     "test".into(),
        //     "hello world".into(),
        //     "hello world reedline".into(),
        //     "this is the reedline crate".into(),
        // ];
        // let completer = Box::new(DefaultCompleter::new_with_wordlen(commands.clone(), 2));
        // Use the interactive menu to select options from the completer
        // let completion_menu = Box::new(CompletionMenu::default());

        // let completer = CustomCompleter::new();
        // dbg!(&completer);
        // let completion_menu = Box::new(CompletionMenu::default());

        // let hinter = CustomHinter {};

        let editor = Reedline::create()
            .with_history(editor_history)
            // .with_completer(completer)
            // .with_hinter(Box::new(hinter));
        // .with_menu(completion_menu);
        ;
        let prompt = CustomPrompt::new();

        let config = CliConfig::new();

        Ok(Self {
            editor,
            prompt,
            config,
            pending_session: None,
        })
    }

    pub fn get_user_input(&mut self) -> anyhow::Result<Signal> {
        let r = self.editor.read_line(&self.prompt)?;
        Ok(r)
    }
}

impl Drop for CliManager {
    fn drop(&mut self) {
        let _ = self.editor.sync_history();

        if let Some(session) = &mut self.pending_session {
            let _ = session.request_close_session();
        }
    }
}
