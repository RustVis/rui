// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct HexaditeInvestigationSemiAuto {}

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

impl Component for HexaditeInvestigationSemiAuto {
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
                data-icon={ "HexaditeInvestigationSemiAuto" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M256 586v876l645 368q-11 32-18 64t-13 66l-742-424V512L1024 0l896 512v384q-28-28-60-50t-68-43V586l-768-439-768 439zm768 147L768 879v290l225 129q2 42 10 83t23 81l-2 1-384-220V805l384-220 325 186q-35 14-68 31t-65 40l-192-109zm739 856q65 33 118 81t90 108 57 128 20 142h-128q0-79-30-149t-82-122-123-83-149-30q-80 0-149 30t-122 82-83 123-30 149h-128q0-73 20-141t57-129 90-108 118-81q-74-54-115-135t-42-174q0-79 30-149t82-122 122-83 150-30q79 0 149 30t122 82 82 123 30 149q0 92-41 173t-115 136zm-483-309q0 53 20 99t55 82 81 55 100 20q53 0 99-20t82-55 55-81 20-100q0-53-20-99t-55-82-81-55-100-20q-53 0-99 20t-82 55-55 81-20 100z" />
            </svg>
        }
    }
}
