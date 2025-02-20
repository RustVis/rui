// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ThirtyFpsSelectRounded)]
pub fn thirty_fps_select_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ThirtyFpsSelectRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4,5L4,5c0,0.55,0.45,1,1,1h4v2H6C5.45,8,5,8.45,5,9v0c0,0.55,0.45,1,1,1h3v2H5c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h4 c1.1,0,2-0.9,2-2v-1.5C11,9.67,10.83,9,10,9c0.83,0,1-0.67,1-1.5V6c0-1.1-0.9-2-2-2H5C4.45,4,4,4.45,4,5z M18,4c1.1,0,2,0.9,2,2v6 c0,1.1-0.9,2-2,2h-3c-1.1,0-2-0.9-2-2V6c0-1.1,0.9-2,2-2H18z M18,6h-3v6h3V6z M4,22L4,22c-0.55,0-1-0.45-1-1v-3c0-0.55,0.45-1,1-1 h0c0.55,0,1,0.45,1,1v3C5,21.55,4.55,22,4,22z M8,22L8,22c-0.55,0-1-0.45-1-1v-3c0-0.55,0.45-1,1-1h0c0.55,0,1,0.45,1,1v3 C9,21.55,8.55,22,8,22z M12,22L12,22c-0.55,0-1-0.45-1-1v-3c0-0.55,0.45-1,1-1h0c0.55,0,1,0.45,1,1v3C13,21.55,12.55,22,12,22z M20,22h-4c-0.55,0-1-0.45-1-1v-3c0-0.55,0.45-1,1-1h4c0.55,0,1,0.45,1,1v3C21,21.55,20.55,22,20,22z"/>
        </SvgIcon>
    }
}
