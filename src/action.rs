#[derive(Debug)]
pub enum Action {
    Add,
    Clear,
    Complete,
    Show,
}

// convert user text to enum
pub fn convert_to_action(string: &str) -> Option<Action> {
    match string {
        "a" => Some(Action::Add),
        "add" => Some(Action::Add),
        "c" => Some(Action::Complete),
        "complete" => Some(Action::Complete),
        "clear" => Some(Action::Clear),
        "show" => Some(Action::Show),
        _ => None,
    }
}
