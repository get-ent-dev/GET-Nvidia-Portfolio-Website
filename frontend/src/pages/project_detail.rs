// frontend/src/pages/project_detail.rs

use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route; // Import Route from your main.rs

#[derive(Properties, PartialEq)]
pub struct ProjectDetailProps {
    pub project_id: u32,
}

#[function_component(ProjectDetail)]
pub fn project_detail(props: &ProjectDetailProps) -> Html {
    let project_info = match props.project_id {
        1 => ("Project Alpha", "This is the first major project, a full-stack web application demonstrating user authentication, data management, and real-time updates. Technologies used include Yew.rs for the frontend, Actix-web for the backend, and PostgreSQL for the database."),
        2 => ("Project Beta", "Project Beta focuses on a high-throughput data processing pipeline. It involves ingesting data from various sources, processing it in real-time with Rust, and storing it for analytics. Key technologies: Rust, Kafka, and a custom data analytics module."),
        3 => ("Project Gamma", "An exploration into integrating machine learning models with performant Rust services. This project showcases how to load pre-trained models and serve predictions via a RESTful API, optimized for low latency."),
        _ => ("Unknown Project", "Details for this project are not available or it does not exist."),
    };

    html! {
        <div class="card my-4 p-4 shadow-sm">
            <h1 class="text-center mb-4">{format!("Project: {}", project_info.0)}</h1>
            <h2 class="text-center mb-3">{"Overview"}</h2>
            <p class="text-justify">{project_info.1}</p>

            <hr class="my-4" />

            <div class="text-center">
                <Link<Route> to={Route::Projects} classes="btn btn-primary mt-3">{"Back to Projects"}</Link<Route>>
            </div>
        </div>
    }
}
