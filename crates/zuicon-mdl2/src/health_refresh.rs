// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct HealthRefresh {}

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

impl Component for HealthRefresh {
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
                data-icon={ "HealthRefresh" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M129 671q0 47 11 97H9q-5-24-6-48t-2-48q0-113 42-212t116-173 173-116 212-43q108 0 208 41t177 118l95 96 95-96q76-77 176-118t209-41q112 0 211 42t172 116 116 172 43 211q0 98-34 189t-97 166h-470l-101-102-101 102h-182l283-282 155 154h355q32-51 49-108t17-117q0-87-32-162t-89-130-132-87-163-32q-82 0-158 30t-136 88l-186 186-186-186q-60-60-134-91t-160-31q-86 0-162 32t-132 89-88 132-33 162zm179 353l701 701q8 41 21 80t34 76l-40 40-865-864q-35-35-62-75t-47-86h243l283-282 448 447v182L576 794l-229 230h-39zm1441 155h2-2zm299 422q0 93-35 174t-95 141-142 96-174 35q-93 0-174-35t-142-95-96-142-35-174q0-91 35-174t100-147h-137v-128h384v384h-127v-191q-60 46-94 112t-34 143q0 66 25 124t69 102 102 69 125 26q66 0 124-25t101-68 69-102 25-125q0-51-15-98t-44-87-67-70-88-47l43-120q67 24 121 66t94 97 60 122 22 137z" />
            </svg>
        }
    }
}
