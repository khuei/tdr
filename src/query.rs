use std::{fs, process};

use anyhow::{bail, format_err, Error};
use serde::Deserialize;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone, Deserialize, Default)]
#[structopt(
    name = "todo-rs",
    version = env!("CARGO_PKG_VERSION")
)]
#[serde(default)]
pub struct Queries {
    #[structopt(skip)]
    pub slot: Option<Vec<usize>>,
    #[structopt(skip)]
    pub text: Option<Vec<String>>,
    #[structopt(skip)]
    pub expire_datetime_string: Option<Vec<String>>,
}

fn get_items() -> Result<Queries, Error> {
    let config_dir = dirs_next::config_dir()
        .ok_or_else(|| format_err!("Could not get config directory"))?
        .join("todo-rs");

    if !config_dir.exists() {
        let _ = fs::create_dir_all(&config_dir);
    }

    let query_path = config_dir.join("query.yml");

    let config = fs::read_to_string(&query_path)?;

    let queries = match serde_yaml::from_str::<Option<Queries>>(&config) {
        Ok(Some(queries)) => queries,
        Ok(None) => bail!(""),
        Err(..) => process::exit(1),
    };

    Ok(queries)
}

pub fn resolve_queries() -> Queries {
    let mut queries = Queries::from_args();

    if let Ok(item_queries) = get_items() {
        queries.slot = item_queries.slot;
        queries.text = item_queries.text;
        queries.expire_datetime_string = item_queries.expire_datetime_string;
    }

    queries
}
