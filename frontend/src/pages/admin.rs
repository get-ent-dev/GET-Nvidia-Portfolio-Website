// frontend/src/pages/admin.rs

use yew::prelude::*;
use web_sys::HtmlInputElement;
use gloo_console::log;

#[function_component(Admin)]
pub fn admin() -> Html {
    let username_state = use_state(|| "".to_string());
    let password_state = use_state(|| "".to_string());
    let message_state = use_state(|| "".to_string());

    // Clone the handles *before* the callbacks' move closures for on_change
    let username_state_on_change = username_state.clone();
    let password_state_on_change = password_state.clone();

    let on_username_change = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        username_state_on_change.set(input.value());
    });

    let on_password_change = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        password_state_on_change.set(input.value());
    });

    let on_submit = Callback::from({
        // Clone the handles *specifically for this move closure*
        let username_state_for_submit = username_state.clone();
        let password_state_for_submit = password_state.clone();
        let message_state_for_submit = message_state.clone();
        move |e: SubmitEvent| {
            e.prevent_default();

            // Use the cloned handles within this closure
            let username = (*username_state_for_submit).clone();
            let password = (*password_state_for_submit).clone();

            log!(format!("Attempting login for: {}", username));

            if username == "admin" && password == "password" {
                message_state_for_submit.set("Login successful! Redirecting... (not actually redirecting yet)".to_string());
            } else {
                message_state_for_submit.set("Invalid username or password.".to_string());
            }
        }
    });

    html! {
        <div class="card my-4 p-4 shadow-sm">
            <h1 class="text-center mb-4">{"Admin Panel Login"}</h1>
            <p class="text-center">{"Access restricted. Please log in."}</p>
            <hr class="my-4" />
            <div class="row justify-content-center">
                <div class="col-md-6">
                    <form onsubmit={on_submit}>
                        <div class="mb-3">
                            <label for="adminUsername" class="form-label">{"Username"}</label>
                            <input
                                type="text"
                                class="form-control"
                                id="adminUsername"
                                value={(*username_state).clone()} // This original handle is still available for the html! macro
                                onchange={on_username_change}
                                required=true
                            />
                        </div>
                        <div class="mb-3">
                            <label for="adminPassword" class="form-label">{"Password"}</label>
                            <input
                                type="password"
                                class="form-control"
                                id="adminPassword"
                                value={(*password_state).clone()} // This original handle is still available for the html! macro
                                onchange={on_password_change}
                                required=true
                            />
                        </div>
                        {
                            // This original handle is also available for the html! macro
                            if !message_state.is_empty() {
                                html! {
                                    <div class={if message_state.starts_with("Login successful") {"alert alert-success"} else {"alert alert-danger"}}>
                                        {(*message_state).clone()}
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                        <div class="d-grid gap-2">
                            <button type="submit" class="btn btn-primary mt-3">{"Login"}</button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    }
}
