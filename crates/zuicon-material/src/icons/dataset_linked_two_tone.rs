// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DatasetLinkedTwoTone)]
pub fn dataset_linked_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DatasetLinkedTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M5,5v14h3.09C8.04,18.67,8,18.34,8,18s0.04-0.67,0.09-1H7v-4h3.69c0.95-0.63,2.09-1,3.31-1h5V5H5z M11,11H7V7h4V11z M13,11V7h4v4H13z"/><path d="M7,17h1.09c0.28-1.67,1.24-3.1,2.6-4H7V17z"/><path d="M5,19V5h14v7h1c0.34,0,0.67,0.04,1,0.09V5c0-1.1-0.9-2-2-2H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h3.81 c-0.35-0.61-0.6-1.28-0.72-2H5z"/><path d="M16,20h-2c-1.1,0-2-0.9-2-2s0.9-2,2-2h2v-2h-2c-2.21,0-4,1.79-4,4c0,2.21,1.79,4,4,4h2V20z"/><path d="M20,14h-2v2h2c1.1,0,2,0.9,2,2s-0.9,2-2,2h-2v2h2c2.21,0,4-1.79,4-4C24,15.79,22.21,14,20,14z"/>
        </SvgIcon>
    }
}
