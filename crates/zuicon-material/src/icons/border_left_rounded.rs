// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BorderLeftRounded)]
pub fn border_left_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BorderLeftRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M11 21h2v-2h-2v2zm0-4h2v-2h-2v2zm0-12h2V3h-2v2zm0 4h2V7h-2v2zm0 4h2v-2h-2v2zm-4 8h2v-2H7v2zM7 5h2V3H7v2zm0 8h2v-2H7v2zm-3 8c.55 0 1-.45 1-1V4c0-.55-.45-1-1-1s-1 .45-1 1v16c0 .55.45 1 1 1zM19 9h2V7h-2v2zm-4 12h2v-2h-2v2zm4-4h2v-2h-2v2zm0-14v2h2V3h-2zm0 10h2v-2h-2v2zm0 8h2v-2h-2v2zm-4-8h2v-2h-2v2zm0-8h2V3h-2v2z"/>
        </SvgIcon>
    }
}
