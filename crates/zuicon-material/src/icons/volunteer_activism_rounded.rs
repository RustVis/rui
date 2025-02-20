// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VolunteerActivismRounded)]
pub fn volunteer_activism_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("VolunteerActivismRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,11L3,11c-1.1,0-2,0.9-2,2v7c0,1.1,0.9,2,2,2h0c1.1,0,2-0.9,2-2v-7C5,11.9,4.1,11,3,11z"/><path d="M10,5.3C10,3.45,11.45,2,13.3,2c1.04,0,2.05,0.49,2.7,1.25C16.65,2.49,17.66,2,18.7,2C20.55,2,22,3.45,22,5.3 c0,2.1-2.5,4.51-5.33,7.09c-0.38,0.35-0.97,0.35-1.35,0C12.5,9.81,10,7.4,10,5.3"/><path d="M19.99,17h-6.83c-0.11,0-0.22-0.02-0.33-0.06l-1.47-0.51c-0.26-0.09-0.39-0.37-0.3-0.63l0,0c0.09-0.26,0.38-0.4,0.64-0.3 l1.12,0.43c0.11,0.04,0.24,0.07,0.36,0.07h2.63c0.65,0,1.18-0.53,1.18-1.18v0c0-0.49-0.31-0.93-0.77-1.11L9.3,11.13 C9.08,11.04,8.84,11,8.6,11H7v9.02l6.37,1.81c0.41,0.12,0.85,0.1,1.25-0.05L22,19l0,0C22,17.89,21.1,17,19.99,17z"/>
        </SvgIcon>
    }
}
