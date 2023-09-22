// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct EmojiGrimaceFill {}

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

impl Component for EmojiGrimaceFill {
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
                data-icon={ "emoji-grimace-fill" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M16 8A8 8 0 1 1 0 8a8 8 0 0 1 16 0ZM7 6.25C7 5.56 6.552 5 6 5s-1 .56-1 1.25.448 1.25 1 1.25 1-.56 1-1.25Zm3 1.25c.552 0 1-.56 1-1.25S10.552 5 10 5s-1 .56-1 1.25.448 1.25 1 1.25Zm1.5 4.5a1.5 1.5 0 0 0 1.48-1.25v-.003a1.512 1.512 0 0 0 0-.497A1.5 1.5 0 0 0 11.5 9h-7a1.5 1.5 0 0 0-1.48 1.25v.003a1.51 1.51 0 0 0 0 .497A1.5 1.5 0 0 0 4.5 12h7Zm-7.969-1.25a1 1 0 0 0 .969.75h.25v-.75H3.531Zm8.938 0a1 1 0 0 1-.969.75h-.25v-.75h1.219ZM11.5 9.5a1 1 0 0 1 .969.75H11.25V9.5h.25Zm-7.969.75A1 1 0 0 1 4.5 9.5h.25v.75H3.531ZM5.25 11.5h1v-.75h-1v.75Zm2.5 0h-1v-.75h1v.75Zm1.5 0h-1v-.75h1v.75Zm1.5 0h-1v-.75h1v.75Zm-1-2h1v.75h-1V9.5Zm-1.5 0h1v.75h-1V9.5Zm-1.5 0h1v.75h-1V9.5Zm-1.5 0h1v.75h-1V9.5Z"/>
            </svg>
        }
    }
}
