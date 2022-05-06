use reedline::Hinter;

pub struct CustomHinter {}

impl Hinter for CustomHinter {
    fn handle(
        &mut self,
        line: &str,
        pos: usize,
        history: &dyn reedline::History,
        use_ansi_coloring: bool,
    ) -> String {
        dbg!(line, pos, use_ansi_coloring);
        let x = history.string_at_cursor();
        dbg!(&x);
        "".to_string()
    }

    fn complete_hint(&self) -> String {
        "complete".to_string()
    }

    fn next_hint_token(&self) -> String {
        "next".to_string()
    }
}
