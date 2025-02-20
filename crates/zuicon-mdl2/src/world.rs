// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct World {}

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

impl Component for World {
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
                data-icon={ "World" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M496 883q13 0 29 4t32 11 32 13 29 12l-16 2q-8 1-17 1-17 0-31-5t-26-13-24-12-22-6q-10 0-18 4t-16 9q0-4-7-4 7-7 26-11t29-5zm135 45q41 0 75 14-14 5-28 8t-29 4q-20 0-36-4 5-8 10-10t8-12zM1024 0q141 0 271 37t244 103 208 161 160 207 104 244 37 272q0 141-37 271t-103 244-161 208-207 160-244 104-272 37q-141 0-271-37t-244-103-208-161-160-207-104-244-37-272q0-141 37-271t103-244 161-208 207-160T752 37t272-37zm762 555q-14-22-28-42t-29-41q-2 9-5 13t-4 18q0 9 7 17t18 16 22 12 19 7zm-69-98q0 8-3 11h6q4 0 6 1l-9-12zm-693 1463q114 0 223-29t206-82 180-130 145-172q-13-30-25-61t-12-64q0-36 3-58t7-39 4-29-3-31-17-41-37-62q1-7 3-19t4-25 1-24-5-19q-26-3-54-11t-50-24l6-5q-13 3-26 8t-25 11-26 8-27 4l-16-2 3-7q-14 4-30 10t-31 6q-10 0-29-7t-38-17-34-22-15-23l2-3q-5-6-13-11t-15-10-13-11-5-14l11-9-23-3-8-30q2 5 9 4t11-4l-36-19 25-64q-14-52-7-80t27-46 44-36 49-49l-3-12 66-80 15-2q28 0 63-2t71-7 71-10 64-13q-32-38-67-72t-75-65q-11 4-27 11t-32 18-25 24-11 27l6 19q-18 29-40 36t-45 8-48 0-48 9l-16-34 15-58-17-25 173-54q-11-28-36-42t-55-14v-10l56-9q-93-46-193-70t-205-24q-87 0-172 17t-164 49-153 80-135 108q26 0 40 13t26 29 25 29 35 14l16-12-2-22 33-47-26-74q5-3 15-10t17-7q30 0 46 3t28 11 21 23 28 38l36-28q10 4 32 13t45 22 39 27 17 26q0 15-11 24t-29 15-37 9-38 8-29 10-12 17l58 19q-20 17-43 31t-48 26l4 17-92 36v28l-7 3 5-35-4-1q-7 0-8 3t-1 7 2 8 1 6l-13-7 2 4q0 3 3 9t8 11 8 10 4 5q0 3-4 6t-10 4-8 3 0 1q14 0 6 2t-25 10-31 23-16 44q0 17 1 33t-1 33q-14-38-42-58t-68-20l-43 4 21 14q-17-2-35-4t-37-1-34 8-30 21l-6 45q0 32 14 52t49 21q30 0 59-9t57-21q-9 22-20 42t-16 44l13 6q24-16 44-5t39 32 39 43 43 32l-34 18-80-45q1 2 2 9t-1 3l-36-61q-32-1-68-10t-73-24-69-33-59-38l-7 107q0 122 33 238t93 218 147 186 193 143q-5-21-1-42t10-42 13-42 7-43q0-32-10-67t-24-71-31-71-27-66-16-58 6-47l-15-7q6-14 16-27t21-26 17-28 7-30q0-10-4-21t-7-21l21 5q17-39 46-53t73-15q5 0 21 4t34 11 34 11 24 8q0 7 8 9t9 7l-2 8q3 1 14 7t24 15 23 16 14 11q18 0 49 12t68 30 73 43 68 50 49 50 20 44l-34 36q4 51-7 78t-34 45-53 30-65 34q0 20-10 43t-25 44-36 35-42 14l-42-32q2 2 0 7t-5 2q10 19 5 44t-17 51-27 49-27 39q54 14 108 21t109 7z" />
            </svg>
        }
    }
}
