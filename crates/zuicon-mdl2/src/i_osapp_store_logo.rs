// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct IOsappStoreLogo {}

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

impl Component for IOsappStoreLogo {
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
                data-icon={ "iOSAppStoreLogo" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M457 1477q0-1 3-15t9-35 10-46 11-47 10-39 5-20l110 64q-7 7-21 20t-30 28-35 32-32 31-25 22-11 9q-4 0-4-4zm1039-264q4 0 6 1 25 9 43 27t31 43 18 50 6 52q0 5-2 22t-6 37-9 35-11 15q-4 0-5-3-7-13-12-25t-15-24q-17-21-39-33t-43-27-34-33-14-55q0-10 2-25t11-20l61-34q4-3 12-3zm3-28l-110 62-74-131q26-17 54-32t55-30l75 131zm-498-509q0 6-8 21t-18 34-21 34-14 22l-115-68 58-99q7-10 18-10 5 0 18 6t27 15 27 17 18 11q10 7 10 17zm-415 416H403V967h256l-73 125zm621-125l71 125H785l73-125h349zm191 0h261v125h-190l-71-125zm-347-549q11 0 16 9l348 610-110 63q-5-10-23-41t-44-75-56-98-62-109-61-107-53-93-37-67-14-28q0-11 9-17 4-3 17-10t28-16 27-14 15-7zM931 803l-303 521-116-68 304-520 115 67zm93-803q141 0 272 36t244 104 207 160 161 207 103 245 37 272q0 141-36 272t-104 244-160 207-207 161-245 103-272 37q-141 0-272-36t-244-104-207-160-161-207-103-245-37-272q0-141 36-272t104-244 160-207 207-161T752 37t272-37zm0 1951q128 0 246-33t222-93 187-145 145-188 94-221 33-247q0-128-33-246t-93-222-145-187-188-145-221-94-247-33q-128 0-246 33t-222 93-187 145-145 188-94 221-33 247q0 128 33 246t93 222 145 187 188 145 221 94 247 33z" />
            </svg>
        }
    }
}
