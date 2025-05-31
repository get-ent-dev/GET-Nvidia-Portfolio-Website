// frontend/src/pages/contact.rs

use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <div class="card my-4 p-4 shadow-sm">
            <h1 class="text-center mb-4">{"Contact Me"}</h1>
            <p class="text-center">
                {"Have a question or want to collaborate? Feel free to reach out!"}
            </p>
            <hr class="my-4" />
            <div class="row justify-content-center">
                <div class="col-md-8">
                    <form>
                        <div class="mb-3">
                            <label for="name" class="form-label">{"Name"}</label>
                            <input type="text" class="form-control" id="name" placeholder="Your Name" />
                        </div>
                        <div class="mb-3">
                            <label for="email" class="form-label">{"Email address"}</label>
                            <input type="email" class="form-control" id="email" placeholder="name@example.com" />
                        </div>
                        <div class="mb-3">
                            <label for="subject" class="form-label">{"Subject"}</label>
                            <input type="text" class="form-control" id="subject" placeholder="Subject of your message" />
                        </div>
                        <div class="mb-3">
                            <label for="message" class="form-label">{"Message"}</label>
                            <textarea class="form-control" id="message" rows="5" placeholder="Your message"></textarea>
                        </div>
                        <div class="d-grid gap-2">
                            <button type="submit" class="btn btn-primary">{"Send Message"}</button>
                        </div>
                    </form>
                    <p class="text-center mt-4">
                        {"You can also find me on:"}
                    </p>
                    <ul class="list-inline text-center">
                        <li class="list-inline-item">
                            <a href="https://github.com/yourusername" target="_blank" class="text-decoration-none">
                                <i class="fab fa-github fa-2x"></i> {"GitHub"}
                            </a>
                        </li>
                        <li class="list-inline-item ms-3">
                            <a href="https://linkedin.com/in/yourusername" target="_blank" class="text-decoration-none">
                                <i class="fab fa-linkedin fa-2x"></i> {"LinkedIn"}
                            </a>
                        </li>
                        // Add more social links if you have them
                    </ul>
                </div>
            </div>
        </div>
    }
}
