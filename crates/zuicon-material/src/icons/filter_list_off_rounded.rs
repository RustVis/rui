// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FilterListOffRounded)]
pub fn filter_list_off_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FilterListOffRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,7c0-0.55-0.45-1-1-1H8.83l2,2H20C20.55,8,21,7.55,21,7z M18,12c0-0.55-0.45-1-1-1h-3.17l2,2H17 C17.55,13,18,12.55,18,12z M13.98,16.81C13.99,16.87,14,16.94,14,17c0,0.55-0.45,1-1,1h-2c-0.55,0-1-0.45-1-1s0.45-1,1-1h2 c0.06,0,0.13,0.01,0.19,0.02L10.17,13H7c-0.55,0-1-0.45-1-1s0.45-1,1-1h1.17l-3-3H4C3.45,8,3,7.55,3,7c0-0.32,0.15-0.6,0.38-0.79 L2.1,4.93c-0.39-0.39-0.39-1.02,0-1.41s1.02-0.39,1.41,0l16.97,16.97c0.39,0.39,0.39,1.02,0,1.41s-1.02,0.39-1.41,0L13.98,16.81z"/>
        </SvgIcon>
    }
}
