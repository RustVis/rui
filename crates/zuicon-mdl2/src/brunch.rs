// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Brunch {}

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

impl Component for Brunch {
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
                data-icon={ "Brunch" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1662 896q-4 72-15 140t-28 134-40 131-49 132q-9 23-14 51t-9 52h285v64q0 56-18 107t-52 92-80 71-100 42l22 136H878l16-128H320q-66 0-124-25t-102-68-69-102-25-125v-64h256q0-79 30-149t82-122 122-83 150-30q50 0 97 13t90 38q-27-97-43-195t-16-200q0-78 12-149t36-145h143L857 110l125-31 109 435h189q0-79 30-149t82-123 122-83 150-31q79 0 149 30t122 82 83 123 30 149q0 80-30 150t-83 122-123 82-150 30zm2-640q-53 0-100 20t-81 56-55 82-20 100h208q20 64 31 125t14 129q54 0 101-20t82-55 56-81 20-100q0-53-20-99t-55-82-81-55-100-20zM640 1280q-53 0-99 20t-82 55-55 81-20 100h512q0-53-20-99t-55-82-81-55-100-20zm286 384H139q10 29 28 52t42 41 52 26 59 9h591q4-32 9-64t6-64zm577 0q2 31 7 60t10 60q46-12 81-43t52-77h-150zm33-850q0-44-3-87t-14-85H913q-9 42-13 83t-4 84q0 58 8 127t22 139 35 136 45 121q29 63 40 128t11 135q0 82-11 163t-23 162h390q-12-75-26-149t-15-152q0-27 2-57t7-62 12-62 19-55q30-70 53-139t39-138 24-142 8-150z" />
            </svg>
        }
    }
}
