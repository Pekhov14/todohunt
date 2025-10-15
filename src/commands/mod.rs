pub mod init;
pub mod scan;
pub mod sync;
pub mod list;
pub mod info;

pub use init::InitCommand;
pub use scan::ScanCommand;
pub use sync::SyncCommand;
pub use list::ListCommand;
pub use info::InfoCommand;

// Common interface for all commands
pub trait Command {
    fn name(&self) -> &'static str;
    fn handle(&self) -> i32;
}
