// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct BeerMug {}

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

impl Component for BeerMug {
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
                data-icon={ "BeerMug" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1600 512q66 0 124 25t101 69 69 102 26 124v512q0 66-25 124t-69 102-102 69-124 25h-64v192q0 40-15 75t-41 61-61 41-75 15H320q-40 0-75-15t-61-41-41-61-15-75V477q-59-34-93-93T0 256q0-52 20-99t54-81 81-55 99-21h1154q53 0 99 20t82 55 55 81 20 100q0 35-9 69t-26 64-41 56-52 43v24h64zM255 128q-26 0-49 10t-41 27-27 41-10 50q0 40 23 73t62 47l43 15v569q0 26 19 45t45 19q26 0 45-19t19-45V576q0-40 15-75t41-61 61-41 75-15q40 0 75 15t61 41 41 61 15 75v256q0 26 19 45t45 19q26 0 45-19t19-45V576q0-40 15-75t41-61 61-41 75-15h371q33-16 55-52t22-76q0-27-10-50t-27-40-41-28-50-10H255zm1153 1728v-76q-15 5-31 8t-33 4H320q-17 0-33-3t-31-9v76q0 26 19 45t45 19h1024q26 0 45-19t19-45zm-64-192q26 0 45-19t19-45V512h-320q-26 0-45 19t-19 45v256q0 40-15 75t-41 61-61 41-75 15q-40 0-75-15t-61-41-41-61-15-75V576q0-26-19-45t-45-19q-26 0-45 19t-19 45v384q0 40-15 75t-41 61-61 41-75 15q-33 0-64-11v459q0 26 19 45t45 19h192v-448q0-27 19-45t45-19q26 0 45 18t19 46v448h384v-576q0-27 19-45t45-19q26 0 45 18t19 46v576h192zm448-320V832q0-40-15-75t-41-61-61-41-75-15h-64v896h64q40 0 75-15t61-41 41-61 15-75z" />
            </svg>
        }
    }
}
