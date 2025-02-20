// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct SharedDatabase {}

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

impl Component for SharedDatabase {
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
                data-icon={ "SharedDatabase" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M960 768q-68 0-144-6t-153-22-149-41-130-62v963q0 20 13 35t27 28q30 26 71 45t88 33 99 24 101 16 96 8 86 3q60 0 116-5t116-16q-6 10-16 28t-19 39-16 40-7 31q-42 8-88 9t-90 2q-33 0-81-2t-104-10-116-18-118-29-110-42-91-55-62-73-23-91V448q0-68 38-118t101-85 139-59 156-35 150-18 120-5q49 0 120 4t149 18 156 35 140 58 100 86 39 119v672q-33 0-65 6t-63 16V637q-58 37-130 62t-148 40-153 22-145 7zM384 448q0 11 3 19t10 17q24 32 65 55t91 41 106 29 110 18 104 10 87 3q38 0 87-3t103-10 110-18 105-29 91-41 66-55q14-17 14-36 0-11-4-19t-10-17q-25-31-65-55t-91-41-105-28-110-19-103-10-88-3q-37 0-86 3t-104 10-111 18-106 29-91 41-65 55q-6 8-9 16t-4 20zm1464 1265q46 25 83 61t63 79 40 93 14 102h-128q0-53-20-100t-54-81-82-55-100-20q-53 0-99 20t-82 55-55 81-20 100h-128q0-52 14-101t40-93 63-80 83-61q-35-35-53-81t-19-96q0-53 20-99t55-82 81-55 100-20q53 0 99 20t82 55 55 81 20 100q0 50-18 96t-54 81zm-184-49q26 0 49-10t41-27 28-41 10-50q0-26-10-49t-27-41-41-28-50-10q-27 0-50 10t-40 27-28 41-10 50q0 27 10 50t27 40 41 28 50 10z" />
            </svg>
        }
    }
}
