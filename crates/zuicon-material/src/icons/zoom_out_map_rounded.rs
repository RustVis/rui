// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ZoomOutMapRounded)]
pub fn zoom_out_map_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ZoomOutMapRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M15.85,3.85L17.3,5.3l-2.18,2.16c-0.39,0.39-0.39,1.03,0,1.42l0,0c0.39,0.39,1.03,0.39,1.42,0L18.7,6.7l1.45,1.45 C20.46,8.46,21,8.24,21,7.79V3.5C21,3.22,20.78,3,20.5,3h-4.29C15.76,3,15.54,3.54,15.85,3.85z M3.85,8.15L5.3,6.7l2.16,2.18 c0.39,0.39,1.03,0.39,1.42,0l0,0c0.39-0.39,0.39-1.03,0-1.42L6.7,5.3l1.45-1.45C8.46,3.54,8.24,3,7.79,3H3.5 C3.22,3,3,3.22,3,3.5v4.29C3,8.24,3.54,8.46,3.85,8.15z M8.15,20.15L6.7,18.7l2.18-2.16c0.39-0.39,0.39-1.03,0-1.42l0,0 c-0.39-0.39-1.03-0.39-1.42,0L5.3,17.3l-1.45-1.45C3.54,15.54,3,15.76,3,16.21v4.29C3,20.78,3.22,21,3.5,21h4.29 C8.24,21,8.46,20.46,8.15,20.15z M20.15,15.85L18.7,17.3l-2.16-2.18c-0.39-0.39-1.03-0.39-1.42,0l0,0 c-0.39,0.39-0.39,1.03,0,1.42l2.18,2.16l-1.45,1.45C15.54,20.46,15.76,21,16.21,21h4.29c0.28,0,0.5-0.22,0.5-0.5v-4.29 C21,15.76,20.46,15.54,20.15,15.85z"/>
        </SvgIcon>
    }
}
