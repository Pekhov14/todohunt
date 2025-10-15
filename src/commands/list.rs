use super::Command;

pub struct ListCommand;

impl ListCommand {
    pub(crate) fn new() -> Self {
        ListCommand
    }
}

impl Command for ListCommand {
    fn name(&self) -> &'static str {
        "list"
    }

    fn handle(&self) -> i32 {
        println!("☑️ Listing matched todos");
        showing_todo_matches();
        0
    }
    
}

fn showing_todo_matches() {
    println!("[+] Found 5 TODOs:
src/user.rs:45 — TODO: validate user data
src/api/mod.rs:22 — FIXME: refactor router")
}
