// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MarkAsUnreadTwoTone)]
pub fn mark_as_unread_two_tone(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="MarkAsUnreadTwoTone"
            view_box={props.view_box.clone()}
            >
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M16.23,7h2.6c-0.06-0.47-0.36-0.94-0.79-1.17L10.5,2L2.8,5.83C2.32,6.09,2,6.64,2,7.17V15c0,1.1,0.9,2,2,2V7.4L10.5,4 L16.23,7z"/><path d="M20,8H7c-1.1,0-2,0.9-2,2v9c0,1.1,0.9,2,2,2h13c1.1,0,2-0.9,2-2v-9C22,8.9,21.1,8,20,8z M20,19H7v-7l6.5,3.33L20,12V19z M13.5,13.33L7,10h13L13.5,13.33z"/>
        </SvgIcon>
    }
}
