// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct AmazonWebServicesLogo {}

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

impl Component for AmazonWebServicesLogo {
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
                data-icon={ "AmazonWebServicesLogo" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1504 912q0-8 3-15t13-7q5 0 14 3t15 6q39 15 76 23t80 9q21 0 45-3t44-13 33-27 13-44q0-27-16-44t-43-28-58-20-66-20-62-27-50-44q-17-23-25-46t-9-53q0-45 20-78t52-56 72-33 80-11q16 0 42 3t52 9 50 15 33 23q8 12 8 31v12q0 6-1 12 0 8-3 15t-13 7q-7 0-13-3t-13-6q-30-14-64-20t-67-6q-20 0-41 3t-40 11-29 25-12 41q0 33 24 51t61 31 79 24 79 32 61 52 25 87q0 50-21 86t-55 59-78 35-87 11q-16 0-47-3t-63-11-60-18-34-27q-3-6-4-13t-1-15v-13q0-6 1-12zM641 419q0-17 16-17h67q20 0 27 7t13 26l115 451 106-451q5-22 18-28t30-7q11 0 23 1t24 1q19 0 27 7t13 26l107 456 118-456q5-17 13-25t27-8h63q17 0 17 17 0 8-2 14t-4 14l-164 526q-7 23-20 29t-30 6q-11 0-23-1t-26-1q-20 0-27-7t-13-27l-105-439-105 439q-5 23-18 30t-31 7q-11 0-23-1t-27-2q-20 0-27-8t-13-26L647 446q-2-6-4-13t-2-14zM468 669v-50q0-39-8-66t-25-43-44-24-66-7q-42 0-75 8t-72 24q-7 2-17 6t-18 5q-10 0-12-10t-3-22v-13q0-6 1-10v-12q0-7 4-12 9-13 37-24t61-18 65-12 48-4q115 0 175 52t60 171v221h2q0 14 1 30t6 30q2 5 6 14t9 20 8 19 4 12q0 6-9 15t-22 19-25 15-18 6q-10 0-20-10t-19-23-16-27-11-22q-79 94-200 94-41 0-75-12t-60-35-38-56-14-76q0-49 19-85t50-61 73-36 87-12q38 0 75 6t76 15zm-3 105v-29q-31-8-62-12t-64-4q-27 0-51 5t-43 17-30 32-11 52q0 47 24 72t72 25q24 0 49-6t47-18 39-30 24-45q4-14 5-29t1-30zm1371 526q12 0 22 7t10 21q0 8-5 15t-11 12q-79 63-181 110t-214 80-223 48-212 16q-138 0-275-25t-268-74-249-121T9 1222q-9-9-9-18 0-7 5-11t12-5q7 0 15 5 237 131 489 200t525 69q201 0 394-41t379-117q5-2 8-3t9-1zm212-79q0 38-10 81t-28 86-43 79-54 61q-4 3-8 5t-10 3q-11 0-11-12 0-8 10-35t24-63 23-73 11-67q0-19-8-29t-22-16-29-6-31-2h-16q-42 0-82 4t-82 9h-4q-5 0-9-2t-5-9q0-6 4-11t9-9q22-17 50-29t59-19 62-11 58-4q9 0 29 1t41 4 40 8 24 15q5 8 6 19t2 22z" />
            </svg>
        }
    }
}
