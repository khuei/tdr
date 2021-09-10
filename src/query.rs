use std::{fs, process};

use anyhow::{bail, format_err, Error};
use serde::Deserialize;
use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone, Deserialize, Default)]
#[structopt(
    name = "tdr",
    version = env!("CARGO_PKG_VERSION")
)]
#[serde(default)]
pub struct Queries {
    #[structopt(skip)]
    pub workspace_slot: Option<Vec<usize>>,
    #[structopt(skip)]
    pub workspace_title: Option<Vec<String>>,
    #[structopt(skip)]
    pub workspace_num_of_item: Option<Vec<usize>>,
    #[structopt(skip)]
    pub item_slot: Option<Vec<usize>>,
    #[structopt(skip)]
    pub item_text: Option<Vec<String>>,
    #[structopt(skip)]
    pub item_expire_datetime_string: Option<Vec<String>>,
    #[structopt(skip)]
    pub item_is_finished: Option<Vec<bool>>,
}

fn get_items() -> Result<Queries, Error> {
    let query_path = dirs_next::home_dir()
        .ok_or_else(|| format_err!("could not get home directory"))?
        .join(".todo.yml");

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
        queries.workspace_slot = item_queries.workspace_slot;
        queries.workspace_title = item_queries.workspace_title;
        queries.workspace_num_of_item = item_queries.workspace_num_of_item;
        queries.item_slot = item_queries.item_slot;
        queries.item_text = item_queries.item_text;
        queries.item_expire_datetime_string = item_queries.item_expire_datetime_string;
        queries.item_is_finished = item_queries.item_is_finished;
    }

    queries
}
