mod manager_environment;
mod resources;
mod seasons; // Ajout du module seasons

pub use manager_environment::EnvironmentManager;
pub use resources::ResourceManager;
pub use seasons::SeasonManager;

pub mod catastrophes;
pub mod climate;
pub mod needs;
