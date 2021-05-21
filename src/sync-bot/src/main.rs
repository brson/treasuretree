#![allow(unused)]

use anyhow::Result;
use log::info;
use std::collections::HashMap;

use geonft_shared::data;

fn main() -> Result<()> {
    env_logger::init();

    loop {
        let plan = make_plan()?;
        execute_plan(plan)?;
    }
}

struct Plan {
    statuses: HashMap<String, data::SyncStatus>,
    steps: Vec<(String, Step)>,
}

enum Step {
    UploadTreasureToIpfs,
    UploadPlantToSolana,
    UploadClaimToSolana,
}

fn make_plan() -> Result<Plan> {
    info!("making new plan");

    let statuses = data::get_all_sync_statuses()?;

    let treasure_events = data::get_all_plants_and_claims_time_sorted()?;

    let mut steps = Vec::new();

    for (event, treasure) in treasure_events {
        let pubkey = treasure.public_key;
        let status = statuses.get(&pubkey);

        match (event, status) {
            _ => todo!()
        }
    }

    Ok(Plan {
        statuses,
        steps
    })
}

fn execute_plan(plan: Plan) -> Result<()> {
    info!("executing plan");

    todo!()
}
