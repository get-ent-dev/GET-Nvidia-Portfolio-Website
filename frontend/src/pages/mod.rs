// frontend/src/pages/mod.rs

pub mod about;
pub mod contact;
pub mod home;
pub mod not_found;
pub mod project_detail;
pub mod projects;
pub mod admin; // Make sure you create admin.rs

pub use about::About;
pub use contact::Contact;
pub use home::Home;
pub use not_found::NotFound;
pub use project_detail::ProjectDetail;
pub use projects::Projects;
pub use admin::Admin; // Re-export Admin
