// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MarkChatReadRounded)]
pub fn mark_chat_read_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MarkChatReadRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18.05,19.29c-0.39,0.39-1.02,0.39-1.41,0l-2.12-2.12c-0.39-0.39-0.39-1.02,0-1.41l0,0c0.39-0.39,1.02-0.39,1.41,0 l1.41,1.41l3.54-3.54c0.39-0.39,1.02-0.39,1.41,0l0,0c0.39,0.39,0.39,1.02,0,1.41L18.05,19.29z M12,17c0-3.87,3.13-7,7-7 c1.08,0,2.09,0.25,3,0.68V4c0-1.1-0.9-2-2-2H4C2.9,2,2,2.9,2,4v18l4-4h6v0c0-0.17,0.01-0.33,0.03-0.5C12.01,17.33,12,17.17,12,17z"/>
        </SvgIcon>
    }
}
