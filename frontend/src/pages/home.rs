// frontend/src/pages/home.rs

use yew::prelude::*;
// No need for yew_router::prelude::* or crate::Route if not using Link/Switch here

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="card my-4 p-4 shadow-sm">
            <h1 class="text-center mb-4">{"Welcome to My Portfolio!"}</h1>
            <p class="lead text-center">
                {"I'm a passionate developer showcasing my projects and skills."}
            </p>
            <hr class="my-4" />
            <div class="row">
                <div class="col-md-6 mb-3">
                    <div class="card h-100">
                        <div class="card-body">
                            <h5 class="card-title">{"About Me"}</h5>
                            <p class="card-text">{"Learn more about my background, skills, and journey into software development."}</p>
                            // You might want a Link to About page here
                        </div>
                    </div>
                </div>
                <div class="col-md-6 mb-3">
                    <div class="card h-100">
                        <div class="card-body">
                            <h5 class="card-title">{"My Projects"}</h5>
                            <p class="card-text">{"Explore a collection of my past and ongoing projects, ranging from web applications to backend services."}</p>
                            // You might want a Link to Projects page here
                        </div>
                    </div>
                </div>
            </div>
            <p class="mt-4 text-center text-muted">
                {"Feel free to navigate through the site to discover more!"}
            </p>
        </div>
    }
}
