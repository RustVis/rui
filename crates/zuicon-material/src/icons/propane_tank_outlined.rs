// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PropaneTankOutlined)]
pub fn propane_tank_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PropaneTankOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17,6.14V4c0-1.1-0.9-2-2-2H9C7.9,2,7,2.9,7,4v2.14c-1.72,0.45-3,2-3,3.86v8c0,2.21,1.79,4,4,4h8c2.21,0,4-1.79,4-4v-8 C20,8.14,18.72,6.59,17,6.14z M9,4h6v2h-2c0-0.55-0.45-1-1-1s-1,0.45-1,1H9V4z M8,8h8c1.1,0,2,0.9,2,2v3H6v-3C6,8.9,6.9,8,8,8z M16,20H8c-1.1,0-2-0.9-2-2v-3h12v3C18,19.1,17.1,20,16,20z"/>
        </SvgIcon>
    }
}
