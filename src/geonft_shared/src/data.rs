use anyhow::Result;

pub static PLANT_DIR: &'static str = "data/plant";
pub static CLAIM_DIR: &'static str = "data/claim";

pub struct PlantedTreasure {
    pub public_key: String,
    pub time: chrono::DateTime::<chrono::Local>,
}

pub fn get_all_planted_treasures() -> Result<Vec<String>> {
    todo!()
}
