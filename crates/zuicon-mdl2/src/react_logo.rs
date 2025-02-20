// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct ReactLogo {}

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

impl Component for ReactLogo {
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
                data-icon={ "ReactLogo" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M2048 1039q0 47-21 88t-57 75-81 63-92 51-94 40-82 28q15 63 24 126t9 127q0 53-9 109t-34 101-67 75-108 29q-55 0-112-21t-111-55-102-72-87-75q-38 36-86 75t-103 72-111 54-112 22q-65 0-107-29t-67-74-34-102-10-109q0-64 9-127t24-126q-37-11-82-28t-93-39-93-51-80-63-57-76-22-88q0-47 21-88t57-75 81-63 92-51 94-40 82-28q-15-63-24-126t-9-127q0-53 9-109t34-101 67-75 108-29q55 0 112 21t111 55 102 72 87 76q38-36 86-75t103-73 111-54 112-22q65 0 107 29t67 74 34 102 10 109q0 64-9 127t-24 126q36 11 82 28t93 39 93 51 80 63 57 76 22 88zm-612-816q-43 0-91 20t-94 49-88 64-71 62q47 49 88 101t81 107q69 6 136 17t133 27q13-56 21-113t8-114q0-30-4-68t-16-72-37-57-66-23zm-834 930q-17 41-31 81t-28 83q43 11 86 18t87 13q-31-48-59-96t-55-99zm-59-392q13 42 27 83t32 82q26-51 55-99t59-97q-44 6-87 13t-86 18zm109 278q80 166 186 322 47 4 93 5t93 2q47 0 93-1t93-6q106-156 186-322-42-85-87-164t-99-158q-47-3-93-5t-93-2q-47 0-93 2t-93 5q-53 78-98 157t-88 165zm680 309q44-5 87-12t86-19q-13-42-27-82t-32-82q-26 51-54 99t-60 96zm114-422q17-41 31-82t28-83q-43-10-86-17t-87-14q30 48 59 96t55 100zm-311-309q-26-35-53-67t-58-65q-30 32-57 64t-54 68q55-3 111-3 55 0 111 3zM489 443q0 57 8 114t21 113q66-16 133-27t136-17q39-55 80-107t89-101q-30-28-71-62t-87-63-95-50-91-20q-41 0-65 23t-37 56-17 72-4 69zm-38 849q20-65 44-128t52-125q-28-62-52-125t-44-128q-24 7-59 19t-74 30-79 40-71 48-52 55-20 61q0 32 19 61t51 55 71 48 79 39 75 30 60 20zm161 563q43 0 91-20t94-49 88-63 71-62q-47-49-88-101t-81-108q-68-6-135-17t-134-26q-13 56-21 112t-8 114q0 31 3 69t17 72 37 56 66 23zm301-393q52 69 111 132 58-63 111-132-56 3-111 3-57 0-111-3zm646 173q0-57-8-113t-21-113q-67 15-134 26t-135 17q-39 55-80 107t-89 102q30 28 71 61t87 63 95 50 91 20q41 0 65-22t37-56 17-72 4-70zm38-343q24-7 59-19t75-30 79-40 71-48 51-55 20-61q0-32-20-61t-51-55-71-48-79-39-75-30-59-20q-20 65-44 128t-52 125q28 62 52 125t44 128zm-573-62q-39 0-74-15t-60-41-41-61-15-74q0-39 15-74t40-60 61-41 74-15q39 0 74 15t60 40 41 61 15 74q0 39-15 74t-40 61-61 41-74 15z" />
            </svg>
        }
    }
}
