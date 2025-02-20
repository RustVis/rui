// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct GiftboxOpen {}

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

impl Component for GiftboxOpen {
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
                data-icon={ "GiftboxOpen" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M192 480q-23-54-64-95t-96-65q54-23 95-64t65-96q23 54 64 95t96 65q-54 23-95 64t-65 96zm1664 64q23 54 64 95t96 65q-54 23-95 64t-65 96q-23-54-64-95t-96-65q54-23 95-64t65-96zm-384 96q-47-109-129-191t-191-129q109-47 191-129T1472 0q47 109 129 191t191 129q-109 47-191 129t-129 191zm0-446q-56 71-126 126 70 56 126 126 55-70 126-126-71-55-126-126zM747 785q-47 0-88-19t-73-53-48-76-18-89q0-59 24-110t66-90 94-61 112-23q84 0 149 35t111 94 68 132 24 151q0 105-34 196t-93 167-141 132-174 97q-9 4-23 4-23 0-39-16t-16-40q0-36 34-52 76-36 140-77t113-95 79-122 40-158q1-10 1-19t1-20q0-52-15-105t-45-96-75-69-105-27q-34 0-67 13t-59 37-42 54-16 68q0 23 9 45t24 40 37 29 46 11q24 0 38-7t23-20 14-25 12-25 17-19 29-8q23 0 40 17t17 40q0 5-2 13-8 32-26 59t-44 46-55 30-64 11zm880 111l274 275-90 90-147-146v933H256v-933l-147 146-90-90 274-275h555q-35 41-77 72t-91 56H384v896h1152v-896h-331q17-31 31-63t25-65h366z" />
            </svg>
        }
    }
}
