// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ViewTimelineRounded)]
pub fn view_timeline_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ViewTimelineRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M11,17H7c-0.55,0-1-0.45-1-1v0 c0-0.55,0.45-1,1-1h4c0.55,0,1,0.45,1,1v0C12,16.55,11.55,17,11,17z M14,13h-4c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h4 c0.55,0,1,0.45,1,1v0C15,12.55,14.55,13,14,13z M17,9h-4c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h4c0.55,0,1,0.45,1,1v0 C18,8.55,17.55,9,17,9z"/>
        </SvgIcon>
    }
}
