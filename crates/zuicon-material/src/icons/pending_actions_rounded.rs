// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PendingActionsRounded)]
pub fn pending_actions_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="PendingActionsRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M18,3h-3.18C14.4,1.84,13.3,1,12,1S9.6,1.84,9.18,3H6C4.9,3,4,3.9,4,5v15c0,1.1,0.9,2,2,2h6.11 c-0.59-0.57-1.07-1.25-1.42-2H6V5h2v1c0,1.1,0.9,2,2,2h4c1.1,0,2-0.9,2-2V5h2v5.08c0.71,0.1,1.38,0.31,2,0.6V5C20,3.9,19.1,3,18,3z M12,5c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1c0.55,0,1,0.45,1,1C13,4.55,12.55,5,12,5z M17,12c-2.76,0-5,2.24-5,5s2.24,5,5,5 c2.76,0,5-2.24,5-5S19.76,12,17,12z M18.29,19l-1.65-1.65c-0.09-0.09-0.15-0.22-0.15-0.35l0-2.49c0-0.28,0.22-0.5,0.5-0.5h0 c0.28,0,0.5,0.22,0.5,0.5l0,2.29l1.5,1.5c0.2,0.2,0.2,0.51,0,0.71v0C18.8,19.2,18.49,19.2,18.29,19z"/>
        </SvgIcon>
    }
}
