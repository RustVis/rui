// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Variable {}

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

impl Component for Variable {
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
                data-icon={ "Variable" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1072 732H848l-159 743q-17 78-58 158t-100 146-136 107-164 42q-26 0-53-5t-49-17-36-32-15-52q0-40 24-68t67-29q25 0 45 11t36 28 29 38 25 40l7 12q32-2 58-22t46-50 37-67 28-75 20-72 14-59l167-777H505l20-81h173l12-62q16-80 54-161t97-146 133-105 166-41q26 0 53 5t49 17 37 32 14 52q0 42-22 70t-68 29q-32 0-53-12t-36-32-27-42-26-45q-37 2-66 28t-51 66-39 90-27 97-20 91-13 69h226l-19 81zm664 64q-25 20-53 53t-54 69-50 74-40 65q10 41 19 81t20 81q10 37 22 77t34 73 52 55 78 22q12 0 28-1t32-4 32-6 27-9l-16 59q-30 13-56 24t-53 19-54 12-62 5q-44 0-72-12t-47-35-31-55-24-70q-14-44-25-87t-22-88q-38 60-76 115t-83 110q-20 24-40 46t-43 39-51 27-60 10q-42 0-69-24t-28-68q0-26 10-45t27-33 39-19 46-7q30 0 57 11t57 19q57-57 106-121t88-134l-2-10q-4-19-12-51t-17-69-22-77-24-72-26-57-27-33q-34-20-77-20-27 0-56 5t-54 17l16-60q14-6 42-17t58-22 58-19 42-8q44 0 73 14t50 39 33 58 24 71q11 39 20 79t18 81l2 8v3q0 1 1 3 19-30 41-68t48-78 55-78 63-69 70-48 78-19q39 0 66 26t28 66q0 25-10 44t-26 32-38 19-45 7q-26 0-58-8t-57-17z" />
            </svg>
        }
    }
}
