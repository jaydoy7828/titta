use crate::{Titta, constants::*};

impl Titta {
    /// subcommand: print help
    pub fn s_help(&mut self) {
        println!("{APP_NAME} v{APP_VERS}");
        println!("{APP_DESC}");
        println!("{APP_AUTH}");
        println!("---");
        println!("{HELP_BODY}");
    }
}
