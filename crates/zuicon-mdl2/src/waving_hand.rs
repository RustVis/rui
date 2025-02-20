// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct WavingHand {}

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

impl Component for WavingHand {
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
                data-icon={ "WavingHand" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1920 1401q0 83-19 163t-57 152-89 132-119 105-144 68-165 25q-59 0-118-11t-114-33-106-54-92-75l-621-621q-27-27-41-62t-15-74q0-57 31-104t84-71q-25-45-25-95 0-57 31-105t85-71q-26-44-26-95 0-40 15-75t41-61 61-41 75-15q50 0 95 25 24-53 71-84t105-31q38 0 73 14t62 42l538 538V706q0-38 14-73t42-63q27-26 62-41t74-15q40 0 75 15t61 41 41 61 15 75v695zm-128-3V706q0-26-19-45t-45-19q-27 0-45 18t-19 46v227q0 33-18 60t-49 40q-10 5-20 6t-22 2q-46 0-77-31L908 440q-19-19-45-19t-45 19-19 45q0 26 19 45l361 361q19 19 19 45t-19 45-45 19q-26 0-45-19L637 530q-19-19-45-19t-45 19-19 45q0 26 19 45l451 451q19 19 19 46 0 26-19 45t-45 19q-26 0-45-19L547 801q-19-19-45-19t-45 19-19 45q0 26 19 45l406 406q19 19 19 45t-19 45-45 19q-26 0-45-19l-317-315q-19-19-44-19-26 0-45 18t-19 46q0 25 19 44l620 621q66 66 156 100t183 35q105 0 191-43t147-116 94-165 34-195zM1152 64q0-26 19-45t45-19q93 0 174 35t143 96 96 142 35 175q0 26-19 45t-45 19q-26 0-45-19t-19-45q0-66-25-124t-69-101-102-69-124-26q-26 0-45-19t-19-45zm0 224q0-26 19-45t45-19q46 0 87 17t71 48 48 72 18 87q0 26-19 45t-45 19q-26 0-45-19t-19-45q0-40-28-68t-68-28q-26 0-45-19t-19-45zM512 1856q0 26-19 45t-45 19q-93 0-174-35t-143-96-96-142-35-175q0-26 19-45t45-19q26 0 45 19t19 45q0 66 25 124t68 102 102 69 125 25q26 0 45 19t19 45zm0-224q0 26-19 45t-45 19q-47 0-87-17t-71-48-48-71-18-88q0-26 19-45t45-19q26 0 45 19t19 45q0 40 28 68t68 28q26 0 45 19t19 45z" />
            </svg>
        }
    }
}
