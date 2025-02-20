// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ParkingLocation {}

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

impl Component for ParkingLocation {
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
                data-icon={ "ParkingLocation" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2019 1459q29 64 29 133v72q0 76-41 139t-110 94q-31 69-94 110t-139 41q-69 0-128-34t-94-94H734q-35 60-94 94t-128 34q-69 0-128-34t-94-94H128q-27 0-50-10t-40-27-28-41-10-50v-256q0-80 30-150t82-122 122-82 150-30h37l328-328q27-27 62-41t74-15h907v128h-29l256 563zm-739-563v256h459l-99-219q-8-17-24-27t-35-10h-301zm-395 0q-26 0-45 19l-237 237h549V896H885zM512 1920q27 0 50-10t40-27 28-41 10-50q0-27-10-50t-27-40-41-28-50-10q-27 0-50 10t-40 27-28 41-10 50q0 27 10 50t27 40 41 28 50 10zm0-384q53 0 99 20t82 55 55 81 20 100h384v-512H384q-53 0-99 20t-82 55-55 81-20 100v256h128q0-53 20-99t55-82 81-55 100-20zm1152 384q27 0 50-10t40-27 28-41 10-50q0-27-10-50t-27-40-41-28-50-10q-27 0-50 10t-40 27-28 41-10 50q0 27 10 50t27 40 41 28 50 10zm256-328q0-41-17-80l-106-232h-517v512h128q0-53 20-99t55-82 81-55 100-20q42 0 81 13t71 37 56 57 37 74q11-27 11-53v-72z" />
            </svg>
        }
    }
}
