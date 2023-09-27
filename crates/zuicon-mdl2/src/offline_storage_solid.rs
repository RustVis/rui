// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct OfflineStorageSolid {}

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

impl Component for OfflineStorageSolid {
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
                data-icon={ "OfflineStorageSolid" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M960 768q-69 0-137-6t-137-19q-34-7-79-18t-93-30-94-42-83-54-58-68-23-83q0-45 22-82t59-68 82-55 94-42 93-29 80-19q68-13 136-19t138-6q69 0 137 6t137 19q34 7 79 18t93 30 94 42 83 54 58 68 23 83q0 45-22 82t-59 68-82 55-94 42-93 30-80 18q-68 13-136 19t-138 6zm0 128q45 0 100-3t115-10 120-18 117-29 106-40 86-53q13-11 21-21t20-23q2-2 9-6t10-7v146q0 46-23 83t-60 69-85 55-96 42-95 30-81 18q-66 12-132 17t-132 6q-66 0-132-5t-132-18q-34-6-80-18t-95-30-96-42-85-55-61-68-23-84V686q3 2 10 6t9 7q11 12 19 22t22 22q36 30 85 53t106 40 117 28 120 19 115 10 101 3zm0 384q45 0 100-3t115-10 120-18 117-29 106-40 86-53q13-11 21-21t20-23q2-2 9-6t10-7v146q0 46-23 83t-60 69-85 55-96 42-95 30-81 18q-66 12-132 17t-132 6q-66 0-132-5t-132-18q-34-7-80-18t-95-30-96-42-85-55-61-68-23-84v-146q3 2 10 6t9 7q11 12 19 22t22 22q36 30 85 53t106 40 117 28 120 19 115 10 101 3zm0 384q45 0 100-3t115-10 120-18 117-29 106-40 86-53q13-11 21-21t20-23q2-2 9-6t10-7v146q0 46-23 83t-60 69-85 55-96 42-95 30-81 18q-66 12-132 17t-132 6q-66 0-132-5t-132-18q-34-6-80-18t-95-30-96-42-85-55-61-68-23-84v-146q3 2 10 6t9 7q11 12 19 22t22 22q36 30 85 53t106 40 117 28 120 19 115 10 101 3z" />
            </svg>
        }
    }
}
