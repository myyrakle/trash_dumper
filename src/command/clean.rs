use serde::Deserialize;

use clap::Args;

#[derive(Clone, Debug, Default, Deserialize, Args)]
pub struct ConfigOptions {
    #[clap(short, long, default_value = "false", help = "trash file path")]
    pub path: Option<String>,
}

#[derive(Clone, Debug, Args)]
#[clap(name = "clean", about = "Remove all trash files")]
pub struct Command {
    #[clap(flatten)]
    pub value: ConfigOptions,
}
