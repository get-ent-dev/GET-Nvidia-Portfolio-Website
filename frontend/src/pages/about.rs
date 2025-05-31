// frontend/src/pages/about.rs

use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class="card my-4 p-4 shadow-sm">
            <h1 class="text-center mb-4">{"About Me"}</h1>
            <p class="text-justify">
                {"Hello! I'm a Rust enthusiast with a knack for building robust and efficient applications. My journey into programming started with a curiosity for how things work, leading me down the path of software development."}
            </p>
            <p class="text-justify">
                {"I specialize in web development using Yew.rs for frontends and Rust/Actix-web for backends. I also have experience with databases like PostgreSQL and working with cloud platforms."}
            </p>
            <h2 class="mt-5 mb-3">{"Skills"}</h2>
            <ul class="list-group list-group-flush mb-4">
                <li class="list-group-item">{"Rust (Yew, Actix-web, Tokio)"}</li>
                <li class="list-group-item">{"WebAssembly (Wasm)"}</li>
                <li class="list-group-item">{"JavaScript/TypeScript"}</li>
                <li class="list-group-item">{"HTML5 & CSS3 (Bootstrap, Stylist)"}</li>
                <li class="list-group-item">{"PostgreSQL, MongoDB"}</li>
                <li class="list-group-item">{"Git/GitHub"}</li>
                <li class="list-group-item">{"Linux, Docker"}</li>
            </ul>
            <h2 class="mt-5 mb-3">{"My Philosophy"}</h2>
            <p class="text-justify">
                {"I believe in writing clean, maintainable, and performant code. I'm always eager to learn new technologies and improve my craft. Open source contributions and community engagement are also important aspects of my development journey."}
            </p>
        </div>
    }
}
