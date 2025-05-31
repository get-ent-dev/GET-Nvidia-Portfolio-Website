// frontend/src/pages/ProjectsPage.rs
use yew::prelude::*;
use stylist::{style, css};
use serde::{Deserialize, Serialize};
use reqwest::header::ACCEPT;
use log::error;
use std::rc::Rc;
use std::cell::RefCell;

use crate::api::API_BASE_URL;
use crate::components::ProjectCard; // Assuming you have this component
use crate::webgl; // Assume webgl module exists

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Project {
    pub id: String,
    pub title: String,
    pub description: String,
    pub image_url: Option<String>,
    pub project_url: Option<String>,
    pub tags: Option<String>,
    // created_at and updated_at are not typically needed on the frontend for display
}

#[derive(Properties, PartialEq)]
pub struct ProjectsPageProps {
    pub renderer: UseStateHandle<Option<Rc<RefCell<webgl::Renderer>>>>, // Pass the renderer handle
}

#[function_component(ProjectsPage)]
pub fn projects_page(props: &ProjectsPageProps) -> Html {
    let projects_state = use_state(|| None::<Vec<Project>>);
    let error_message_state = use_state(String::default);
    let loading_state = use_state(|| true);

    {
        let projects_state = projects_state.clone();
        let error_message_state = error_message_state.clone();
        let loading_state = loading_state.clone();
        use_effect_with_deps(move |_| {
            loading_state.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let client = reqwest::Client::new();
                match client
                    .get(&format!("{}/api/projects", API_BASE_URL))
                    .header(ACCEPT, "application/json")
                    .send()
                    .await
                {
                    Ok(response) => {
                        if response.status().is_success() {
                            match response.json::<Vec<Project>>().await {
                                Ok(projects_data) => {
                                    projects_state.set(Some(projects_data));
                                    error_message_state.set(String::default());
                                }
                                Err(e) => {
                                    error_message_state.set(format!("Failed to parse projects: {}", e));
                                }
                            }
                        } else {
                            let text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                            error_message_state.set(format!("Failed to fetch projects: {}", text));
                        }
                    }
                    Err(e) => {
                        error_message_state.set(format!("Network error fetching projects: {}", e));
                    }
                }
                loading_state.set(false);
            });
            || {}
        }, ());
    }

    let projects_style = style! {
        r#"
        .projects-container {
            padding: 40px 20px;
            max-width: 1200px;
            margin: 0 auto;
            min-height: calc(100vh - 100px); /* Adjust for header/footer */
            color: #FFF;
            position: relative;
            z-index: 1; /* Above WebGL canvas */
        }

        .projects-grid {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
            gap: 30px;
            padding-top: 30px;
        }

        .page-title {
            text-align: center;
            font-size: 3em;
            margin-bottom: 50px;
            text-shadow: 0 0 10px rgba(255, 255, 255, 0.4);
        }

        .loading-text, .error-text {
            text-align: center;
            font-size: 1.5em;
            margin-top: 50px;
        }

        .error-text {
            color: #FF6666;
        }

        @media (max-width: 768px) {
            .page-title {
                font-size: 2.5em;
            }
            .projects-grid {
                grid-template-columns: 1fr;
            }
        }
        "#
    }.expect("Failed to load projects page style");

    html! {
        <div class={projects_style}>
            <div class="projects-container">
                <h1 class="page-title">{"Our Projects"}</h1>
                {if *loading_state {
                    html! { <p class="loading-text">{"Loading projects..."}</p> }
                } else if !error_message_state.is_empty() {
                    html! { <p class="error-text">{&*error_message_state}</p> }
                } else if let Some(projects_list) = &*projects_state {
                    if projects_list.is_empty() {
                        html! { <p class="loading-text">{"No projects found."}</p> }
                    } else {
                        html! {
                            <div class="projects-grid">
                                {for projects_list.iter().map(|project| html! {
                                    <ProjectCard
                                        project={project.clone()}
                                        renderer={props.renderer.clone()} // Pass renderer if card needs 3D effects
                                    />
                                })}
                            </div>
                        }
                    }
                } else {
                    html! { <p class="loading-text">{"An unexpected state occurred."}</p> }
                }}
            </div>
        </div>
    }
}

// frontend/src/components/ProjectCard.rs (A placeholder)
// You would implement the actual 3D card flip here using CSS transforms.
// If using WebGL for the flip, it would interact with the passed `renderer`.
#[derive(Properties, PartialEq, Clone)]
pub struct ProjectCardProps {
    pub project: Project,
    pub renderer: UseStateHandle<Option<Rc<RefCell<webgl::Renderer>>>>, // Optional: if card needs 3D WebGL
}

#[function_component(ProjectCard)]
pub fn project_card(props: &ProjectCardProps) -> Html {
    let card_style = style! {
        r#"
        .project-card {
            background-color: #1a1a1a;
            border: 1px solid #333;
            border-radius: 8px;
            overflow: hidden;
            box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
            transition: transform 0.3s ease, box-shadow 0.3s ease;
            position: relative;
            aspect-ratio: 16 / 9; /* Maintain aspect ratio */
            perspective: 1000px; /* For 3D card flip */
            display: flex;
            flex-direction: column;
            justify-content: space-between;
        }

        .project-card:hover {
            transform: translateY(-10px);
            box-shadow: 0 8px 16px rgba(0, 0, 0, 0.4);
        }

        .card-inner {
            width: 100%;
            height: 100%;
            transition: transform 0.6s;
            transform-style: preserve-3d;
            position: absolute;
        }

        .project-card:hover .card-inner {
            transform: rotateY(180deg);
        }

        .card-front, .card-back {
            position: absolute;
            width: 100%;
            height: 100%;
            -webkit-backface-visibility: hidden; /* Hide back of the element when flipped */
            backface-visibility: hidden;
            display: flex;
            flex-direction: column;
            padding: 20px;
            box-sizing: border-box;
            color: #FFF;
        }

        .card-front {
            background-color: #1a1a1a;
            z-index: 2;
            text-align: center;
            justify-content: center;
            align-items: center;
        }

        .card-back {
            background-color: #2a2a2a;
            transform: rotateY(180deg);
            z-index: 1;
            text-align: left;
        }

        .project-image {
            width: 100%;
            height: 150px; /* Fixed height for image */
            object-fit: cover;
            margin-bottom: 15px;
            filter: grayscale(100%); /* Black and White filter */
            transition: filter 0.3s ease;
        }

        .project-card:hover .project-image {
            filter: grayscale(0%); /* Colorize on hover */
        }

        .project-title {
            font-size: 1.5em;
            margin-bottom: 10px;
            color: #FFF;
        }

        .project-description-front {
            font-size: 0.9em;
            color: #AAA;
            overflow: hidden;
            text-overflow: ellipsis;
            display: -webkit-box;
            -webkit-line-clamp: 3; /* Limit to 3 lines */
            -webkit-box-orient: vertical;
        }

        .project-description-back {
            font-size: 0.9em;
            color: #AAA;
            flex-grow: 1; /* Take remaining space */
            overflow-y: auto; /* Scroll if content overflows */
            padding-bottom: 10px; /* Space for button */
        }

        .project-tags {
            font-size: 0.8em;
            color: #888;
            margin-top: 10px;
        }

        .project-link {
            display: inline-block;
            margin-top: 15px;
            padding: 8px 15px;
            background-color: #444;
            color: #FFF;
            border-radius: 4px;
            text-decoration: none;
            transition: background-color 0.3s ease;
        }

        .project-link:hover {
            background-color: #666;
        }
        "#
    }.expect("Failed to load project card style");

    html! {
        <div class={card_style}>
            <div class="card-inner">
                <div class="card-front">
                    {if let Some(url) = &props.project.image_url {
                        html! { <img src={url.clone()} alt={props.project.title.clone()} class="project-image" /> }
                    } else {
                        html! {}
                    }}
                    <h3 class="project-title">{&props.project.title}</h3>
                    <p class="project-description-front">{&props.project.description}</p>
                    {if let Some(tags_str) = &props.project.tags {
                        html! { <p class="project-tags">{format!("Tags: {}", tags_str)}</p> }
                    } else { html! {} }}
                </div>
                <div class="card-back">
                    <h3 class="project-title">{&props.project.title}</h3>
                    <p class="project-description-back">{&props.project.description}</p>
                    {if let Some(project_url) = &props.project.project_url {
                        html! {
                            <a href={project_url.clone()} target="_blank" rel="noopener noreferrer" class="project-link">
                                {"View Project"}
                            </a>
                        }
                    } else { html! {} }}
                </div>
            </div>
        </div>
    }
}
