// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LyricsTwoTone)]
pub fn lyrics_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LyricsTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4,4v14l2-2h9v-4.03c-0.62-0.83-1-1.85-1-2.97c0-1.12,0.38-2.14,1-2.97V4H4z M10,14H6v-2h4V14z M13,11H6V9 h7V11z M13,8H6V6h7V8z" opacity=".3"/><path d="M15,11.97V16H6l-2,2V4h11v2.03c0.52-0.69,1.2-1.25,2-1.6V4c0-1.1-0.9-2-2-2H4C2.9,2,2.01,2.9,2.01,4L2,22l4-4h9 c1.1,0,2-0.9,2-2v-2.42C16.2,13.22,15.52,12.66,15,11.97z"/><path d="M20,6.18C19.69,6.07,19.35,6,19,6c-1.66,0-3,1.34-3,3c0,1.66,1.34,3,3,3s3-1.34,3-3V3h2V1h-4V6.18z"/>
        </SvgIcon>
    }
}
