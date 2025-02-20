// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Transition {}

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

impl Component for Transition {
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
                data-icon={ "Transition" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M768 1472q0-66-25-124t-69-101-102-69-124-26q-66 0-124 25t-102 69-69 102-25 124v64H0v-64q0-63 16-121t48-110 76-94 100-72q-54-46-83-109t-29-134q0-66 25-124t68-101 102-69 125-26q66 0 124 25t101 69 69 102 26 124q0 70-29 133t-83 110q55 29 99 71t76 94 48 110 17 122v64H768v-64zM256 832q0 40 15 75t41 61 61 41 75 15q40 0 75-15t61-41 41-61 15-75q0-40-15-75t-41-61-61-41-75-15q-40 0-75 15t-61 41-41 61-15 75zm1552 243q55 29 99 71t76 94 48 110 17 122v64h-128v-64q0-66-25-124t-69-101-102-69-124-26q-66 0-124 25t-102 69-69 102-25 124v64h-128v-64q0-63 16-121t48-110 76-94 100-72q-54-46-83-109t-29-134q0-66 25-124t68-101 102-69 125-26q66 0 124 25t101 69 69 102 26 124q0 70-29 133t-83 110zm-400-243q0 40 15 75t41 61 61 41 75 15q40 0 75-15t61-41 41-61 15-75q0-40-15-75t-41-61-61-41-75-15q-40 0-75 15t-61 41-41 61-15 75zm-256-704q-81 0-159 17t-151 49-138 78-120 107l-95-86q64-70 140-124t160-92 176-57 187-20q137 0 268 40t244 121V0h128v384h-384V256h169q-95-63-203-95t-222-33zM896 1920q81 0 159-17t151-49 138-78 120-107l95 86q-64 70-140 124t-161 92-176 57-186 20q-137 0-268-40t-244-121v161H256v-384h384v128H471q95 63 203 95t222 33z" />
            </svg>
        }
    }
}
