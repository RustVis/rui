// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct SnowShowerNight {}

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

impl Component for SnowShowerNight {
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
                data-icon={ "SnowShowerNight" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1761 1036q63 17 115 52t91 85 60 109 21 126q0 70-24 133t-67 114-101 85-128 47v-131q42-11 77-34t61-57 40-73 14-84q0-55-20-102t-57-81-84-53-102-20q-12-82-51-152t-98-122-134-81-158-29q-77 0-146 25t-127 69-98 106-61 135q-44-38-97-58t-111-21q-66 0-124 25t-102 68-69 102-25 125q0 48 13 92t39 83 61 68 79 50v136q-71-21-130-63t-101-98-65-125-24-143q0-77 25-148t74-132q-81-55-136-134T14 754q58 14 114 14 106 0 199-40t163-109 110-163 40-200q0-56-14-114 87 20 160 67t126 113 82 147 30 171v17q0 9-2 17 47-17 95-25t99-9q93 0 178 29t158 81 126 125 83 161zM723 921q60-101 156-171 17-56 17-110 0-85-35-161T761 348q-15 105-62 197T581 708 417 827t-197 62q42 51 104 85 56-38 120-58t132-20q74 0 147 25zm307 912q32 19 32 55 0 27-19 45t-45 19q-12 0-26-5t-27-14-26-16-23-14q0 19 1 44t-3 47-18 38-44 16q-30 0-44-15t-18-38-4-48 2-44q-10 6-22 14t-26 16-28 13-26 6q-26 0-45-19t-20-45q0-18 9-32t24-23l70-41-70-41q-15-9-24-23t-9-32q0-26 19-45t45-19q13 0 27 5t27 14 26 16 23 14q0-19-1-44t2-47 19-38 44-16q30 0 43 15t18 38 4 48-1 44q10-6 22-14t26-16 28-13 27-6q26 0 44 19t19 45q0 36-32 55l-70 41 70 41zm481-329q26 0 44 19t19 45q0 36-32 55l-70 41 70 41q32 19 32 55 0 27-19 45t-45 19q-12 0-26-5t-27-14-26-16-23-14q0 19 1 44t-3 47-18 38-44 16q-30 0-44-15t-18-38-4-48 2-44q-10 6-22 14t-26 16-28 13-26 6q-26 0-45-19t-20-45q0-18 9-32t24-23l70-41-70-41q-15-9-24-23t-9-32q0-26 19-45t45-19q13 0 27 5t27 14 26 16 23 14q0-19-1-44t2-47 19-38 44-16q30 0 43 15t18 38 4 48-1 44q10-6 22-14t26-16 28-13 27-6z" />
            </svg>
        }
    }
}
