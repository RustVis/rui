// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct SearchNearby {}

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

impl Component for SearchNearby {
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
                data-icon={ "SearchNearby" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1280 1431q38 5 90 15t110 27 113 40 100 54 71 71 28 90q0 50-27 89t-68 68-88 51-86 34q-64 22-134 37t-144 24-145 13-140 4q-68 0-140-4t-145-13-143-24-135-37q-39-13-86-34t-87-50-68-69-28-89q0-50 27-89t72-71 100-55 113-39 109-27 91-16v-162q-29-10-52-28t-41-42-26-52-9-59V576q0-40 15-75t41-61 61-41 75-15h34q-34-60-34-128 0-53 20-99t55-82 81-55T960 0q53 0 99 20t82 55 55 81 20 100q0 68-34 128h34q40 0 75 15t61 41 41 61 15 75v512q0 30-9 58t-26 53-40 42-53 28v162zM832 256q0 27 10 50t27 40 41 28 50 10q27 0 50-10t40-27 28-41 10-50q0-27-10-50t-27-40-41-28-50-10q-27 0-50 10t-40 27-28 41-10 50zm128 1664q43 0 100-2t119-9 128-17 125-27 110-37 86-51q10-8 23-21t13-28q0-14-13-28t-30-26-36-22-28-15q-64-30-135-48t-142-29v168q0 12-17 21t-39 15-43 10-29 5v-627h40q22 0 42-4t33-18 13-42V576q0-26-19-45t-45-19H704q-26 0-45 19t-19 45v512q0 28 13 41t32 19 42 5 41-1v627q-8-1-29-5t-43-10-39-15-17-21v-168q-70 11-141 29t-136 48l-28 14q-18 9-35 22t-31 27-13 28q0 14 13 27t23 22q34 29 85 50t111 38 125 26 127 17 120 9 100 3zm-64-768h128v639q-16 1-32 1t-32 0h-32q-16 0-32-1v-639z" />
            </svg>
        }
    }
}
