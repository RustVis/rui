// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Sketch {}

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

impl Component for Sketch {
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
                data-icon={ "sketch" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M925.6 405.1l-203-253.7a6.5 6.5 0 0 0-5-2.4H306.4c-1.9 0-3.8.9-5 2.4l-203 253.7a6.5 6.5 0 0 0 .2 8.3l408.6 459.5c1.2 1.4 3 2.1 4.8 2.1 1.8 0 3.5-.8 4.8-2.1l408.6-459.5a6.5 6.5 0 0 0 .2-8.3zM645.2 206.4l34.4 133.9-132.5-133.9h98.1zm8.2 178.5H370.6L512 242l141.4 142.9zM378.8 206.4h98.1L344.3 340.3l34.5-133.9zm-53.4 7l-44.1 171.5h-93.1l137.2-171.5zM194.6 434.9H289l125.8 247.7-220.2-247.7zM512 763.4L345.1 434.9h333.7L512 763.4zm97.1-80.8L735 434.9h94.4L609.1 682.6zm133.6-297.7l-44.1-171.5 137.2 171.5h-93.1z"/>
            </svg>
        }
    }
}
