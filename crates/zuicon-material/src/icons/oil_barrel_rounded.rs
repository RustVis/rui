// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(OilBarrelRounded)]
pub fn oil_barrel_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("OilBarrelRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,13c0.55,0,1-0.45,1-1s-0.45-1-1-1h-1V5h1c0.55,0,1-0.45,1-1s-0.45-1-1-1H4C3.45,3,3,3.45,3,4s0.45,1,1,1h1v6H4 c-0.55,0-1,0.45-1,1s0.45,1,1,1h1v6H4c-0.55,0-1,0.45-1,1s0.45,1,1,1h16c0.55,0,1-0.45,1-1s-0.45-1-1-1h-1v-6H20z M12,16 c-1.66,0-3-1.32-3-2.95c0-1.16,0.41-1.58,2.24-3.68c0.4-0.46,1.12-0.46,1.51,0c1.82,2.09,2.24,2.52,2.24,3.68 C15,14.68,13.66,16,12,16z"/>
        </SvgIcon>
    }
}
