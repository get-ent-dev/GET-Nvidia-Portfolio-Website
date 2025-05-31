// frontend/src/pages/HomePage.rs
use yew::prelude::*;
use yew_router::prelude::*;
use stylist::{style, css};
use web_sys::HtmlCanvasElement;
use gloo_render::AnimationFrame;
use std::rc::Rc;
use std::cell::RefCell;
use log::info;

use crate::webgl; // Assume webgl module exists and has a Renderer

#[derive(Properties, PartialEq)]
pub struct HomePageProps {
    pub canvas_ref: NodeRef, // Pass the reference to the WebGL canvas
    pub renderer: UseStateHandle<Option<Rc<RefCell<webgl::Renderer>>>>, // Pass the renderer state handle
}

#[function_component(HomePage)]
pub fn home_page(props: &HomePageProps) -> Html {
    let navigator = use_navigator().expect("Navigator not found");

    let home_style = style! {
        r#"
        .home-section {
            min-height: 100vh;
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            text-align: center;
            padding: 20px;
            position: relative; /* Ensure content is above WebGL, z-index 1 */
            z-index: 1;
            box-sizing: border-box; /* Include padding in element's total width and height */
        }

        .intro-text {
            font-size: 3em;
            font-weight: bold;
            margin-bottom: 20px;
            text-shadow: 0 0 10px rgba(255, 255, 255, 0.5); /* Subtle glow */
            animation: fadeIn 2s ease-out;
            color: #FFF; /* Ensure text is white */
        }

        .sub-text {
            font-size: 1.2em;
            max-width: 800px;
            line-height: 1.6;
            margin-bottom: 40px;
            animation: slideUp 2s ease-out;
            color: #DDD; /* Slightly softer white */
        }

        .cta-button {
            padding: 15px 30px;
            font-size: 1.2em;
            border-radius: 5px;
            background-color: #333;
            color: #FFF;
            border: 2px solid #555;
            transition: background-color 0.3s, border-color 0.3s, transform 0.3s;
            text-transform: uppercase;
            letter-spacing: 1px;
        }

        .cta-button:hover {
            background-color: #555;
            border-color: #777;
            transform: translateY(-5px); /* Subtle lift effect */
        }

        @media (max-width: 768px) {
            .intro-text {
                font-size: 2em;
            }
            .sub-text {
                font-size: 1em;
            }
            .cta-button {
                padding: 10px 20px;
                font-size: 1em;
            }
        }

        @keyframes fadeIn {
            from { opacity: 0; }
            to { opacity: 1; }
        }

        @keyframes slideUp {
            from { transform: translateY(50px); opacity: 0; }
            to { transform: translateY(0); opacity: 1; }
        }
        "#
    }.expect("Failed to load home page style");

    // Effect to trigger 3D logo animation
    use_effect_with_deps(move |(canvas_ref, renderer_handle)| {
        info!("HomePage: use_effect_with_deps triggered.");
        if let (Some(canvas), Some(renderer_rc)) = (canvas_ref.cast::<HtmlCanvasElement>(), &**renderer_handle) {
            info!("HomePage: Canvas and renderer available. Triggering logo animation.");
            let mut renderer = renderer_rc.borrow_mut();
            // Assuming your renderer has a method to add/animate a specific logo
            renderer.add_logo_animation(); // Implement this in your webgl::Renderer
        } else {
            info!("HomePage: Canvas or renderer not yet available.");
        }
        || {} // Cleanup function
    }, (props.canvas_ref.clone(), props.renderer.clone()));


    html! {
        <div class={home_style}>
            <section class="home-section">
                <h1 class="intro-text">{"GET Nvidia: Innovation in AI & Electronics"}</h1>
                <p class="sub-text">
                    {"Discover our groundbreaking projects and solutions in the world of advanced computing and intelligent systems."}
                </p>
                <button
                    class="cta-button"
                    onclick={Callback::from(move |_| navigator.push(&Route::Projects))}
                >
                    {"View Our Projects"}
                </button>
            </section>
        </div>
    }
}
