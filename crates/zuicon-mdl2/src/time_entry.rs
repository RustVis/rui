// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct TimeEntry {}

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

impl Component for TimeEntry {
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
                data-icon={ "TimeEntry" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1536 1408h192v128h-320v-384h128v256zm-256-896H256V384h1024v128zm192 384q119 0 224 45t183 124 123 183 46 224q0 119-45 224t-124 183-183 123-224 46q-119 0-224-45t-183-124-123-183-46-224q0-119 45-224t124-183 183-123 224-46zm0 1024q93 0 174-35t142-96 96-142 36-175q0-93-35-174t-96-142-142-96-175-36q-93 0-174 35t-142 96-96 142-36 175q0 93 35 174t96 142 142 96 175 36zM1166 768q-109 48-200 128H256V768h910zm-391 384q-14 31-25 63t-21 65H256v-128h519zm-519 384h451q3 32 8 64t14 64H256v-128zm594 384q50 71 116 128H0V0h1536v707l-32-2q-16-1-32-1t-32 1-32 2V128H128v1792h722z" />
            </svg>
        }
    }
}
