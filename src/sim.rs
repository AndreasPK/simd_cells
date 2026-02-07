use crate::engine::types::EngineState;

pub mod food;
pub mod critter;
pub mod systems;
pub mod settings;


pub fn init_sim(sim: &mut EngineState) -> () {
    let food_components = food::FoodE::components(sim, 1, 1, food::FoodType::Food1, 1);
    sim.spawn_entity(food_components);

}
