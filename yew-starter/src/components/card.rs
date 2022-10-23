use crate::models::user::User;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardProp {
    pub user: User,
}

#[function_component(Card)]
pub fn card(CardProp { user }: &CardProp) -> Html {
    html!(
      <div class="m-3 p-4 rounded d-flex align-items-center">
        <img src="https://robohash.org/hicveldicta.png?size=50x50&set=set1" class="mr-2" alt="img" />
        <div class="">
            <p class="fw-bold mb-1">{format!("{} {}", user.first_name.clone(), user.last_name.clone())}</p>
            <p class="fw-normal mb-1">{user.gender.clone()}</p>
            <p class="fw-normal mb-1">{user.email.clone()}</p>
            <p class="fw-normal mb-1">{user.phone.clone()}</p>
        </div>
      </div>
    )
}
