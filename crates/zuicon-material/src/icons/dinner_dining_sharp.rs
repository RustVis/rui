// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DinnerDiningSharp)]
pub fn dinner_dining_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DinnerDiningSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,18l16.97,0c0,0,0,0,0,0c0.29-3.26-2.28-6-5.48-6c-2.35,0-4.35,1.48-5.14,3.55C8.94,15.32,8.48,15.17,8,15.08V9h4V6.75 h9v-1.5h-9V3H3v1.5h1v0.75H3v1.5h1V7.5H3V9h1v7.39C3.56,16.85,3.22,17.39,3,18z M8,7.5V6.75h2V7.5H8z M10,5.25H8V4.5h2V5.25z M5.5,4.5h1v0.75h-1V4.5z M5.5,6.75h1V7.5h-1V6.75z M6.5,9v6.06c-0.35,0.06-0.68,0.17-1,0.3V9H6.5z"/>
        </SvgIcon>
    }
}
