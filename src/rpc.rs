// <rpc message-id="101"
//     xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
//     <some-method>
//     <!-- method parameters here... -->
//     </some-method>
// </rpc>
// pub struct Rpc {
//     message_id: String,
//     xmlns: String,
// }

// use anyhow::Result;
// use rustyline::Editor;

// fn main() -> Result<()> {
//     let mut rl = Editor::<()>::new();

//     loop {
//         let line = rl.readline("> ")?;
//         println!("Line: {}", line);
//     }
// }
