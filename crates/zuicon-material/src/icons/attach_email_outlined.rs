// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AttachEmailOutlined)]
pub fn attach_email_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AttachEmailOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,6l8,5l8-5v3h2V4c0-1.1-0.9-2-2-2H3C1.9,2,1.01,2.9,1.01,4L1,16c0,1.1,0.9,2,2,2h10v-2H3V6z M19,4l-8,5L3,4H19z"/><path d="M21,14v4c0,1.1-0.9,2-2,2s-2-0.9-2-2v-4.5c0-0.28,0.22-0.5,0.5-0.5s0.5,0.22,0.5,0.5V18h2v-4.5c0-1.38-1.12-2.5-2.5-2.5 S15,12.12,15,13.5V18c0,2.21,1.79,4,4,4s4-1.79,4-4v-4H21z"/>
        </SvgIcon>
    }
}
