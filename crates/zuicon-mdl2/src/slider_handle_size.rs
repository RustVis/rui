// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct SliderHandleSize {}

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

impl Component for SliderHandleSize {
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
                data-icon={ "SliderHandleSize" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M1919 1249q61 55 95 129t34 158q0 80-30 149t-82 122-123 83-149 30q-88 0-167-38-105 38-217 38-45 0-89-6t-89-19q-51 13-102 19t-104 6q-124 0-238-32t-214-90-181-140-140-181-91-214-32-239q0-124 32-238t90-214 140-181 181-140 214-91 239-32q110 0 213 25t195 74 172 115 140 151 104 181 60 206q63 79 98 173t41 196zM128 1024q0 106 27 204t78 183 120 156 155 120 184 77 204 28q-61-46-108-103t-81-122-50-138-17-149q0-133 50-249t137-204 203-137 250-50q86 0 168 22t156 66q-45-106-118-193t-165-149-201-96-224-34q-106 0-204 27t-183 78-156 120-120 155-77 184-28 204zm640 256q0 106 40 199t110 162 163 110 199 41q23 0 45-2t46-6q-45-52-68-116t-23-132q0-79 30-149t82-122 122-83 150-30q60 0 116 18-19-88-66-162t-113-127-149-83-172-30q-106 0-199 40T919 918t-110 163-41 199zm896 512q53 0 99-20t82-55 55-81 20-100q0-53-20-99t-55-82-81-55-100-20q-53 0-99 20t-82 55-55 81-20 100q0 53 20 99t55 82 81 55 100 20z" />
            </svg>
        }
    }
}
