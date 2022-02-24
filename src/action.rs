#[derive(Debug)]
pub enum Action {
    Add,
    Clear,
    Complete,
    Configure,
    Show,
}

// convert user text to enum
pub fn convert_to_action(string: &str) -> Option<Action> {
    match string {
        "add" => Some(Action::Add),
        "clear" => Some(Action::Clear),
        "complete" => Some(Action::Complete),
        "config" => Some(Action::Configure),
        "configure" => Some(Action::Configure),
        "finish" => Some(Action::Complete),
        "list" => Some(Action::Show),
        "show" => Some(Action::Show),
        _ => None,
    }
}
