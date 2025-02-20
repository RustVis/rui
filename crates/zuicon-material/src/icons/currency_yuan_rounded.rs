// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CurrencyYuanRounded)]
pub fn currency_yuan_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CurrencyYuanRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,21c-0.55,0-1-0.45-1-1v-6H7c-0.55,0-1-0.45-1-1s0.45-1,1-1h3.72L5.98,4.54C5.55,3.87,6.03,3,6.82,3 c0.34,0,0.66,0.17,0.84,0.46L12,10.29l4.34-6.83C16.52,3.17,16.84,3,17.18,3c0.79,0,1.27,0.87,0.84,1.54L13.28,12H17 c0.55,0,1,0.45,1,1s-0.45,1-1,1h-4v6C13,20.55,12.55,21,12,21z"/>
        </SvgIcon>
    }
}
