// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct XboxController {}

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

impl Component for XboxController {
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
                data-icon={ "XboxController" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1728 480q17 14 31 31t26 38q35 68 64 139t59 142q37 92 70 186t52 193q8 44 13 88t5 89q0 59-13 115t-45 107q-15 22-37 44t-48 39-56 28-57 11q-31 0-69-27t-77-64-71-76-54-62q-22-25-46-55t-52-49l-24-17q-20-14-42-21t-46-10-48-3-47-1H785q-24 0-47 4t-46 10-43 21l-24 17q-27 20-51 49t-47 55q-21 23-54 62t-71 76-76 64-70 27q-28 0-57-11t-55-28-49-39-37-44q-32-51-45-107T0 1386q0-45 5-89t13-88q8-46 23-100t36-114 43-120 48-119 49-110 46-96q11-20 25-37t32-33v-25q0-12 2-22t9-20 20-18q11-7 35-15t52-17 54-15 40-10q38-8 77-12t77-4q15 0 33 1t33 9h1q23 12 45 25t45 26h362q23-13 40-24t34-19 36-13 47-5q38 0 77 4t77 12q14 3 40 9t53 15 52 17 36 16q25 17 28 37t3 48zm69 1120q36-10 60-32t38-51 19-63 6-67q0-39-4-78t-12-78q-20-106-58-208t-80-202q-23-54-45-108t-50-106q-8-15-20-23t-23-18-20-20-8-32v-20q-57-20-115-32t-119-12h-11q-6 0-11 1-38 22-57 34t-37 18-38 7-61 1H897q-40 0-61-1t-38-7-37-18-57-34q-5-1-11-1t-11 0q-60 0-118 12t-116 32q0 28-5 44t-30 33q-11 6-20 15t-16 21q-28 51-50 105t-45 109q-41 99-79 201t-59 209q-8 38-12 77t-4 79q0 33 5 66t20 63 37 51 60 33q15-9 40-32t52-52 51-56 38-44q10-11 26-30t35-38 35-36 31-25q5-3 9-6t9-6q28-21 59-32t63-17 66-7 68-2h384q34 0 67 1t66 7 64 17 59 33q5 3 9 6t9 6q13 8 30 24t36 36 34 39 27 30q15 17 38 44t49 54 51 51 42 35zm-773-960q26 0 45 19t19 45q0 26-19 45t-45 19q-26 0-45-19t-19-45q0-26 19-45t45-19z" />
            </svg>
        }
    }
}
