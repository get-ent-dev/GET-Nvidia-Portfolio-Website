// frontend/src/pages/projects.rs

use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route; // Import Route from your main.rs

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <div class="card my-4 p-4 shadow-sm">
            <h1 class="text-center mb-4">{"My Projects"}</h1>
            <p class="lead text-center">{"Here's a list of some of my work:"}</p>
            <hr class="my-4" />
            <ul class="list-group list-group-flush">
                <li class="list-group-item d-flex justify-content-between align-items-center">
                    <div>
                        <h5 class="mb-1">{"Project Alpha"}</h5>
                        <p class="mb-1">{"A cutting-edge web application built with Yew.rs and a Rust backend."}</p>
                    </div>
                    <Link<Route> to={Route::Project { id: 1 }} classes="btn btn-primary btn-sm">{"View Details"}</Link<Route>>
                </li>
                <li class="list-group-item d-flex justify-content-between align-items-center">
                    <div>
                        <h5 class="mb-1">{"Project Beta"}</h5>
                        <p class="mb-1">{"An IoT data processing pipeline using Rust and Kafka."}</p>
                    </div>
                    <Link<Route> to={Route::Project { id: 2 }} classes="btn btn-primary btn-sm">{"View Details"}</Link<Route>>
                </li>
                <li class="list-group-item d-flex justify-content-between align-items-center">
                    <div>
                        <h5 class="mb-1">{"Project Gamma"}</h5>
                        <p class="mb-1">{"A machine learning model inference service with a Rust API."}</p>
                    </div>
                    <Link<Route> to={Route::Project { id: 3 }} classes="btn btn-primary btn-sm">{"View Details"}</Link<Route>>
                </li>
                <li class="list-group-item d-flex justify-content-between align-items-center">
                    <div>
                        <h5 class="mb-1">{"Project Delta (Coming Soon!)"}</h5>
                        <p class="mb-1">{"Stay tuned for more exciting projects!"}</p>
                    </div>
                    <button class="btn btn-secondary btn-sm" disabled=true>{"No Details Yet"}</button>
                </li>
            </ul>
        </div>
    }
}
