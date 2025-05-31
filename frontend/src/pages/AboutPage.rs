// frontend/src/pages/AboutPage.rs
use yew::prelude::*;
use stylist::{style, css};
use serde::{Deserialize, Serialize};
use reqwest::header::ACCEPT;
use log::error;

use crate::api::API_BASE_URL;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct AboutPageContent {
    pub id: String,
    pub content: String,
    pub image_url: Option<String>,
    // created_at and updated_at might not be needed for display
}

#[function_component(AboutPage)]
pub fn about_page() -> Html {
    let about_content_state = use_state(|| None::<AboutPageContent>);
    let error_message_state = use_state(String::default);
    let loading_state = use_state(|| true);

    {
        let about_content_state = about_content_state.clone();
        let error_message_state = error_message_state.clone();
        let loading_state = loading_state.clone();
        use_effect_with_deps(move |_| {
            loading_state.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                let client = reqwest::Client::new();
                match client
                    .get(&format!("{}/api/about", API_BASE_URL))
                    .header(ACCEPT, "application/json")
                    .send()
                    .await
                {
                    Ok(response) => {
                        if response.status().is_success() {
                            match response.json::<AboutPageContent>().await {
                                Ok(content_data) => {
                                    about_content_state.set(Some(content_data));
                                    error_message_state.set(String::default());
                                }
                                Err(e) => {
                                    error_message_state.set(format!("Failed to parse about content: {}", e));
                                }
                            }
                        } else {
                            let text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                            error_message_state.set(format!("Failed to fetch about content: {}", text));
                        }
                    }
                    Err(e) => {
                        error_message_state.set(format!("Network error fetching about content: {}", e));
                    }
                }
                loading_state.set(false);
            });
            || {}
        }, ());
    }

    let about_style = style! {
        r#"
        .about-container {
            padding: 40px 20px;
            max-width: 900px;
            margin: 0 auto;
            min-height: calc(100vh - 100px);
            color: #FFF;
            line-height: 1.8;
            position: relative;
            z-index: 1;
        }

        .page-title {
            text-align: center;
            font-size: 3em;
            margin-bottom: 40px;
            text-shadow: 0 0 10px rgba(255, 255, 255, 0.4);
            animation: fadeIn 1.5s ease-out;
        }

        .about-content {
            background-color: #1a1a1a;
            padding: 30px;
            border-radius: 8px;
            box-shadow: 0 4px 15px rgba(0, 0, 0, 0.3);
            text-align: justify;
            opacity: 0; /* For initial fade-in */
            transform: translateY(20px); /* For initial slide-up */
            animation: fadeInSlideUp 1.5s ease-out forwards 0.5s; /* Apply animation with delay */
        }

        .about-image {
            width: 100%;
            height: 300px;
            object-fit: cover;
            border-radius: 8px;
            margin-bottom: 30px;
            filter: grayscale(100%);
            box-shadow: 0 4px 10px rgba(0, 0, 0, 0.2);
            transition: filter 0.5s ease;
        }

        .about-image:hover {
            filter: grayscale(0%);
        }

        .loading-text, .error-text {
            text-align: center;
            font-size: 1.5em;
            margin-top: 50px;
        }

        .error-text {
            color: #FF6666;
        }

        @keyframes fadeInSlideUp {
            from { opacity: 0; transform: translateY(20px); }
            to { opacity: 1; transform: translateY(0); }
        }

        @media (max-width: 768px) {
            .page-title {
                font-size: 2.5em;
            }
            .about-container {
                padding: 20px;
            }
            .about-content {
                padding: 20px;
            }
        }
        "#
    }.expect("Failed to load about page style");

    html! {
        <div class={about_style}>
            <div class="about-container">
                <h1 class="page-title">{"About GET Nvidia"}</h1>
                {if *loading_state {
                    html! { <p class="loading-text">{"Loading about content..."}</p> }
                } else if !error_message_state.is_empty() {
                    html! { <p class="error-text">{&*error_message_state}</p> }
                } else if let Some(content) = &*about_content_state {
                    html! {
                        <div class="about-content">
                            {if let Some(img_url) = &content.image_url {
                                html! { <img src={img_url.clone()} alt="About GET Nvidia" class="about-image" /> }
                            } else { html! {} }}
                            <p>{content.content.clone()}</p>
                        </div>
                    }
                } else {
                    html! { <p class="loading-text">{"About content not available."}</p> }
                }}
            </div>
        </div>
    }
}
