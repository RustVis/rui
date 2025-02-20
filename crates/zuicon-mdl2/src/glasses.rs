// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Glasses {}

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

impl Component for Glasses {
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
                data-icon={ "Glasses" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1280 512q16 0 29 7l739 369v136q-26 0-45 19t-19 45v128q0 93-35 174t-96 142-142 96-175 36q-93 0-174-35t-142-96-96-142-36-175q0-26-19-45t-45-19q-26 0-45 19t-19 45q0 93-35 174t-96 142-142 96-175 36q-93 0-174-35t-142-96-96-142-36-175v-128q0-26-19-45t-45-19V888l739-369q13-7 29-7 26 0 45 19 10 10 24 22t28 27 22 29 9 31q0 26-19 45t-45 19q-25 0-42-16t-35-34L271 896h497q38 0 65 9t49 24 39 31 32 31 33 23 38 10q21 0 38-9t32-24 33-31 38-31 49-23 66-10h497l-484-242q-17 17-34 33t-43 17q-26 0-45-19t-19-45q0-16 9-31t22-29 27-26 25-23q19-19 45-19zm-448 640q0-26-10-49t-27-41-41-28-50-10H256q-26 0-45 19t-19 45v128q0 66 25 124t68 102 102 69 125 25q66 0 124-25t101-68 69-102 26-125v-64zm704 384q66 0 124-25t101-68 69-102 26-125q0-22-1-47t-1-50 2-49 11-46h-523q-26 0-49 10t-41 27-28 41-10 50v64q0 66 25 124t68 102 102 69 125 25z" />
            </svg>
        }
    }
}
