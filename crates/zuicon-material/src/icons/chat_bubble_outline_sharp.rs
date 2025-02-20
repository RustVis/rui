// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ChatBubbleOutlineSharp)]
pub fn chat_bubble_outline_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ChatBubbleOutlineSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M22 2H2v20l4-4h16V2zm-2 14H6l-2 2V4h16v12z"/>
        </SvgIcon>
    }
}
