// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Vignette {}

#[derive(Properties, Debug, Clone, PartialEq, Eq)]
pub struct Props {
    #[prop_or_default]
    pub class: Option<&'static str>,

    #[prop_or_default]
    pub width: Option<&'static str>,

    #[prop_or_default]
    pub height: Option<&'static str>,

    #[prop_or_default]
    pub color: Option<&'static str>,

    #[prop_or_default]
    pub fill: Option<&'static str>,

    #[prop_or_default]
    pub spin: bool,

    #[prop_or_default]
    pub rotate: i16,
}

impl Component for Vignette {
    type Properties = Props;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        // TODO(Shaohua): Do not generate style attribute if it is empty.
        let mut style = String::new();
        if props.rotate != 0 {
            style += &format!("transform: rotate({}deg);", props.rotate);
        }
        html! {
            <svg
                xmlns={ "http://www.w3.org/2000/svg" }
                class={ props.class.unwrap_or("") }
                width={ props.width.unwrap_or("16") }
                height={ props.height.unwrap_or("16") }
                focusable={ "false" }
                data-icon={ "vignette" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M8 1a7 7 0 1 0 0 14A7 7 0 0 0 8 1ZM0 8a8 8 0 1 1 16 0A8 8 0 0 1 0 8Z"/>
  <path d="M8.5 4.5a.5.5 0 1 1-1 0 .5.5 0 0 1 1 0Zm0 7a.5.5 0 1 1-1 0 .5.5 0 0 1 1 0Zm1.683-6.281a.5.5 0 1 1-.866-.5.5.5 0 0 1 .866.5Zm-3.5 6.062a.5.5 0 1 1-.866-.5.5.5 0 0 1 .866.5Zm4.598-4.598a.5.5 0 1 1-.5-.866.5.5 0 0 1 .5.866Zm-6.062 3.5a.5.5 0 1 1-.5-.866.5.5 0 0 1 .5.866ZM11.5 8.5a.5.5 0 1 1 0-1 .5.5 0 0 1 0 1Zm-7 0a.5.5 0 1 1 0-1 .5.5 0 0 1 0 1Zm6.281 1.683a.5.5 0 1 1 .5-.866.5.5 0 0 1-.5.866Zm-6.062-3.5a.5.5 0 1 1 .5-.866.5.5 0 0 1-.5.866Zm4.598 4.598a.5.5 0 1 1 .866-.5.5.5 0 0 1-.866.5Zm-3.5-6.062a.5.5 0 1 1 .866-.5.5.5 0 0 1-.866.5Z"/>
            </svg>
        }
    }
}
