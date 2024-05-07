pub mod cli;
pub mod process;
pub mod utils;

pub use cli::base64::*;
pub use cli::text::*;
pub use cli::*;
pub use process::*;
pub use utils::*;

pub use enum_dispatch::enum_dispatch;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}
