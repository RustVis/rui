// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ViewComfyRounded)]
pub fn view_comfy_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ViewComfyRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2,5v5c0,0.55,0.45,1,1,1h18c0.55,0,1-0.45,1-1V5c0-0.55-0.45-1-1-1H3C2.45,4,2,4.45,2,5z M11,20h10c0.55,0,1-0.45,1-1v-5 c0-0.55-0.45-1-1-1H11c-0.55,0-1,0.45-1,1v5C10,19.55,10.45,20,11,20z M3,20h4c0.55,0,1-0.45,1-1v-5c0-0.55-0.45-1-1-1H3 c-0.55,0-1,0.45-1,1v5C2,19.55,2.45,20,3,20z"/>
        </SvgIcon>
    }
}
