// frontend/src/pages/AdminPage.rs
use yew::prelude::*;
use yew_router::prelude::*;
use stylist::{style, css};
use serde::{Deserialize, Serialize};
use reqwest::header::CONTENT_TYPE;
use web_sys::HtmlInputElement;
use log::{info, error};

use crate::api::API_BASE_URL;
use crate::Route; // Assuming Route is in main.rs or a shared module
use crate::contexts::user_context::{UserContext, User}; // Assuming a UserContext for global state

// Admin Dashboard components (placeholders)
mod admin_projects_panel;
mod admin_about_panel;
mod admin_contact_messages_panel;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Properties, PartialEq)]
pub struct AdminPageProps {
    // Add props if needed, e.g., to pass the renderer for 3D viz
}

#[function_component(AdminPage)]
pub fn admin_page(props: &AdminPageProps) -> Html {
    let navigator = use_navigator().expect("Navigator not found");
    let user_ctx = use_context::<UserContext>().expect("UserContext not found");

    let username_state = use_state(String::default);
    let password_state = use_state(String::default);
    let error_message_state = use_state(String::default);
    let is_submitting_state = use_state(|| false);

    // Effect to check authentication status on load
    {
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        use_effect_with_deps(move |_| {
            // Check if already logged in via context.
            // In a more robust app, you might have an API endpoint to verify session validity.
            if user_ctx.is_authenticated() {
                info!("AdminPage: Already authenticated, redirecting to dashboard.");
                navigator.push(&Route::AdminDashboard);
            }
            || {}
        }, ());
    }


    let on_username_change = Callback::from(|e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        username_state.set(input.value());
    });

    let on_password_change = Callback::from(|e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        password_state.set(input.value());
    });

    let on_submit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default(); // Prevent default form submission
        let username = (*username_state).clone();
        let password = (*password_state).clone();
        let error_message = error_message_state.clone();
        let is_submitting = is_submitting_state.clone();
        let user_ctx = user_ctx.clone();
        let navigator_clone = navigator.clone();

        if *is_submitting { return; } // Prevent double submission
        is_submitting.set(true);
        error_message.set(String::default()); // Clear previous error

        wasm_bindgen_futures::spawn_local(async move {
            let client = reqwest::Client::new();
            let login_data = LoginRequest {
                username: username,
                password: password,
            };

            match client
                .post(&format!("{}/api/auth/login", API_BASE_URL))
                .header(CONTENT_TYPE, "application/json")
                .json(&login_data)
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        info!("Login successful!");
                        // Assume backend returns user data, or just a success
                        user_ctx.login(User { id: "admin_id".to_string(), username: "get".to_string() }); // Update context
                        navigator_clone.push(&Route::AdminDashboard); // Redirect
                    } else {
                        let text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                        error!("Login failed: status={}, body={}", response.status(), text);
                        error_message.set(format!("Login failed: {}", text));
                    }
                }
                Err(e) => {
                    error!("Network error during login: {}", e);
                    error_message.set(format!("Network error: {}", e));
                }
            }
            is_submitting.set(false);
        });
    });

    let admin_style = style! {
        r#"
        .admin-container {
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: calc(100vh - 100px); /* Adjust for header/footer */
            background-color: #0a0a0a;
            color: #FFF;
            position: relative;
            z-index: 1;
            padding: 20px;
        }

        .login-form {
            background-color: #1a1a1a;
            padding: 40px;
            border-radius: 8px;
            box-shadow: 0 0 30px rgba(255, 255, 255, 0.15); /* More prominent glow */
            text-align: center;
            width: 100%;
            max-width: 450px;
            border: 1px solid #333;
            animation: fadeInScale 1s ease-out; /* Add animation */
        }

        .login-form h2 {
            color: #FFF;
            margin-bottom: 30px;
            font-size: 2.5em;
            text-shadow: 0 0 8px rgba(255, 255, 255, 0.3);
        }

        .form-group {
            margin-bottom: 25px;
            text-align: left;
        }

        .form-group label {
            display: block;
            margin-bottom: 10px;
            color: #DDD;
            font-size: 1.1em;
        }

        .form-group input {
            width: calc(100% - 24px); /* Padding + border */
            padding: 12px;
            border: 1px solid #444;
            background-color: #0d0d0d;
            color: #FFF;
            border-radius: 4px;
            font-size: 1.05em;
            transition: border-color 0.3s ease, box-shadow 0.3s ease;
            box-shadow: inset 0 0 5px rgba(255, 255, 255, 0.05);
        }

        .form-group input:focus {
            border-color: #777;
            box-shadow: 0 0 15px rgba(255, 255, 255, 0.2), inset 0 0 5px rgba(255, 255, 255, 0.1);
            outline: none;
            transform: scale(1.005);
        }

        .login-button {
            width: 100%;
            padding: 15px;
            background-color: #333;
            color: #FFF;
            border: 2px solid #555;
            border-radius: 5px;
            font-size: 1.2em;
            cursor: pointer;
            transition: background-color 0.3s ease, border-color 0.3s ease, transform 0.3s;
            text-transform: uppercase;
            letter-spacing: 1px;
            margin-top: 10px;
        }

        .login-button:hover {
            background-color: #555;
            border-color: #777;
            transform: translateY(-5px);
        }

        .login-button:disabled {
            background-color: #222;
            border-color: #444;
            cursor: not-allowed;
            opacity: 0.7;
        }

        .error-message {
            color: #FF6666;
            margin-top: 20px;
            font-size: 1em;
            font-weight: bold;
            animation: shake 0.5s;
        }

        @keyframes fadeInScale {
            from { opacity: 0; transform: scale(0.95); }
            to { opacity: 1; transform: scale(1); }
        }

        @keyframes shake {
            0%, 100% { transform: translateX(0); }
            10%, 30%, 50%, 70%, 90% { transform: translateX(-5px); }
            20%, 40%, 60%, 80% { transform: translateX(5px); }
        }
        "#
    }.expect("Failed to load admin style");

    // If already authenticated, redirect to AdminDashboard (handled by use_effect)
    // For local dev, we might render the dashboard here directly
    if user_ctx.is_authenticated() {
        html! {
            <div class={admin_style.clone()}>
                <div class="admin-container">
                    <admin_dashboard_layout::AdminDashboardLayout>
                        <Switch<Route> render={admin_dashboard_switch} />
                    </admin_dashboard_layout::AdminDashboardLayout>
                </div>
            </div>
        }
    } else {
        // Show login form
        html! {
            <div class={admin_style}>
                <div class="admin-container">
                    <form class="login-form" onsubmit={on_submit}>
                        <h2>{"Admin Login"}</h2>
                        <div class="form-group">
                            <label for="username">{"Username"}</label>
                            <input type="text" id="username" value={(*username_state).clone()} onchange={on_username_change} required=true autocomplete="username" />
                        </div>
                        <div class="form-group">
                            <label for="password">{"Password"}</label>
                            <input type="password" id="password" value={(*password_state).clone()} onchange={on_password_change} required=true autocomplete="current-password" />
                        </div>
                        <button type="submit" class="login-button" disabled={*is_submitting_state}>
                            {if *is_submitting_state { "Logging in..." } else { "Login" }}
                        </button>
                        {if !error_message_state.is_empty() {
                            html! { <p class="error-message">{&*error_message_state}</p> }
                        } else { html! {} }}
                    </form>
                </div>
            </div>
        }
    }
}
