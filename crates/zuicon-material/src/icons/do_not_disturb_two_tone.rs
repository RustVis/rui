// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DoNotDisturbTwoTone)]
pub fn do_not_disturb_two_tone(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="DoNotDisturbTwoTone"
            view_box={props.view_box.clone()}
            >
            <path d="M12,2C6.48,2,2,6.48,2,12s4.48,10,10,10s10-4.48,10-10S17.52,2,12,2z M12,20c-4.42,0-8-3.58-8-8 c0-1.85,0.63-3.55,1.69-4.9L16.9,18.31C15.55,19.37,13.85,20,12,20z M18.31,16.9L7.1,5.69C8.45,4.63,10.15,4,12,4 c4.42,0,8,3.58,8,8C20,13.85,19.37,15.55,18.31,16.9z"/>
        </SvgIcon>
    }
}
