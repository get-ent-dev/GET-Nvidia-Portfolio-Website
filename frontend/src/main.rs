// frontend/src/main.rs

use yew::prelude::*;
use yew_router::prelude::*;
// use stylist::GlobalStyle; // Commented out for stylist removal test
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
// use std::rc::Rc; // Removed as no longer directly used in main.rs
// use std::cell::RefCell; // Removed as no longer directly used in main.rs
use web_sys::js_sys::Function;

// Import your pages and routes
mod pages;
// mod themes; // Commented out for themes removal test

// Corrected: Ensure these are pub components within their respective files
use pages::{About, Contact, Home, NotFound, ProjectDetail, Projects, Admin};
// use themes::Theme; // Commented out for themes removal test

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/projects")]
    Projects,
    #[at("/projects/:id")]
    Project { id: u32 },
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,
    #[at("/admin")]
    Admin,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Projects => html! { <Projects /> },
        Route::Project { id } => html! { <ProjectDetail project_id={id} /> },
        Route::About => html! { <About /> },
        Route::Contact => html! { <Contact /> },
        Route::Admin => html! { <Admin /> },
        Route::NotFound => html! { <NotFound /> },
    }
}

// Global CSS for the application
// #[function_component(AppGlobalStyle)] // Commented out for stylist removal test
// fn app_global_style() -> Html { // Commented out for stylist removal test
//     let global_css_instance = themes::get_global_css_instance(); // Commented out for stylist removal test
//     html! { // Commented out for stylist removal test
//         <GlobalStyle css={global_css_instance} /> // Commented out for stylist removal test
//     } // Commented out for stylist removal test
// } // Commented out for stylist removal test


#[function_component(App)]
fn app() -> Html {
    // let _current_theme = use_state(|| Theme::Dark); // Commented out as Theme is from themes.rs

    let animation_time = use_state(|| 0.0);

    // This `callback_ref` holds the `Closure` for its lifetime and across re-renders.
    let callback_ref = use_mut_ref(|| None::<Closure<dyn Fn(f64)>>);
    let _animation_handle_ref = use_mut_ref(|| None::<i32>); // Stores the request ID for cleanup

    {
        let animation_time_clone = animation_time.clone();
        // This is the Rc that will be moved into the outer `use_effect_with` closure.
        // It will be used to store the created Closure and make the initial RAF call.
        let raf_control_rc = callback_ref.clone();

        use_effect_with((), move |_| {
            // This clone is specifically for the *inner* `Closure::new` to capture.
            // It allows the `Closure` to refer to itself for recursive calls without
            // moving `raf_control_rc` itself into the inner closure's scope.
            let raf_recursive_rc = raf_control_rc.clone();

            let callback_closure = Closure::new(move |timestamp: f64| {
                animation_time_clone.set(timestamp);

                // For the next animation frame, use the `raf_recursive_rc`
                // that was captured by *this* specific `Closure` instance.
                web_sys::window()
                    .unwrap()
                    .request_animation_frame(
                        raf_recursive_rc // Use this Rc for the recursive call
                            .borrow() // Borrow the RefCell contents
                            .as_ref() // Get Option<&Closure>
                            .unwrap() // Get &Closure
                            .as_ref() // Get &JsValue
                            .dyn_ref::<Function>() // Cast to Function
                            .unwrap()
                    )
                    .unwrap();
            });

            // Store the owned `callback_closure` into the `RefCell` managed by `raf_control_rc`.
            // This makes the `Closure` self-referential via the `Rc<RefCell>` and also
            // allows the `use_effect_with` closure to keep a handle to it.
            *raf_control_rc.borrow_mut() = Some(callback_closure);

            // Initial call to request_animation_frame using the stored closure.
            // Use `raf_control_rc` here, as it is owned by the `use_effect_with` closure.
            let _request_id = web_sys::window() // Prepending with _ to silence unused_variables warning
                .unwrap()
                .request_animation_frame(
                    raf_control_rc // Use this Rc for the initial call
                        .borrow()
                        .as_ref()
                        .unwrap()
                        .as_ref()
                        .dyn_ref::<Function>()
                        .unwrap()
                )
                .unwrap();

            // You can store the request_id in _animation_handle_ref if you need to cancel it later.
            // *_animation_handle_ref.borrow_mut() = Some(request_id);

            move || {
                // Cleanup logic. When this closure (from use_effect_with) is dropped,
                // `raf_control_rc` will be dropped, and thus the stored `Closure` will be dropped.
            }
        });
    }

    html! {
        <BrowserRouter>
            // <AppGlobalStyle /> // Commented out for stylist removal test
            <header class="navbar navbar-expand-lg navbar-light bg-light">
                <div class="container-fluid">
                    <Link<Route> to={Route::Home} classes="navbar-brand">
                        {"My Portfolio"}
                    </Link<Route>>
                    <div class="collapse navbar-collapse">
                        <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                            <li class="nav-item">
                                <Link<Route> to={Route::Home} classes="nav-link">{"Home"}</Link<Route>>
                            </li>
                            <li class="nav-item">
                                <Link<Route> to={Route::Projects} classes="nav-link">{"Projects"}</Link<Route>>
                            </li>
                            <li class="nav-item">
                                <Link<Route> to={Route::About} classes="nav-link">{"About"}</Link<Route>>
                            </li>
                            <li class="nav-item">
                                <Link<Route> to={Route::Contact} classes="nav-link">{"Contact"}</Link<Route>>
                            </li>
                            <li class="nav-item">
                                <Link<Route> to={Route::Admin} classes="nav-link">{"Admin"}</Link<Route>>
                            </li>
                        </ul>
                    </div>
                </div>
            </header>
            <main class="container mt-4">
                <Switch<Route> render={switch} />
            </main>
            <footer class="mt-5 p-3 bg-light text-center">
                <p>{"© 2025 My Portfolio. All rights reserved."}</p>
                <p>{"Animation time: "}{format!("{:.2}", *animation_time)}</p>
            </footer>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
