// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FenceRounded)]
pub fn fence_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FenceRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,11L21,11c0-0.55-0.45-1-1-1h-1V7l-2.29-2.29c-0.39-0.39-1.02-0.39-1.41,0L14,6l-1.29-1.29c-0.39-0.39-1.02-0.39-1.41,0 L10,6L8.71,4.71c-0.39-0.39-1.02-0.39-1.41,0L5,7v3H4c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h1v2H4c-0.55,0-1,0.45-1,1v0 c0,0.55,0.45,1,1,1h1v3c0,0.55,0.45,1,1,1h12c0.55,0,1-0.45,1-1v-3h1c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1h-1v-2h1 C20.55,12,21,11.55,21,11z M16,6.83l1,1V10h-2V7.83l0.41-0.41L16,6.83z M12,6.83l0.59,0.59L13,7.83V10h-2V7.83l0.41-0.41L12,6.83z M11,14v-2h2v2H11z M13,16v2h-2v-2H13z M7,7.83l1-1l0.59,0.59L9,7.83V10H7V7.83z M7,12h2v2H7V12z M7,16h2v2H7V16z M17,18h-2v-2h2 V18z M17,14h-2v-2h2V14z"/>
        </SvgIcon>
    }
}
