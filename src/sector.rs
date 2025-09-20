use serde::Deserialize;

pub type SectorID = u8;

#[derive(Deserialize, Debug)]
pub struct Sector {
    pub id : SectorID,
    pub relations : Vec<SectorID>,
    #[serde(default)]
    pub is_based : bool,
    pub cost : f64
}