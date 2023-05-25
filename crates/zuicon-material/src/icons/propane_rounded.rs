// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PropaneRounded)]
pub fn propane_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="PropaneRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M16.75,6L16,6V5c0-1.1-0.9-2-2-2h-4C8.9,3,8,3.9,8,5v1L7.25,6C3.97,6,1.1,8.53,1,11.82C0.9,15.21,3.62,18,7,18v2 c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-2h6v2c0,0.55,0.45,1,1,1h0c0.55,0,1-0.45,1-1v-2c3.38,0,6.1-2.79,6-6.18 C22.9,8.53,20.03,6,16.75,6z M10,5h4v1h-4V5z"/>
        </SvgIcon>
    }
}
