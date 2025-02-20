// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Telemarketer {}

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

impl Component for Telemarketer {
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
                data-icon={ "Telemarketer" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1920 2048h-128q0-106-27-204t-78-183-120-156-155-120-184-77-204-28q-106 0-204 27t-183 78-156 120-120 155-77 184-28 204H128q0-145 42-276t121-240 187-193 244-135q-124-67-210-180h-64q-40 0-75-15t-61-41-41-61-15-75V576q0-38 14-72t38-60 58-42 71-18q39-88 99-159t137-121 166-77 185-27q96 0 185 27t165 77 137 121 100 159q38 2 71 18t57 42 39 60 14 72v256q0 40-15 75t-41 61-61 41-75 15q-30 0-57-9-43 59-97 107t-120 82q134 51 243 134t188 193 120 241 43 276zM1664 576q0-26-19-45t-45-19q-26 0-45 19t-19 45v256q0 26 19 45t45 19q26 0 45-19t19-45V576zM384 832q0 26 19 45t45 19h64V576q0-26-19-45t-45-19q-26 0-45 19t-19 45v256zm301 192q70 62 157 95t182 33q62 0 121-14t112-42 100-67 83-91q-16-23-24-50t-8-56V576q0-46 20-87t59-68q-32-67-80-121t-109-92-130-59-144-21q-74 0-144 20t-130 59-108 93-81 121q38 27 58 68t21 87v320h128q0-27 10-50t27-40 41-28 50-10q27 0 50 10t40 27 28 41 10 50q0 27-10 50t-27 40-41 28-50 10H685z" />
            </svg>
        }
    }
}
