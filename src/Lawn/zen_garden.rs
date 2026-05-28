// 对应 C++ 中的 ZenGarden.h / ZenGarden.cpp

use crate::const_enums::*;

pub struct PottedPlant {
    pub seed_type: SeedType,
    pub age: PottedPlantAge,
    pub need: PottedPlantNeed,
    pub x: i32,
    pub y: i32,
    pub variation: DrawVariation,
    pub times_watered: i32,
    pub last_water_time: i64,
    pub plant_need_phase: i32,
}

impl PottedPlant {
    pub fn new(seed_type: SeedType) -> Self {
        Self {
            seed_type,
            age: PottedPlantAge::Sprout,
            need: PottedPlantNeed::None,
            x: 0,
            y: 0,
            variation: DrawVariation::Normal,
            times_watered: 0,
            last_water_time: 0,
            plant_need_phase: 0,
        }
    }

    pub fn water(&mut self) {
        self.times_watered += 1;
        self.need = PottedPlantNeed::None;
        self.age = self.age.next_age();
    }
}

pub struct ZenGarden {
    pub garden_type: GardenType,
    pub plants: Vec<PottedPlant>,
    pub active: bool,
    pub background: BackgroundType,
    pub has_bee: bool,
    pub has_snail: bool,
    pub bee_counter: i32,
}

impl ZenGarden {
    pub fn new(garden_type: GardenType) -> Self {
        Self {
            garden_type,
            plants: Vec::new(),
            active: false,
            background: BackgroundType::MushroomGarden,
            has_bee: false,
            has_snail: false,
            bee_counter: 0,
        }
    }

    pub fn update(&mut self) {
        if !self.active { return; }
        self.bee_counter += 1;
    }

    pub fn add_plant(&mut self, seed_type: SeedType) {
        self.plants.push(PottedPlant::new(seed_type));
    }

    pub fn water_plant(&mut self, index: usize) {
        if let Some(plant) = self.plants.get_mut(index) {
            plant.water();
        }
    }
}
