// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LivingRounded)]
pub fn living_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LivingRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M16.5,11.5c-0.55,0-1,0.45-1,1v2h-7v-2c0-0.55-0.45-1-1-1s-1,0.45-1,1V16c0,0.28,0.22,0.5,0.5,0.5h10 c0.28,0,0.5-0.22,0.5-0.5v-3.5C17.5,11.95,17.05,11.5,16.5,11.5z"/><path d="M10,12.5V13h4v-0.5c0-1.3,0.99-2.35,2.25-2.47V9c0-0.83-0.67-1.5-1.5-1.5h-5.5c-0.83,0-1.5,0.67-1.5,1.5v1.03 C9.01,10.15,10,11.2,10,12.5z"/><path d="M20,2H4C2.9,2,2,2.9,2,4v16c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M19,16c0,1.1-0.9,2-2,2H7 c-1.1,0-2-0.9-2-2v-3.5c0-0.92,0.51-1.72,1.25-2.15V9c0-1.66,1.34-3,3-3h5.5c1.66,0,3,1.34,3,3v1.35C18.49,10.78,19,11.58,19,12.5 V16z"/>
        </SvgIcon>
    }
}
