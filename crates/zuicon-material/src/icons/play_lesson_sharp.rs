// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PlayLessonSharp)]
pub fn play_lesson_sharp(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="PlayLessonSharp"
            view_box={props.view_box.clone()}
            >
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M18,11c0.34,0,0.67,0.03,1,0.08V2H3v20h9.26C11.47,20.87,11,19.49,11,18C11,14.13,14.13,11,18,11z M7,11V4h5v7L9.5,9.5 L7,11z"/><path d="M18,13c-2.76,0-5,2.24-5,5s2.24,5,5,5s5-2.24,5-5S20.76,13,18,13z M16.75,20.5v-5l4,2.5L16.75,20.5z"/>
        </SvgIcon>
    }
}
