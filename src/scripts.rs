use crate::error::*;
use run_script::ScriptOptions;
use snafu::ResultExt;

#[derive(Debug, Default, Clone, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(deny_unknown_fields, default)]
pub struct Script {
    pub message: Option<String>,
    pub cmd: Option<String>,
}

impl Script {
    pub(crate) fn run(&self) -> Result<()> {
        if let Some(cmd) = &self.cmd {
            let options = ScriptOptions::new();
            let args = vec![];
            run_script::run(&cmd, &args, &options).context(ScriptError {
                script: cmd.clone(),
            })?;
        }
        Ok(())
    }
}
