// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CastleTwoTone)]
pub fn castle_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CastleTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17,7H7v6H3v6h5v-1c0-2.21,1.79-4,4-4s4,1.79,4,4v1h5v-6h-4V7z M11,12H9V9h2V12z M15,12h-2V9h2V12z" opacity=".3"/><path d="M21,9v2h-2V3h-2v2h-2V3h-2v2h-2V3H9v2H7V3H5v8H3V9H1v12h9v-3c0-1.1,0.9-2,2-2s2,0.9,2,2v3h9V9H21z M21,19h-5v-1 c0-2.21-1.79-4-4-4s-4,1.79-4,4v1H3v-6h4V7h10v6h4V19z"/>
        </SvgIcon>
    }
}
