// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Hide3 {}

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

impl Component for Hide3 {
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
                data-icon={ "Hide3" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M74 292l90-90 1630 1629-91 91-457-457q-54 35-105 53t-117 18q-80 0-150-30t-122-82-82-122-30-150q0-65 18-116t53-106L391 610Q266 715 197 851t-69 301H0q0-91 21-179t60-170 94-153 126-130L74 292zm694 860q0 53 20 99t55 82 81 55 100 20q36 0 67-9t62-27l-349-349q-17 31-26 62t-10 67zm328-245L963 774l30-4q15-2 31-2 79 0 149 30t122 82 83 123 30 149q0 15-2 30t-4 31l-133-133q-42-131-173-173zm952 245h-128q0-118-36-221t-99-188-150-152-185-113-208-70-218-24q-98 0-192 19t-185 56l-98-98q116-53 231-79t244-26q144 0 285 35t265 105 226 170 166 234q40 82 61 171t21 181z" />
            </svg>
        }
    }
}
