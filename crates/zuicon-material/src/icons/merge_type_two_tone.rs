// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MergeTypeTwoTone)]
pub fn merge_type_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MergeTypeTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M5.59 19L7 20.41l6-6V8h3.5L12 3.5 7.5 8H11v5.59zm11.407 1.41l-3.408-3.407 1.4-1.407 3.41 3.408z"/>
        </SvgIcon>
    }
}
