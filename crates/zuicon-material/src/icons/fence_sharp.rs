// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FenceSharp)]
pub fn fence_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FenceSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,12v-2h-2V7l-3-3l-2,2l-2-2l-2,2L8,4L5,7v3H3v2h2v2H3v2h2v4h14v-4h2v-2h-2v-2H21z M16,6.83l1,1V10h-2V7.83l0.41-0.41 L16,6.83z M12,6.83l0.59,0.59L13,7.83V10h-2V7.83l0.41-0.41L12,6.83z M11,14v-2h2v2H11z M13,16v2h-2v-2H13z M7,7.83l1-1l0.59,0.59 L9,7.83V10H7V7.83z M7,12h2v2H7V12z M7,16h2v2H7V16z M17,18h-2v-2h2V18z M17,14h-2v-2h2V14z"/>
        </SvgIcon>
    }
}
