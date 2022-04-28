use netconf_client::NetconfSession;
use rustyline::{error::ReadlineError, Editor};

pub const HISTORY_FILE: &str = "netconf-cli-history.txt";

pub struct CliState {
    // TODO - make private
    pub editor: Editor<()>,
    session: Option<NetconfSession>,
    prompt: String,
}

impl CliState {
    pub fn new() -> Self {
        let mut editor = Editor::<()>::new();
        if editor.load_history(HISTORY_FILE).is_err() {
            println!("No previous history.");
        }
        let mut instance = Self {
            editor,
            session: None,
            prompt: "".to_string(),
        };
        instance.update_prompt();
        instance
    }

    pub fn set_session(&mut self, session: Option<NetconfSession>) {
        self.session = session;
        self.update_prompt();
    }

    pub fn mut_session(&mut self) -> &mut Option<NetconfSession> {
        &mut self.session
    }

    fn update_prompt(&mut self) {
        self.prompt = match &self.session {
            Some(session) => {
                let session_id = session.session_id().unwrap_or(0);
                format!(
                    "--> NETCONF(session-id:{}@{})$ ",
                    session_id,
                    session.target_string()
                )
            }
            None => "--> NETCONF$ ".to_string(),
        };
    }

    pub fn read_line(&mut self) -> Result<String, ReadlineError> {
        let r = self.editor.readline(&self.prompt)?;
        Ok(r)
    }
}

impl Drop for CliState {
    fn drop(&mut self) {
        if let Some(session) = &mut self.session {
            session.request_close_session();
        }
        self.editor.save_history(HISTORY_FILE).unwrap();
    }
}
