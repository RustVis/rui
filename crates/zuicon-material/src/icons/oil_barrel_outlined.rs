// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(OilBarrelOutlined)]
pub fn oil_barrel_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("OilBarrelOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M9,13.05C9,14.68,10.34,16,12,16s3-1.32,3-2.95c0-1.31-0.53-1.69-3-4.55C9.52,11.38,9,11.75,9,13.05z"/><path d="M20,13c0.55,0,1-0.45,1-1s-0.45-1-1-1h-1V5h1c0.55,0,1-0.45,1-1s-0.45-1-1-1H4C3.45,3,3,3.45,3,4s0.45,1,1,1h1v6H4 c-0.55,0-1,0.45-1,1s0.45,1,1,1h1v6H4c-0.55,0-1,0.45-1,1s0.45,1,1,1h16c0.55,0,1-0.45,1-1s-0.45-1-1-1h-1v-6H20z M17,19H7v-6 c0.55,0,1-0.45,1-1s-0.45-1-1-1V5h10v6c-0.55,0-1,0.45-1,1s0.45,1,1,1V19z"/>
        </SvgIcon>
    }
}
