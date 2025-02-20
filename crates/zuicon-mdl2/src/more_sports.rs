// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct MoreSports {}

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

impl Component for MoreSports {
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
                data-icon={ "MoreSports" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M621 1171l256 256-90 90-256-256 90-90zM1216 0q115 0 221 29t199 84 168 130 130 168 84 199 30 222q0 146-48 279t-134 241-206 184-261 108q9 74 9 148 0 53-20 99t-55 82-81 55-100 20q-159 0-306-41t-276-116-233-180-180-233-116-275T0 896q0-53 20-99t55-82 81-55 100-20q74 0 148 9 32-142 108-261t184-205T937 48t279-48zm354 569q48 35 105 53t117 18q50 0 98-12-20-68-54-130t-79-116l-187 187zm96-278q-54-45-116-79t-130-54q-12 48-12 98 0 60 18 117t53 105l187-187zm-371-158q-20-3-39-4t-40-1q-124 0-239 42T766 292l450 450 172-172q-53-67-80-147t-28-167q0-63 15-123zM675 382q-53 63-90 136t-55 155q137 32 263 99t231 161l102-101-451-450zm477 1538q27 0 50-10t40-27 28-41 10-50q0-141-36-272t-104-244-160-207-207-161-245-103-272-37q-27 0-50 10t-40 27-28 41-10 50q0 141 36 272t104 244 160 207 207 161 245 103 272 37zm-37-896q94 105 161 231t99 263q81-18 154-55t137-90l-450-451-101 102zm641 258q79-95 121-210t43-240q0-20-1-39t-4-40q-60 15-123 15-86 0-166-27t-148-81l-172 172 450 450z" />
            </svg>
        }
    }
}
