// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct GitLogo {}

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

impl Component for GitLogo {
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
                data-icon={ "GitLogo" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1024 0q25 0 49 11t42 28l894 894q20 20 29 45t10 52q0 26-11 49t-28 40l-890 890q-39 39-95 39-25 0-49-11t-42-28L39 1115q-17-17-28-41t-11-50q0-26 10-51t29-44l613-613 232 232q-12 29-12 61 0 64 46 110l8 8q2 2 5 3t6 1 10 4v567q-12 5-20 14t-18 18q-26 26-41 55t-15 67q0 34 13 63t35 52 52 35 64 13q34 0 65-11t53-31 36-50 14-65q0-22-6-44t-18-41-28-36-35-27V763l212 212q-12 29-12 60 0 33 12 61t34 50 49 33 62 13q33 0 61-12t50-34 33-49 13-62q0-32-12-60t-34-50-49-34-61-13q-27 0-52 9l-227-227q9-25 9-52 0-33-12-61t-34-50-49-33-62-12q-24 0-50 8L743 224 929 39q39-39 95-39z" />
            </svg>
        }
    }
}
