pub mod handlers;
pub mod models;
#[cfg(feature = "server")]
pub mod repository;

pub use models::{CreateResidenceCard, ResidenceCard, UpdateResidenceCard};
#[cfg(feature = "server")]
pub use repository::ResidenceCardRepository;
