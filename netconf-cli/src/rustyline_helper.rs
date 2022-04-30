use rustyline::highlight::Highlighter;
use rustyline_derive::{Completer, Helper, Hinter, Validator};

#[derive(Helper, Completer, Hinter, Validator)]
pub struct RustyLineHelper {
    prompt: String,
}

impl Highlighter for RustyLineHelper {
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> std::borrow::Cow<'l, str> {
        let _ = pos;
        std::borrow::Cow::Borrowed(line)
    }

    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> std::borrow::Cow<'b, str> {
        let _ = default;
        std::borrow::Cow::Borrowed(prompt)
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> std::borrow::Cow<'h, str> {
        std::borrow::Cow::Borrowed(hint)
    }

    fn highlight_candidate<'c>(
        &self,
        candidate: &'c str, // FIXME should be Completer::Candidate
        completion: rustyline::CompletionType,
    ) -> std::borrow::Cow<'c, str> {
        let _ = completion;
        std::borrow::Cow::Borrowed(candidate)
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        let _ = (line, pos);
        false
    }
}

// impl rustyline::validate::Validator for RustyLineHelper {}

// impl rustyline::hint::Hinter for RustyLineHelper {
//     type Hint = String;

//     fn hint(&self, line: &str, pos: usize, ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
//         let _ = (line, pos, ctx);
//         None
//     }
// }

// impl rustyline::completion::Completer for RustyLineHelper {
//     type Candidate = String;

//     fn complete(
//         &self, // FIXME should be `&mut self`
//         line: &str,
//         pos: usize,
//         ctx: &rustyline::Context<'_>,
//     ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
//         let _ = (line, pos, ctx);
//         Ok((0, Vec::with_capacity(0)))
//     }

//     fn update(&self, line: &mut rustyline::line_buffer::LineBuffer, start: usize, elected: &str) {
//         let end = line.pos();
//         line.replace(start..end, elected)
//     }
// }
