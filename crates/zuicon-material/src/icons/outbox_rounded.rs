// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(OutboxRounded)]
pub fn outbox_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("OutboxRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M9.21,11H11v2c0,0.55,0.45,1,1,1s1-0.45,1-1v-2h1.79c0.45,0,0.67-0.54,0.35-0.85l-2.79-2.79c-0.2-0.2-0.51-0.2-0.71,0 l-2.79,2.79C8.54,10.46,8.76,11,9.21,11z"/><path d="M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M19,14h-3.02 c-0.63,0-1.22,0.3-1.6,0.8C13.84,15.53,12.98,16,12,16s-1.84-0.47-2.38-1.2C9.24,14.3,8.65,14,8.02,14H5V5h14V14z"/>
        </SvgIcon>
    }
}
