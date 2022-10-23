use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MessageProp {
    pub text: String,
    pub css_class: String,
}

#[function_component(Message)]
pub fn message(MessageProp { text, css_class }: &MessageProp) -> Html {
    html!(
      <p class={css_class.clone()}>
        {text.clone()}
      </p>
    )
}
