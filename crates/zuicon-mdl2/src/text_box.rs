// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct TextBox {}

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

impl Component for TextBox {
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
                data-icon={ "TextBox" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2048 0v2048H0V0h2048zm-128 128H128v1792h1792V128zM397 894H283l235-628h117l234 628H754l-56-160H451l-54 160zm176-535q-2 10-3 20t-5 20l-87 250h193l-88-250q-3-9-4-19t-4-21h-2zm460 535H931V230h102v294h1q26-43 63-66t89-23q48 0 82 18t57 50 32 71 10 83q0 48-11 92t-36 79-62 55-92 21q-86 0-132-75h-1v65zm-1-188q0 24 8 45t22 38 34 25 46 10q35 0 58-15t37-39 20-54 6-59q0-26-6-51t-19-46-34-32-52-12q-29 0-51 11t-38 29-23 43-8 52v55zm632 198q-50 0-91-16t-70-46-45-71-16-91q0-55 17-100t48-77 76-50 101-18q28 0 55 5t53 16v95q-22-16-47-25t-53-10q-35 0-62 12t-45 34-29 50-10 62q0 66 37 108t105 42q29 0 55-10t49-29v88q-29 17-62 24t-66 7zM438 1792q-48 0-82-19t-57-50-32-72-11-83q0-47 11-91t37-78 62-55 91-21q43 0 77 17t55 56h2v-278h101v664H591v-77h-2q-50 87-151 87zm44-388q-35 0-58 14t-38 36-20 51-6 59q0 27 6 53t20 48 35 34 54 13q28 0 50-11t37-30 22-43 8-51v-59q0-48-30-81t-80-33zm530-81q51 0 87 17t59 48 35 71 11 87v39H899q2 63 39 96t99 34q36 0 69-11t63-32v81q-36 23-78 31t-84 8q-54 0-93-17t-66-49-39-73-13-94q0-48 15-90t43-75 68-52 90-19zm94 190q0-23-5-44t-17-36-30-25-44-10q-24 0-43 9t-34 26-22 37-12 43h207zm226-252q0-36 12-64t35-48 51-30 65-11q15 0 30 2t29 7v84q-21-12-47-12-41 0-57 23t-17 61v60h104v80h-103v369h-102v-369h-76v-80h76v-72z" />
            </svg>
        }
    }
}
