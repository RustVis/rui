// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CurrencyFrancRounded)]
pub fn currency_franc_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CurrencyFrancRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18,4c0-0.55-0.45-1-1-1H8C7.45,3,7,3.45,7,4v12H6c-0.55,0-1,0.45-1,1s0.45,1,1,1h1v2c0,0.55,0.45,1,1,1h0 c0.55,0,1-0.45,1-1v-2h3c0.55,0,1-0.45,1-1s-0.45-1-1-1H9v-3h7c0.55,0,1-0.45,1-1c0-0.55-0.45-1-1-1H9V5h8C17.55,5,18,4.55,18,4z"/>
        </SvgIcon>
    }
}
