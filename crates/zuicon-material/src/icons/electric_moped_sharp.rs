// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ElectricMopedSharp)]
pub fn electric_moped_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ElectricMopedSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,8.35V3h-5v2h3v2.65L13.52,12H10V7H6c-2.21,0-4,1.79-4,4v3h2c0,1.66,1.34,3,3,3s3-1.34,3-3h4.48L19,8.35z M7,15 c-0.55,0-1-0.45-1-1h2C8,14.55,7.55,15,7,15z"/><path d="M19,11c-1.66,0-3,1.34-3,3s1.34,3,3,3s3-1.34,3-3S20.66,11,19,11z M19,15c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1 S19.55,15,19,15z"/>
        </SvgIcon>
    }
}
