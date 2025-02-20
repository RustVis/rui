// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NearbyErrorRounded)]
pub fn nearby_error_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NearbyErrorRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M11.29,8.28l-3.01,3.01c-0.39,0.39-0.39,1.02,0,1.41l3.01,3.01c0.39,0.39,1.02,0.39,1.41,0l3.01-3.01 c0.39-0.39,0.39-1.02,0-1.41l-3.01-3.01C12.32,7.89,11.68,7.89,11.29,8.28z"/><path d="M10.59,2.59l-8.01,8c-0.78,0.78-0.78,2.05,0,2.83l8.01,8c0.78,0.78,2.05,0.78,2.83,0L18,16.82V13.2l-6,6L4.79,12L12,4.79 l6,6V7.17l-4.58-4.58C12.64,1.8,11.37,1.8,10.59,2.59z"/><path d="M20,11v6c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-6c0-0.55-0.45-1-1-1h0C20.45,10,20,10.45,20,11z"/>
        </SvgIcon>
    }
}
