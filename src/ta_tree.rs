use crate::Titta;
use std::io;

impl Titta {
    /// subcommand
    pub fn s_view_as_tree(&mut self) -> io::Result<()> {
        println!("Tree!");
        Ok(())
    }
}
