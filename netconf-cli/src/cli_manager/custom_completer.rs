use reedline::{Completer, Span, Suggestion};

#[derive(Debug)]
pub struct CustomCompleter {}

impl CustomCompleter {
    // pub fn new() -> Self {
    //     Self {}
    // }
}

impl Completer for CustomCompleter {
    fn complete(&mut self, line: &str, _pos: usize) -> Vec<reedline::Suggestion> {
        dbg!(&line, _pos);
        if line.is_empty() {
            let root_commands = vec!["hello", "lock", "unlock"];
            let vv = root_commands
                .iter()
                .map(|x| Suggestion {
                    value: x.to_string(),
                    description: None,
                    extra: None,
                    span: Span { start: 0, end: 0 },
                    append_whitespace: true,
                })
                .collect();
            dbg!(&vv);
            return vv;
        }
        vec![]
    }
}
