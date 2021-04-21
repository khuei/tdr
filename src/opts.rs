use std::{fs, process};

use anyhow::{bail, format_err, Error};
use serde::Deserialize;
use structopt::StructOpt;

use crate::theme::Theme;

#[derive(Debug, StructOpt, Clone, Deserialize, Default)]
#[structopt(
    name = "todo-rs",
    version = env!("CARGO_PKG_VERSION")
)]
#[serde(default)]
pub struct Opts {
    #[structopt(skip)]
    pub theme: Option<Theme>,
}

const DEFAULT_CONFIG: &str = "---
# Apply a custom theme
#
# All colors are optional. If commented out / omitted, the color will get
# sourced from your terminal color scheme
#
#theme:
#  background: '#403E41'
#  gray: '#727072'
#  profit: '#ADD977'
#  loss: '#FA648A'
#  text_normal: '#FCFCFA'
#  text_primary: '#FFDA65'
#  text_secondary: '#79DBEA'
#  border_primary: '#FC9766'
#  border_secondary: '#FCFCFA'
#  border_axis: '#FC9766'
#  highlight_focused: '#FC9766'
#  highlight_unfocused: '#727072'
";

fn get_config_opts() -> Result<Opts, Error> {
    let config_dir = dirs_next::config_dir()
        .ok_or_else(|| format_err!("Could not get config directory"))?
        .join("tickrs");

    if !config_dir.exists() {
        let _ = fs::create_dir_all(&config_dir);
    }

    let config_path = config_dir.join("config.yml");

    if !config_path.exists() {
        let _ = fs::write(&config_path, DEFAULT_CONFIG);
    }

    let config = fs::read_to_string(&config_path)?;

    let opts = match serde_yaml::from_str::<Option<Opts>>(&config) {
        Ok(Some(opts)) => opts,
        Ok(None) => bail!("Empty config file"),
        Err(e) => {
            println!(
                "Error parsing config file, make sure format is valid\n\n  {}",
                e
            );
            process::exit(1);
        }
    };

    Ok(opts)
}

fn get_cli_opts() -> Opts {
    Opts::from_args()
}

pub fn resolve_opts() -> Opts {
    let mut opts = get_cli_opts();

    if let Ok(config_opts) = get_config_opts() {
        opts.theme = config_opts.theme;
    }

    opts
}
