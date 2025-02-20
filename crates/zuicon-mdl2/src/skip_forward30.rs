// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct SkipForward30 {}

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

impl Component for SkipForward30 {
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
                data-icon={ "SkipForward30" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M704 1480q44 4 82 20t68 45 45 65 17 84q0 67-26 117t-70 84-101 51-118 17q-53 0-106-10t-98-39v-110q90 72 207 72 40 0 77-10t66-32 46-53 18-78q0-46-15-76t-40-51-58-30-69-14-73-4-70-1v-86q47 0 98-2t95-17 71-49 28-98q0-82-46-117t-123-35q-50 0-94 17t-84 47v-100q46-28 98-39t106-12q49 0 93 13t79 41 55 67 21 93q0 92-45 147t-134 80v3zm662-444q62 0 106 20t76 56 51 81 30 98 14 103 4 101q0 49-5 102t-17 106-35 99-55 83-79 57-109 21q-59 0-103-20t-75-55-52-79-32-94-16-101-5-97q0-50 4-105t17-109 33-101 55-86 81-58 112-22zm-9 840q43 0 74-19t51-50 33-70 18-81 8-80 2-71q0-30-1-70t-8-82-17-84-32-73-51-53-73-20q-45 0-76 20t-53 52-34 74-19 85-8 85-2 73q0 31 2 70t8 80 18 79 33 69 52 48 75 18zm691-1492v512h-512V768h316q-59-117-146-211t-196-161-232-103-254-37q-155 0-296 47T471 437 268 645 136 915L14 877q33-111 88-209t130-182 163-150 191-112 211-71 227-25q136 0 266 34t247 99 213 157 170 209V384h128z" />
            </svg>
        }
    }
}
