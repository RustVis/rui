// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct DeviceBug {}

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

impl Component for DeviceBug {
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
                data-icon={ "DeviceBug" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1792 1627v74l237 238-90 90-174-173q-19 44-49 79t-68 60-83 39-93 14q-48 0-92-13t-83-39-69-61-49-79l-174 173-90-90 237-238v-48q0-39 2-73t10-68 26-65 47-64q-21-48-21-103 0-46 16-89t46-78l-107-108 90-90 124 124q42-15 87-15t87 15l124-124 90 90-107 108q30 35 46 78t16 89q0 55-21 103 37 41 58 89l174-173 90 90-237 238zm-192-347q0-27-10-50t-27-40-41-28-50-10q-27 0-50 10t-40 27-28 41-10 50q0 13 3 26 29-14 61-20t64-6q32 0 64 6t61 20q3-13 3-26zm80 448v-128q0-43-16-81t-45-66-66-44-81-17q-43 0-81 16t-66 45-44 66-17 81v128q0 43 16 81t45 66 66 44 81 17q43 0 81-16t66-45 44-66 17-81zm-656-64H640v-128h256v-128H0V256h1920v512h-128V384H128v896h896v384z" />
            </svg>
        }
    }
}
