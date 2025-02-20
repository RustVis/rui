// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct BugSolid {}

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

impl Component for BugSolid {
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
                data-icon={ "BugSolid" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M704 448q0-72 30-135L595 173l90-90 126 126q45-39 99-60t114-21q60 0 114 21t99 60l126-126 90 90-139 140q30 63 30 135 0 13-1 25t-4 25q-75-32-154-49t-161-17q-81 0-160 17t-155 49q-2-12-3-24t-2-26zm896 448q66 0 124 25t101 69 69 102 26 124v192h-128v-192q0-34-11-65t-33-57-49-43-63-24q19 62 24 125t5 128v72q0 36-1 72l3 4q61 66 93 138t32 163q0 85-32 162t-93 138l-90-90q42-42 64-97t23-114q0-67-29-128-32 100-92 182t-140 142-177 91-202 33q-105 0-202-32t-177-92-140-141-92-183q-29 61-29 128 0 59 22 114t65 97l-90 90q-60-60-92-137t-33-163q0-91 32-163t93-138l3-4v-72q0-36-1-72 0-64 5-127t24-126q-34 6-62 23t-50 43-32 57-12 66v192H128v-192q0-66 25-124t68-101 102-69 125-26h22q40-69 97-128-65-2-121-28t-99-69-66-100-25-123V256h128v192q0 40 15 75t41 61 61 41 75 15h169q133-64 279-64t279 64h169q40 0 75-15t61-41 41-61 15-75V256h128v192q0 65-24 122t-67 101-98 69-122 28q57 59 97 128h22z" />
            </svg>
        }
    }
}
