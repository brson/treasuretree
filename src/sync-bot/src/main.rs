#![allow(unused)]

use anyhow::Result;
use log::info;

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

enum TreasureStatus {
    Planted,
    Claimed,
    PlantedAndSynced,
    ClaimedAndPlantSynced,
    ClaimedAndFullySynced,
}

fn make_plan() -> Result<Plan> {
    info!("making new plan");
    panic!()
}

fn execute_plan(plan: Plan) -> Result<()> {
    info!("executing plan");
    panic!()
}
