// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TenKTwoTone)]
pub fn ten_k_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TenKTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M5,9h2.5v6H6v-4.5H5V19h14v-4h-1.75l-1.75-2.25V15H14V9h1.5v2.25L17.25,9H19V5H5V9z M8.5,10 c0-0.55,0.45-1,1-1H12c0.55,0,1,0.45,1,1v4c0,0.55-0.45,1-1,1H9.5c-0.55,0-1-0.45-1-1V10z" opacity=".3"/><path d="M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M19,9v6v4H5v-8.5h1V15h1.5V9H5V5 h14V9z"/><path d="M9.5,15H12c0.55,0,1-0.45,1-1v-4c0-0.55-0.45-1-1-1H9.5c-0.55,0-1,0.45-1,1v4C8.5,14.55,8.95,15,9.5,15z M10,10.5h1.5v3 H10V10.5z"/>
        </SvgIcon>
    }
}
