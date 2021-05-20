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

    let all_treasures = find_all_treasures()?;

    todo!()
}

fn execute_plan(plan: Plan) -> Result<()> {
    info!("executing plan");

    todo!()
}

fn find_all_treasures() -> Result<Vec<String>> {
    todo!()
}
