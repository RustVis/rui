// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ReceiptLongRounded)]
pub fn receipt_long_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ReceiptLongRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M14,9h-4C9.45,9,9,8.55,9,8v0c0-0.55,0.45-1,1-1h4c0.55,0,1,0.45,1,1v0C15,8.55,14.55,9,14,9z"/><path d="M14,12h-4c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h4c0.55,0,1,0.45,1,1v0C15,11.55,14.55,12,14,12z"/><path d="M19.5,3.5L18,2l-1.5,1.5L15,2l-1.5,1.5L12,2l-1.5,1.5L9,2L7.5,3.5L6,2v14H4c-0.55,0-1,0.45-1,1v2c0,1.66,1.34,3,3,3h12 c1.66,0,3-1.34,3-3V2L19.5,3.5z M15,20H6c-0.55,0-1-0.45-1-1v-1h3h4h3V20z M19,19c0,0.55-0.45,1-1,1s-1-0.45-1-1v-2 c0-0.55-0.45-1-1-1h-2h-2H8V5h11V19z"/>
        </SvgIcon>
    }
}
