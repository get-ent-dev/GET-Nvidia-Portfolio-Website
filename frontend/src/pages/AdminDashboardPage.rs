// frontend/src/pages/AdminDashboardPage.rs (Placeholder for the main dashboard)
use yew::prelude::*;
use stylist::style;
use crate::webgl; // Assume webgl module exists
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Properties, PartialEq)]
pub struct AdminDashboardPageProps {
    // Pass renderer if 3D visualizations are on the main dashboard
    // pub renderer: UseStateHandle<Option<Rc<RefCell<webgl::Renderer>>>>,
}

#[function_component(AdminDashboardPage)]
pub fn admin_dashboard_page(props: &AdminDashboardPageProps) -> Html {
    let dashboard_style = style! {
        r#"
        .dashboard-content {
            padding: 20px;
        }

        .dashboard-content h2 {
            font-size: 2.5em;
            margin-bottom: 30px;
            color: #FFF;
        }

        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 20px;
            margin-bottom: 40px;
        }

        .stat-card {
            background-color: #1a1a1a;
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 4px 10px rgba(0,0,0,0.3);
            text-align: center;
            border: 1px solid #333;
        }

        .stat-card h3 {
            font-size: 1.5em;
            color: #DDD;
            margin-bottom: 10px;
        }

        .stat-card p {
            font-size: 2.5em;
            font-weight: bold;
            color: #FFF;
        }

        .chart-container {
            background-color: #1a1a1a;
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 4px 10px rgba(0,0,0,0.3);
            height: 400px; /* Adjust as needed for your 3D viz */
            position: relative;
            border: 1px solid #333;
        }

        /* 3D visualization specific CSS if needed */
        /* For WebGL canvas within the chart container, adjust its position/size */
        .chart-canvas {
            width: 100%;
            height: 100%;
            display: block;
        }
        "#
    }.expect("Failed to load admin dashboard style");

    // Example data for visualization (fetch from backend in real app)
    let total_projects = use_state(|| 15);
    let new_messages = use_state(|| 3);
    let total_visitors = use_state(|| 12345);

    // Placeholder for 3D data visualization using WebGL
    // This would ideally interact with your webgl::Renderer to draw charts
    // You might need to pass a canvas reference here, or have the renderer manage internal canvases.
    use_effect_with_deps(move |_| {
        // If props.renderer is available, you could call a method:
        // if let Some(renderer_rc) = &**props.renderer {
        //     let mut renderer = renderer_rc.borrow_mut();
        //     renderer.render_bar_chart(&vec![10.0, 20.0, 5.0, 15.0]);
        // }
        || {}
    }, ());


    html! {
        <div class={dashboard_style}>
            <div class="dashboard-content">
                <h2>{"Admin Dashboard"}</h2>

                <div class="stats-grid">
                    <div class="stat-card">
                        <h3>{"Total Projects"}</h3>
                        <p>{*total_projects}</p>
                    </div>
                    <div class="stat-card">
                        <h3>{"New Messages"}</h3>
                        <p>{*new_messages}</p>
                    </div>
                    <div class="stat-card">
                        <h3>{"Total Visitors"}</h3>
                        <p>{*total_visitors}</p>
                    </div>
                    {/* Add more stats as needed */}
                </div>

                <h3>{"Visitor Statistics (3D Visualization)"}</h3>
                <div class="chart-container">
                    // This is where your WebGL canvas for charts would go.
                    // You might create a dedicated canvas element here and pass its ref to the renderer.
                    <canvas id="dashboard-chart-canvas" class="chart-canvas"></canvas>
                    <p>{"(Placeholder for interactive 3D visitor statistics chart)"}</p>
                </div>
            </div>
        </div>
    }
}
