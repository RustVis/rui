// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MoneyTwoTone)]
pub fn money_two_tone(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="MoneyTwoTone"
            view_box={props.view_box.clone()}
            >
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M10 10h1v4h-1zm6 0h1v4h-1zM4 18h16V6H4v12zm10-9c0-.55.45-1 1-1h3c.55 0 1 .45 1 1v6c0 .55-.45 1-1 1h-3c-.55 0-1-.45-1-1V9zM8 9c0-.55.45-1 1-1h3c.55 0 1 .45 1 1v6c0 .55-.45 1-1 1H9c-.55 0-1-.45-1-1V9zM5 8h2v8H5V8z" opacity=".3"/><path d="M15 16h3c.55 0 1-.45 1-1V9c0-.55-.45-1-1-1h-3c-.55 0-1 .45-1 1v6c0 .55.45 1 1 1zm1-6h1v4h-1v-4zm-7 6h3c.55 0 1-.45 1-1V9c0-.55-.45-1-1-1H9c-.55 0-1 .45-1 1v6c0 .55.45 1 1 1zm1-6h1v4h-1v-4zM5 8h2v8H5zM2 4v16h20V4H2zm18 14H4V6h16v12z"/>
        </SvgIcon>
    }
}
