// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Bidashboard {}

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

impl Component for Bidashboard {
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
                data-icon={ "BIDashboard" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M512 896h384v1152H512V896zm128 1024h128v-896H640v896zm384-768h384v896h-384v-896zm128 768h128v-640h-128v640zM0 1408h384v640H0v-640zm128 512h128v-384H128v384zM1536 640h384v1408h-384V640zm128 1280h128V768h-128v1152zM1389 621q19 41 19 83 0 40-15 75t-41 61-61 41-75 15q-40 0-75-15t-61-41-41-61-15-75v-12q0-6 1-12l-188-94q-26 26-61 40t-72 14q-42 0-83-19L365 877q19 41 19 83 0 40-15 75t-41 61-61 41-75 15q-40 0-75-15t-61-41-41-61-15-75q0-40 15-75t41-61 61-41 75-15q42 0 83 19l256-256q-19-41-19-83 0-40 15-75t41-61 61-41 75-15q40 0 75 15t61 41 41 61 15 75v12q0 6-1 12l188 94q26-26 61-40t72-14q42 0 83 19l256-256q-19-41-19-83 0-40 15-75t41-61 61-41 75-15q40 0 75 15t61 41 41 61 15 75q0 40-15 75t-41 61-61 41-75 15q-42 0-83-19l-256 256zM192 1024q26 0 45-19t19-45q0-26-19-45t-45-19q-26 0-45 19t-19 45q0 26 19 45t45 19zm1536-896q-26 0-45 19t-19 45q0 26 19 45t45 19q26 0 45-19t19-45q0-26-19-45t-45-19zM704 512q26 0 45-19t19-45q0-26-19-45t-45-19q-26 0-45 19t-19 45q0 26 19 45t45 19zm512 256q26 0 45-19t19-45q0-26-19-45t-45-19q-26 0-45 19t-19 45q0 26 19 45t45 19z" />
            </svg>
        }
    }
}
