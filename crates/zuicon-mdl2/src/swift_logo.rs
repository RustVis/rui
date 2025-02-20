// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct SwiftLogo {}

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

impl Component for SwiftLogo {
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
                data-icon={ "SwiftLogo" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1522 127q82 0 155 31t127 86 85 127 32 155v994q0 83-31 155t-86 127-127 85-155 32H528q-83 0-155-31t-127-86-86-126-32-156V526q0-83 31-155t86-127 127-85 156-32h994zm125 1466q5 0 8-9t5-22 2-22 1-12q0-80-31-156t-82-138q11-38 16-78t6-81q0-106-34-208t-92-192-137-165-167-128q72 97 116 212t44 237q0 51-9 100t-27 98q-7-5-13-8t-14-8q-48-26-93-60t-90-68q-112-84-217-174T634 521q-5-4-8-10t-6-10-7-8-11-4q0 4 17 28t43 60 58 75 58 76 49 62 26 32q33 40 67 78t69 77q-63-35-121-77t-116-85q-81-60-159-123T439 560q17 28 36 53t39 50q64 80 130 158t135 154 143 145 156 133q-65 39-138 57t-148 19q-130 0-254-47t-229-123q57 92 137 170t176 136 201 89 217 33q72 0 141-15t133-49q38-20 75-34t81-14q63 0 102 30t70 83q2 5 5 5z" />
            </svg>
        }
    }
}
