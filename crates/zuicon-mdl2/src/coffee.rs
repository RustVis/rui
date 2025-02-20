// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Coffee {}

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

impl Component for Coffee {
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
                data-icon={ "Coffee" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M448 1024q56 0 101 25t82 68 61 96 43 111 25 112 8 100q0 45-8 100t-25 112-42 111-62 96-81 67-102 26q-35 0-64-17-29 17-64 17-56 0-101-25t-82-68-61-96-43-111-25-112-8-100q0-45 8-100t25-112 42-111 62-96 81-67 102-26q35 0 64 17 29-17 64-17zm-321 512q0 45 7 91t21 90 36 86 51 76q16 19 34 30t44 11v-768q-25 0-43 11t-35 30q-29 35-51 76t-36 85-21 91-7 91zm321 384q25 0 43-11t35-30q29-34 50-75t36-86 22-91 7-91q0-45-7-91t-21-90-36-86-51-76q-16-19-34-30t-44-11v768zm1600-320q0 35-17 64 17 29 17 64 0 56-25 101t-68 82-96 61-111 43-112 25-100 8q-45 0-100-8t-112-25-111-42-96-62-67-81-26-102q0-35 17-64-17-29-17-64 0-56 25-101t68-82 96-61 111-43 112-25 100-8q45 0 100 8t112 25 111 42 96 62 67 81 26 102zm-512-193q-45 0-91 7t-90 21-86 36-76 51q-19 16-30 34t-11 44h768q0-25-11-43t-30-35q-35-29-76-50t-85-36-91-22-91-7zm0 514q45 0 91-7t90-21 86-36 76-51q19-16 30-34t11-44h-768q0 25 11 43t30 35q34 29 75 51t86 36 91 21 91 7zm-384-914q-29 17-64 17-56 0-101-25t-82-68-61-96-43-111-25-112-8-100q0-45 8-100t25-112 42-111 62-96 81-67 102-26q35 0 64 17 29-17 64-17 56 0 101 25t82 68 61 96 43 111 25 112 8 100q0 45-8 100t-25 112-42 111-62 96-81 67-102 26q-35 0-64-17zm257-495q0-45-7-91t-21-90-36-86-51-76q-16-19-34-30t-44-11v768q25 0 43-11t35-30q29-34 50-75t36-86 22-91 7-91zm-514 0q0 45 7 91t21 90 36 86 51 76q16 19 34 30t44 11V128q-25 0-43 11t-35 30q-29 35-51 76t-36 85-21 91-7 91z" />
            </svg>
        }
    }
}
