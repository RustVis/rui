// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ChronosLogo {}

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

impl Component for ChronosLogo {
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
                data-icon={ "ChronosLogo" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1047 1002l-54-284h424l-370 284zM1024 0q141 0 272 36t244 104 207 160 161 207 103 245 37 272q0 141-36 272t-104 244-160 207-207 161-245 103-272 37q-141 0-272-36t-244-104-207-160-161-207-103-245-37-272q0-141 36-272t104-244 160-207 207-161T752 37t272-37zM255 1500q103-81 205-161t206-163q50-39 101-78t99-82l7-7v-7q-3-24-8-49t-9-49L701 169q-133 50-242 136T273 504 153 748t-43 275q0 126 34 246t100 228l11 3zm962 350q-23-108-45-214t-46-216q-15-72-30-142t-27-143q-1-5-1-10t-1-10q0-27 10-43t29-35q13-14 29-25t31-23q35-28 69-55t69-55q107-84 213-166t212-168L808 210q26 122 51 242t50 243q15 74 30 147t29 147q1 5 1 10t1 10q0 26-9 42t-27 33q-14 14-30 26t-32 25l-58 48q-29 24-60 47-99 78-196 154t-198 154l857 312zm107 38q36-14 70-28t70-33q110-59 198-145t150-190 95-224 33-244q0-114-28-224t-83-211q-110 86-218 171t-218 173q-55 43-112 85t-109 91l-6 6v8q7 60 22 126t28 126q26 129 53 256t55 257zm-285-515l89 392-662-241 219-151h354z" />
            </svg>
        }
    }
}
