pub mod cli;
pub mod process;
pub mod utils;

pub use cli::{Base64SubCommand, HttpSubCommand, Opts, SubCommand, TextSignFormat, TextSubCommand};
pub use process::*;
pub use utils::*;

#[allow(async_fn_in_trait)]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}
