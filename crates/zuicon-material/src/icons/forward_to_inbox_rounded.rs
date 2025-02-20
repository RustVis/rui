// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ForwardToInboxRounded)]
pub fn forward_to_inbox_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ForwardToInboxRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,4H4C2.9,4,2,4.9,2,6v12c0,1.1,0.9,2,2,2h9v-2H4V8l6.94,4.34c0.65,0.41,1.47,0.41,2.12,0L20,8v5h2V6C22,4.9,21.1,4,20,4 z M12,11L4,6h16L12,11z M19,16.21c0-0.45,0.54-0.67,0.85-0.35l2.79,2.79c0.2,0.2,0.2,0.51,0,0.71l-2.79,2.79 C19.54,22.46,19,22.24,19,21.79V20h-3c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h3V16.21z"/>
        </SvgIcon>
    }
}
