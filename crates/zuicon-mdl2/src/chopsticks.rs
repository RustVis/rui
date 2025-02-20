// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Chopsticks {}

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

impl Component for Chopsticks {
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
                data-icon={ "Chopsticks" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1920 709q0 26-19 45t-45 19H768v379h832q26 0 45 19t19 45q0 115-29 221t-84 199-130 168-168 130-199 84-222 30q-115 0-221-29t-199-84-168-130-130-168-84-199-30-222q0-26 19-45t45-19h576V773H512v187q0 26-19 45t-45 19q-26 0-45-19t-19-45V773H192q-26 0-45-19t-19-45q0-26 19-45t45-19h192V489l-40 8q-25 5-52 10t-51 9-38 4q-26 0-45-19t-19-45q0-23 15-41t38-22l1659-292q4-1 11-1 26 0 45 19t19 45q0 23-15 41t-38 22L768 422v223h1088q26 0 45 19t19 45zM832 1920q136 0 258-49t216-135 155-203 72-253H131q12 136 72 253t154 203 217 135 258 49zM512 645h128V444l-128 23v178z" />
            </svg>
        }
    }
}
