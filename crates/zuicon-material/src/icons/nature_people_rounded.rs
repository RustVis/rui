// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NaturePeopleRounded)]
pub fn nature_people_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="NaturePeopleRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M22.17 9.17c0-3.91-3.19-7.06-7.11-7-3.83.06-6.99 3.37-6.88 7.19.09 3.38 2.58 6.16 5.83 6.7V20H6v-3h.5c.28 0 .5-.22.5-.5V13c0-.55-.45-1-1-1H3c-.55 0-1 .45-1 1v3.5c0 .28.22.5.5.5H3v4c0 .55.45 1 1 1h14c.55 0 1-.45 1-1s-.45-1-1-1h-2v-3.88c3.47-.41 6.17-3.36 6.17-6.95z"/>
        </SvgIcon>
    }
}
