// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct TwitterLogo {}

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

impl Component for TwitterLogo {
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
                data-icon={ "TwitterLogo" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2048 389q-42 63-95 117t-115 100q1 14 1 27t1 28q0 126-27 249t-78 238q-74 167-185 298t-251 223-307 139-348 48q-172 0-335-47T0 1667q49 6 100 6 143 0 276-46t246-134q-67-1-129-22t-113-60-90-92-60-117q20 3 39 5t40 2q56 0 110-15-74-15-135-53t-107-92-70-123-25-144v-5q88 50 191 53-44-30-78-68t-59-84-37-95-13-103q0-56 14-109t43-102q80 99 178 177t208 135 232 88 248 39q-6-23-8-47t-3-49q0-87 33-163t90-134 133-90 164-33q88 0 167 34t140 99q71-14 137-39t129-63q-23 73-70 133t-114 99q126-15 241-66z" />
            </svg>
        }
    }
}
