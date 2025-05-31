// frontend/src/contexts/user_context.rs
use yew::prelude::*;
use std::rc::Rc;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
}

#[derive(Clone, PartialEq)]
pub struct UserContext {
    user: Option<User>,
    dispatch: Callback<UserAction>,
}

pub enum UserAction {
    Login(User),
    Logout,
}

impl Reducer for UserContext {
    type Action = UserAction;
    type State = UserContext;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_user = match action {
            UserAction::Login(user) => Some(user),
            UserAction::Logout => {
                // Clear session storage on logout
                if let Some(storage) = web_sys::window().and_then(|w| w.session_storage().ok().flatten()) {
                    let _ = storage.remove_item("user_session");
                }
                None
            },
        };

        // Persist login state to session storage
        if let Some(storage) = web_sys::window().and_then(|w| w.session_storage().ok().flatten()) {
            if let Some(user_data) = &next_user {
                if let Ok(serialized_user) = serde_json::to_string(user_data) {
                    let _ = storage.set_item("user_session", &serialized_user);
                }
            }
        }

        UserContext {
            user: next_user,
            dispatch: self.dispatch.clone(),
        }.into()
    }
}

impl UserContext {
    pub fn is_authenticated(&self) -> bool {
        self.user.is_some()
    }

    pub fn login(&self, user: User) {
        self.dispatch.emit(UserAction::Login(user));
    }

    pub fn logout(&self) {
        self.dispatch.emit(UserAction::Logout);
    }

    pub fn get_user(&self) -> Option<&User> {
        self.user.as_ref()
    }
}

#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &ChildrenProps) -> Html {
    let user_state = {
        // Initialize from session storage on load
        let initial_user = if let Some(storage) = web_sys::window().and_then(|w| w.session_storage().ok().flatten()) {
            if let Ok(Some(user_json)) = storage.get_item("user_session") {
                serde_json::from_str(&user_json).ok()
            } else {
                None
            }
        } else {
            None
        };
        use_reducer_with_init(
            |initial_state| UserContext {
                user: initial_state,
                dispatch: Callback::noop(), // Will be overwritten by use_reducer
            },
            initial_user,
        )
    };

    let user_ctx = Rc::new(UserContext {
        user: user_state.user.clone(),
        dispatch: user_state.dispatch.clone(),
    });

    html! {
        <ContextProvider<UserContext> context={user_ctx}>
            {props.children.clone()}
        </ContextProvider<UserContext>>
    }
}

