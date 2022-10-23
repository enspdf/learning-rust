use yew::prelude::*;

mod components;
mod models;

use gloo_net::{http::Request, Error};
use models::user::Users;

use components::{card::Card, header::Header, loader::Loader, message::Message};

#[function_component(App)]
fn app() -> Html {
    let users: UseStateHandle<Option<Users>> = use_state(|| None);
    let error: UseStateHandle<Option<Error>> = use_state(|| None);

    {
        //create copies of states
        let users = users.clone();
        let error = error.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_users = Request::get("https://dummyjson.com/users").send().await;
                    match fetched_users {
                        Ok(response) => {
                            let json = response.json::<Users>().await;
                            match json {
                                Ok(json_resp) => {
                                    users.set(Some(json_resp));
                                }
                                Err(e) => error.set(Some(e)),
                            }
                        }
                        Err(e) => error.set(Some(e)),
                    }
                });
                || ()
            },
            (),
        );
    }

    let user_list_logic = match users.as_ref() {
        Some(users) => users
            .users
            .iter()
            .map(|user| {
                html! {
                  <Card user={user.clone() }/>
                }
            })
            .collect(),
        None => match error.as_ref() {
            Some(_) => {
                html! {
                    <Message text={"Error getting list of users"} css_class={"text-danger"}/>
                }
            }
            None => {
                html! {
                  <Loader />
                }
            }
        },
    };

    html! {
      <>
        <Header />
        {user_list_logic}
      </>
    }
}

fn main() {
    yew::start_app::<App>();
}
