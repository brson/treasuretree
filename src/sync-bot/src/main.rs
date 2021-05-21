#![allow(unused)]

use anyhow::Result;
use log::info;

use geonft_shared::data;

fn main() -> Result<()> {
    env_logger::init();

    loop {
        let plan = make_plan()?;
        execute_plan(plan)?;
    }
}

struct Plan {
    steps: Vec<Step>,
}

enum Step {
    UploadTreasureToIpfs {
    },
    UploadPlantToSolana {
    },
    UploadClaimToSolana {
    },
}

fn make_plan() -> Result<Plan> {
    info!("making new plan");

    let statuses = data::get_all_sync_statuses()?;

    todo!()
}

fn execute_plan(plan: Plan) -> Result<()> {
    info!("executing plan");

    todo!()
}
