mod console;
mod fs;
mod mcp;
mod prompt_parser;
mod router;
mod think;

pub use console::Prompt;
pub use router::*;

#[async_trait::async_trait]
pub(crate) trait ToolTrait {
    type Input;
    type Output;

    fn description(&self) -> String;
    async fn call(&self, input: Self::Input) -> Result<Self::Output, String>;
}

trait Documented {
    fn doc() -> &'static str;
}
