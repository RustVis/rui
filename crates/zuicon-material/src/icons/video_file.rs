// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VideoFile)]
pub fn video_file(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("VideoFile"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14,2H6.01c-1.1,0-2,0.89-2,2L4,20c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V8L14,2z M13,9V3.5L18.5,9H13z M14,14l2-1.06v4.12 L14,16v1c0,0.55-0.45,1-1,1H9c-0.55,0-1-0.45-1-1v-4c0-0.55,0.45-1,1-1h4c0.55,0,1,0.45,1,1V14z"/>
        </SvgIcon>
    }
}
