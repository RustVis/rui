// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct AutoFitWindow {}

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

impl Component for AutoFitWindow {
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
                data-icon={ "AutoFitWindow" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M0 2048V896h2048v1152h-128v-128h-512v128h-128v-128H768v128H640v-128H128v128H0zm1280-256v-384H768v384h512zm640 0v-384h-512v384h512zM128 1024v256h1792v-256H128zm0 384v384h512v-384H128zm0-896v256H0V128h128v256h256v128H128zm1792-384h128v640h-128V512h-384V384h384V128zM797 768H669L1120 0h128L797 768zm-93-384q-40 0-75-15t-61-41-41-61-15-75q0-40 15-75t41-61 61-41 75-15q40 0 75 15t61 41 41 61 15 75q0 40-15 75t-41 61-61 41-75 15zm0-288q-40 0-68 28t-28 68q0 40 28 68t68 28q40 0 68-28t28-68q0-40-28-68t-68-28zm320 480q0-40 15-75t41-61 61-41 75-15q40 0 75 15t61 41 41 61 15 75q0 40-15 75t-41 61-61 41-75 15q-40 0-75-15t-61-41-41-61-15-75zm288 0q0-40-28-68t-68-28q-40 0-68 28t-28 68q0 40 28 68t68 28q40 0 68-28t28-68z" />
            </svg>
        }
    }
}
