// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FaxOutlined)]
pub fn fax_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FaxOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,9h-1V4H8v5H7.22C6.67,8.39,5.89,8,5,8c-1.66,0-3,1.34-3,3v7c0,1.66,1.34,3,3,3c0.89,0,1.67-0.39,2.22-1H22v-8 C22,10.34,20.66,9,19,9z M6,18c0,0.55-0.45,1-1,1s-1-0.45-1-1v-7c0-0.55,0.45-1,1-1s1,0.45,1,1V18z M10,6h6v3h-6V6z M20,18H8v-7 h11c0.55,0,1,0.45,1,1V18z"/>
        </SvgIcon>
    }
}
