// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(West)]
pub fn west(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("West"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M9,19l1.41-1.41L5.83,13H22V11H5.83l4.59-4.59L9,5l-7,7L9,19z"/>
        </SvgIcon>
    }
}
