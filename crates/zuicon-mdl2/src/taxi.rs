// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Taxi {}

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

impl Component for Taxi {
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
                data-icon={ "Taxi" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M384 1152q27 0 50 10t40 27 28 41 10 50q0 27-10 50t-27 40-41 28-50 10q-27 0-50-10t-40-27-28-41-10-50q0-27 10-50t27-40 41-28 50-10zm1280 0q27 0 50 10t40 27 28 41 10 50q0 27-10 50t-27 40-41 28-50 10q-27 0-50-10t-40-27-28-41-10-50q0-27 10-50t27-40 41-28 50-10zm384-256h-37l-66 65q2 5 10 30t19 59 25 73 24 71 18 54 7 22v650q0 27-10 50t-27 40-41 28-50 10h-128q-27 0-50-10t-40-27-28-41-10-50H384q0 27-10 50t-27 40-41 28-50 10H128q-27 0-50-10t-40-27-28-41-10-50v-650l7-21 18-54 24-72q13-39 24-73t20-59 10-30l-66-65H0V768h91l57 58 74-223q17-52 48-92t72-69 91-43 103-15l111-221q8-16 24-25t33-10h640q17 0 33 9t24 26l111 221q54 0 103 15t90 43 73 68 48 93l74 223 57-58h91v128zM744 256l-64 128h688l-64-128H744zM259 896h1530l-85-253q-20-59-69-95t-113-36H526q-63 0-112 36t-70 95l-85 253zm1149 896v-49l-104-207H744l-104 207v49h768zm128 0h384v-502l-6-18q-6-18-15-47t-21-61-21-63-17-51-9-26H217q-2 5-9 26t-17 50-21 63-20 62-16 46-6 19v502h384v-79l152-305h720l152 305v79z" />
            </svg>
        }
    }
}
