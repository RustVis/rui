// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LivingOutlined)]
pub fn living_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LivingOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M17.75,10.35V9c0-1.66-1.34-3-3-3h-5.5c-1.66,0-3,1.34-3,3v1.35C5.51,10.78,5,11.58,5,12.5V16c0,1.1,0.9,2,2,2h10 c1.1,0,2-0.9,2-2v-3.5C19,11.58,18.49,10.78,17.75,10.35z M9.25,7.5h5.5c0.83,0,1.5,0.67,1.5,1.5v1.03C14.99,10.15,14,11.2,14,12.5 V13h-4v-0.5c0-1.3-0.99-2.35-2.25-2.47V9C7.75,8.17,8.42,7.5,9.25,7.5z M17.5,16c0,0.28-0.22,0.5-0.5,0.5H7 c-0.28,0-0.5-0.22-0.5-0.5v-3.5c0-0.55,0.45-1,1-1s1,0.45,1,1v2h7v-2c0-0.55,0.45-1,1-1c0.55,0,1,0.45,1,1V16z M20,4v16H4V4H20 M20,2H4C2.9,2,2,2.9,2,4v16c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z"/>
        </SvgIcon>
    }
}
