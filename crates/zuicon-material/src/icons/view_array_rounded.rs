// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ViewArrayRounded)]
pub fn view_array_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="ViewArrayRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M20,5h-1c-0.55,0-1,0.45-1,1v12c0,0.55,0.45,1,1,1h1c0.55,0,1-0.45,1-1V6C21,5.45,20.55,5,20,5z M16,5H8C7.45,5,7,5.45,7,6 v12c0,0.55,0.45,1,1,1h8c0.55,0,1-0.45,1-1V6C17,5.45,16.55,5,16,5z M5,5H4C3.45,5,3,5.45,3,6v12c0,0.55,0.45,1,1,1h1 c0.55,0,1-0.45,1-1V6C6,5.45,5.55,5,5,5z"/>
        </SvgIcon>
    }
}
