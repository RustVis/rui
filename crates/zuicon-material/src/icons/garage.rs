// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Garage)]
pub fn garage(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Garage"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M20,2H4C2.9,2,2,2.9,2,4v16c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M19,17.69 c0,0.45-0.35,0.81-0.78,0.81h-0.44c-0.44,0-0.78-0.36-0.78-0.81V16.5H7v1.19c0,0.45-0.35,0.81-0.78,0.81H5.78 C5.35,18.5,5,18.14,5,17.69v-6.5C5.82,8.72,6.34,7.16,6.56,6.5c0.05-0.16,0.12-0.29,0.19-0.4C6.77,6.08,6.78,6.06,6.8,6.04 C7.18,5.51,7.72,5.5,7.72,5.5h8.56c0,0,0.54,0.01,0.92,0.53c0.02,0.03,0.03,0.05,0.05,0.07c0.07,0.11,0.14,0.24,0.19,0.4 c0.22,0.66,0.74,2.23,1.56,4.69V17.69z"/>
        </SvgIcon>
    }
}
