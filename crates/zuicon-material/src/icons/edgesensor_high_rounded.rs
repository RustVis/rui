// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EdgesensorHighRounded)]
pub fn edgesensor_high_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EdgesensorHighRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M4,7L4,7c0.55,0,1,0.45,1,1v5c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1V8C3,7.45,3.45,7,4,7z M1,10L1,10c0.55,0,1,0.45,1,1 v5c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1v-5C0,10.45,0.45,10,1,10z M23,7L23,7c0.55,0,1,0.45,1,1v5c0,0.55-0.45,1-1,1h0 c-0.55,0-1-0.45-1-1V8C22,7.45,22.45,7,23,7z M20,10L20,10c0.55,0,1,0.45,1,1v5c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1v-5 C19,10.45,19.45,10,20,10z M16,2.01L8,2C6.9,2,6,2.9,6,4v16c0,1.1,0.9,2,2,2h8c1.1,0,2-0.9,2-2V4C18,2.9,17.1,2.01,16,2.01z M16,17H8V7h8V17z"/>
        </SvgIcon>
    }
}
