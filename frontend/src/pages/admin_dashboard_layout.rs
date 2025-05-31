// frontend/src/pages/admin_dashboard_layout.rs (New file for admin dashboard structure)
// This file would handle the sidebar navigation for admin.
use yew::prelude::*;
use yew_router::prelude::*;
use stylist::style;
use crate::Route; // Assuming Route is accessible
use crate::contexts::user_context::UserContext;
use reqwest::header::ACCEPT;
use log::error;

#[derive(Properties, PartialEq)]
pub struct AdminDashboardLayoutProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(AdminDashboardLayout)]
pub fn admin_dashboard_layout(props: &AdminDashboardLayoutProps) -> Html {
    let navigator = use_navigator().expect("Navigator not found");
    let user_ctx = use_context::<UserContext>().expect("UserContext not found");

    let logout_callback = Callback::from(move |_| {
        let user_ctx = user_ctx.clone();
        let navigator = navigator.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let client = reqwest::Client::new();
            match client
                .post(&format!("{}/api/admin/logout", crate::api::API_BASE_URL))
                .header(ACCEPT, "application/json")
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        user_ctx.logout(); // Clear context
                        navigator.push(&Route::Admin); // Redirect to login
                    } else {
                        error!("Logout failed: {:?}", response.text().await);
                    }
                }
                Err(e) => {
                    error!("Network error during logout: {}", e);
                }
            }
        });
    });

    let layout_style = style! {
        r#"
        .admin-dashboard-layout {
            display: flex;
            min-height: calc(100vh - 100px); /* Adjust for header/footer */
            background-color: #0a0a0a;
            color: #FFF;
        }

        .admin-sidebar {
            width: 250px;
            background-color: #1a1a1a;
            padding: 20px;
            border-right: 1px solid #333;
            box-shadow: 2px 0 10px rgba(0, 0, 0, 0.3);
            display: flex;
            flex-direction: column;
            justify-content: space-between;
        }

        .admin-nav ul {
            list-style: none;
            padding: 0;
            margin: 0;
        }

        .admin-nav li {
            margin-bottom: 15px;
        }

        .admin-nav a {
            display: block;
            padding: 10px 15px;
            color: #FFF;
            text-decoration: none;
            border-radius: 5px;
            transition: background-color 0.3s ease, color 0.3s ease;
            font-size: 1.1em;
        }

        .admin-nav a:hover, .admin-nav a.active {
            background-color: #333;
            color: #FFF;
        }

        .admin-content {
            flex-grow: 1;
            padding: 40px;
            overflow-y: auto;
            position: relative;
            z-index: 1;
        }

        .logout-button {
            padding: 10px 15px;
            background-color: #555;
            color: #FFF;
            border: 1px solid #777;
            border-radius: 5px;
            cursor: pointer;
            transition: background-color 0.3s ease;
            margin-top: 30px;
        }

        .logout-button:hover {
            background-color: #777;
        }
        "#
    }.expect("Failed to load admin layout style");

    html! {
        <div class={layout_style}>
            <aside class="admin-sidebar">
                <nav class="admin-nav">
                    <ul>
                        <li><Link<Route> to={Route::AdminDashboard}>{"Dashboard"}</Link<Route>></li>
                        <li><Link<Route> to={Route::AdminProjects}>{"Projects"}</Link<Route>></li>
                        <li><Link<Route> to={Route::AdminAbout}>{"About Page"}</Link<Route>></li>
                        <li><Link<Route> to={Route::AdminContactMessages}>{"Contact Messages"}</Link<Route>></li>
                    </ul>
                </nav>
                <button class="logout-button" onclick={logout_callback}>{"Logout"}</button>
            </aside>
            <main class="admin-content">
                {props.children.clone()}
            </main>
        </div>
    }
}
