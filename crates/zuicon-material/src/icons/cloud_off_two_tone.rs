// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CloudOffTwoTone)]
pub fn cloud_off_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CloudOffTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M22 15c0-1.66-1.34-3-3-3h-1.5v-.5C17.5 8.46 15.04 6 12 6c-.77 0-1.49.17-2.16.46L20.79 17.4c.73-.55 1.21-1.41 1.21-2.4zM2 14c0 2.21 1.79 4 4 4h9.73l-8-8H6c-2.21 0-4 1.79-4 4z" opacity=".3"/><path d="M19.35 10.04C18.67 6.59 15.64 4 12 4c-1.33 0-2.57.36-3.65.97l1.49 1.49C10.51 6.17 11.23 6 12 6c3.04 0 5.5 2.46 5.5 5.5v.5H19c1.66 0 3 1.34 3 3 0 .99-.48 1.85-1.21 2.4l1.41 1.41c1.09-.92 1.8-2.27 1.8-3.81 0-2.64-2.05-4.78-4.65-4.96zM3 5.27l2.77 2.77h-.42C2.34 8.36 0 10.91 0 14c0 3.31 2.69 6 6 6h11.73l2 2 1.41-1.41L4.41 3.86 3 5.27zM7.73 10l8 8H6c-2.21 0-4-1.79-4-4s1.79-4 4-4h1.73z"/>
        </SvgIcon>
    }
}
