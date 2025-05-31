// frontend/src/pages/ContactPage.rs
use yew::prelude::*;
use stylist::{style, css};
use serde::{Deserialize, Serialize};
use reqwest::header::CONTENT_TYPE;
use web_sys::HtmlInputElement;
use web_sys::HtmlTextAreaElement;
use log::{info, error};

use crate::api::API_BASE_URL;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContactFormSubmission {
    pub name: String,
    pub email: String,
    pub subject: Option<String>,
    pub message: String,
}

#[function_component(ContactPage)]
pub fn contact_page() -> Html {
    let name_state = use_state(String::default);
    let email_state = use_state(String::default);
    let subject_state = use_state(String::default);
    let message_state = use_state(String::default);
    let submission_status_state = use_state(|| None::<String>); // None, Some("success"), Some("error")
    let is_submitting_state = use_state(|| false);

    let on_name_change = Callback::from(|e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        name_state.set(input.value());
    });

    let on_email_change = Callback::from(|e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        email_state.set(input.value());
    });

    let on_subject_change = Callback::from(|e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        subject_state.set(input.value());
    });

    let on_message_change = Callback::from(|e: Event| {
        let input: HtmlTextAreaElement = e.target_unchecked_into();
        message_state.set(input.value());
    });

    let on_submit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default(); // Prevent default form submission
        let name = (*name_state).clone();
        let email = (*email_state).clone();
        let subject = (*subject_state).clone();
        let message = (*message_state).clone();
        let submission_status = submission_status_state.clone();
        let is_submitting = is_submitting_state.clone();

        if name.is_empty() || email.is_empty() || message.is_empty() {
            submission_status.set(Some("error: Please fill in all required fields.".to_string()));
            return;
        }
        if *is_submitting { return; } // Prevent double submission

        is_submitting.set(true);
        submission_status.set(None); // Clear previous status

        wasm_bindgen_futures::spawn_local(async move {
            let client = reqwest::Client::new();
            let contact_data = ContactFormSubmission {
                name,
                email,
                subject: if subject.is_empty() { None } else { Some(subject) },
                message,
            };

            match client
                .post(&format!("{}/api/contact", API_BASE_URL))
                .header(CONTENT_TYPE, "application/json")
                .json(&contact_data)
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        submission_status.set(Some("success: Your message has been sent successfully!".to_string()));
                        // Clear form fields
                        name_state.set(String::default());
                        email_state.set(String::default());
                        subject_state.set(String::default());
                        message_state.set(String::default());
                    } else {
                        let text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                        error!("Contact form submission failed: status={}, body={}", response.status(), text);
                        submission_status.set(Some(format!("error: Failed to send message. Server responded: {}", text)));
                    }
                }
                Err(e) => {
                    error!("Network error sending contact form: {}", e);
                    submission_status.set(Some(format!("error: Network error. Please try again later. {}", e)));
                }
            }
            is_submitting.set(false);
        });
    });

    let contact_style = style! {
        r#"
        .contact-container {
            padding: 40px 20px;
            max-width: 700px;
            margin: 0 auto;
            min-height: calc(100vh - 100px);
            color: #FFF;
            position: relative;
            z-index: 1;
        }

        .page-title {
            text-align: center;
            font-size: 3em;
            margin-bottom: 40px;
            text-shadow: 0 0 10px rgba(255, 255, 255, 0.4);
            animation: fadeIn 1.5s ease-out;
        }

        .contact-form {
            background-color: #1a1a1a;
            padding: 30px;
            border-radius: 8px;
            box-shadow: 0 4px 15px rgba(0, 0, 0, 0.3);
            display: flex;
            flex-direction: column;
            animation: fadeInSlideUp 1.5s ease-out forwards 0.5s;
        }

        .form-group {
            margin-bottom: 20px;
        }

        .form-group label {
            display: block;
            margin-bottom: 8px;
            color: #DDD;
            font-size: 1.1em;
        }

        .form-group input[type="text"],
        .form-group input[type="email"],
        .form-group textarea {
            width: calc(100% - 24px); /* Account for padding and border */
            padding: 12px;
            border: 1px solid #333;
            background-color: #0d0d0d;
            color: #FFF;
            border-radius: 4px;
            font-size: 1em;
            transition: border-color 0.3s ease, box-shadow 0.3s ease;
            box-shadow: inset 0 0 5px rgba(255, 255, 255, 0.05); /* Subtle inner glow */
        }

        .form-group input[type="text"]:focus,
        .form-group input[type="email"]:focus,
        .form-group textarea:focus {
            border-color: #666;
            outline: none;
            box-shadow: 0 0 12px rgba(255, 255, 255, 0.2), inset 0 0 5px rgba(255, 255, 255, 0.1);
            transform: scale(1.005); /* Subtle scale for 3D feel */
        }

        .form-group textarea {
            min-height: 120px;
            resize: vertical;
        }

        .submit-button {
            padding: 15px 25px;
            background-color: #333;
            color: #FFF;
            border: 2px solid #555;
            border-radius: 5px;
            font-size: 1.1em;
            cursor: pointer;
            transition: background-color 0.3s ease, border-color 0.3s ease, transform 0.3s;
            align-self: flex-end; /* Align to right */
        }

        .submit-button:hover {
            background-color: #555;
            border-color: #777;
            transform: translateY(-3px); /* Subtle lift */
        }

        .submit-button:disabled {
            background-color: #222;
            border-color: #444;
            cursor: not-allowed;
            opacity: 0.7;
        }

        .status-message {
            margin-top: 20px;
            padding: 15px;
            border-radius: 5px;
            text-align: center;
            font-weight: bold;
            font-size: 1.1em;
        }

        .status-message.success {
            background-color: #28a745;
            color: #FFF;
        }

        .status-message.error {
            background-color: #dc3545;
            color: #FFF;
        }

        @media (max-width: 768px) {
            .page-title {
                font-size: 2.5em;
            }
            .contact-container {
                padding: 20px;
            }
            .contact-form {
                padding: 20px;
            }
        }
        "#
    }.expect("Failed to load contact page style");

    html! {
        <div class={contact_style}>
            <div class="contact-container">
                <h1 class="page-title">{"Contact Us"}</h1>
                <form class="contact-form" onsubmit={on_submit}>
                    <div class="form-group">
                        <label for="name">{"Name"}</label>
                        <input type="text" id="name" value={(*name_state).clone()} onchange={on_name_change} required=true />
                    </div>
                    <div class="form-group">
                        <label for="email">{"Email"}</label>
                        <input type="email" id="email" value={(*email_state).clone()} onchange={on_email_change} required=true />
                    </div>
                    <div class="form-group">
                        <label for="subject">{"Subject (Optional)"}</label>
                        <input type="text" id="subject" value={(*subject_state).clone()} onchange={on_subject_change} />
                    </div>
                    <div class="form-group">
                        <label for="message">{"Message"}</label>
                        <textarea id="message" value={(*message_state).clone()} onchange={on_message_change} required=true rows="6"></textarea>
                    </div>
                    <button type="submit" class="submit-button" disabled={*is_submitting_state}>
                        {if *is_submitting_state { "Sending..." } else { "Send Message" }}
                    </button>
                    {if let Some(status) = &*submission_status_state {
                        let class = if status.starts_with("success") { "status-message success" } else { "status-message error" };
                        html! { <p class={class}>{status.clone()}</p> }
                    } else { html! {} }}
                </form>
            </div>
        </div>
    }
}
