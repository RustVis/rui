// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ShowerRounded)]
pub fn shower_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="ShowerRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M13,5.08V4c0-0.55-0.45-1-1-1h0c-0.55,0-1,0.45-1,1v1.08C7.61,5.57,5,8.47,5,12v1c0,0.55,0.45,1,1,1h12c0.55,0,1-0.45,1-1 v-1C19,8.47,16.39,5.57,13,5.08z"/>
        </SvgIcon>
    }
}
