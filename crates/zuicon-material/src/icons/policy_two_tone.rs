// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PolicyTwoTone)]
pub fn policy_two_tone(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="PolicyTwoTone"
            view_box={props.view_box.clone()}
            >
            <path d="M5,6.3V11c0,4.52,2.98,8.69,7,9.93c1.74-0.53,3.28-1.62,4.47-3.04l-1.72-1.72 c-1.94,1.29-4.58,1.07-6.29-0.64c-1.95-1.95-1.95-5.12,0-7.07c1.95-1.95,5.12-1.95,7.07,0c1.71,1.71,1.92,4.35,0.64,6.29 l1.45,1.45C18.49,14.65,19,12.85,19,11V6.3l-7-3.11L5,6.3z" opacity=".3"/><path d="M12,1L3,5v6c0,5.55,3.84,10.74,9,12c0.65-0.16,1.27-0.38,1.87-0.65c1.8-0.82,3.36-2.13,4.57-3.74 C20.04,16.46,21,13.77,21,11V5L12,1z M19,11c0,1.85-0.51,3.65-1.38,5.21l-1.45-1.45c1.29-1.94,1.07-4.58-0.64-6.29 c-1.95-1.95-5.12-1.95-7.07,0c-1.95,1.95-1.95,5.12,0,7.07c1.71,1.71,4.35,1.92,6.29,0.64l1.72,1.72 c-1.19,1.42-2.73,2.51-4.47,3.04C7.98,19.69,5,15.52,5,11V6.3l7-3.11l7,3.11V11z M15,12c0,1.66-1.34,3-3,3s-3-1.34-3-3s1.34-3,3-3 S15,10.34,15,12z"/>
        </SvgIcon>
    }
}
