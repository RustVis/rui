// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CastleRounded)]
pub fn castle_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CastleRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,9c-0.55,0-1,0.45-1,1v1h-2V4c0-0.55-0.45-1-1-1s-1,0.45-1,1v1h-2V4c0-0.55-0.45-1-1-1s-1,0.45-1,1v1h-2V4 c0-0.55-0.45-1-1-1S9,3.45,9,4v1H7V4c0-0.55-0.45-1-1-1S5,3.45,5,4v7H3v-1c0-0.55-0.45-1-1-1s-1,0.45-1,1v9c0,1.1,0.9,2,2,2h7v-3 c0-1.1,0.9-2,2-2s2,0.9,2,2v3h7c1.1,0,2-0.9,2-2v-9C23,9.45,22.55,9,22,9z M11,12H9V9h2V12z M15,12h-2V9h2V12z"/>
        </SvgIcon>
    }
}
