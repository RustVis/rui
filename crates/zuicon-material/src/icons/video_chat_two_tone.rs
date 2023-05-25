// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VideoChatTwoTone)]
pub fn video_chat_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("VideoChatTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M4,17.17L5.17,16H20V4H4V17.17z M7,7c0-0.55,0.45-1,1-1h6c0.55,0,1,0.45,1,1 v1.99L17,7v6l-2-1.99V13c0,0.55-0.45,1-1,1H8c-0.55,0-1-0.45-1-1V7z" enable-background="new" opacity=".3"/><path d="M20,2H4C2.9,2,2.01,2.9,2.01,4L2,22l4-4h14c1.1,0,2-0.9,2-2V4C22,2.9,21.1,2,20,2z M20,16H5.17L4,17.17V4h16V16z"/><path d="M8,14h6c0.55,0,1-0.45,1-1v-1.99L17,13V7l-2,1.99V7c0-0.55-0.45-1-1-1H8C7.45,6,7,6.45,7,7v6C7,13.55,7.45,14,8,14z"/>
        </SvgIcon>
    }
}
