// frontend/src/pages/not_found.rs

use yew::prelude::*;
use yew_router::prelude::*; // Needed for Link component
use crate::Route; // Import Route enum from main.rs

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="card my-4 p-4 text-center shadow-sm">
            <h1 class="display-1 text-danger">{"404"}</h1>
            <h2 class="mb-3">{"Page Not Found"}</h2>
            <p class="lead">{"Oops! The page you're looking for doesn't exist."}</p>
            <p>
                {"You can go back to the "}
                <Link<Route> to={Route::Home} classes="btn btn-link p-0 align-baseline">{"home page"}</Link<Route>>
                {"."}
            </p>
            <img
                src="https://media.giphy.com/media/unQ3aYFGt6K1W/giphy.gif" // A fun 404 GIF
                alt="Page Not Found GIF"
                class="img-fluid rounded mx-auto d-block mt-4"
                style="max-width: 400px;"
            />
        </div>
    }
}
