// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ViewStreamTwoTone)]
pub fn view_stream_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ViewStreamTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,17H5v-4h14V17z M5,11V7h14v4H5z" opacity=".3"/><path d="M3,7v10c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V7c0-1.1-0.9-2-2-2H5C3.9,5,3,5.9,3,7z M19,17H5v-4h14V17z M5,11V7h14v4H5z"/>
        </SvgIcon>
    }
}
