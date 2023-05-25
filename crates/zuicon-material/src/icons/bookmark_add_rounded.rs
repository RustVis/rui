// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BookmarkAddRounded)]
pub fn bookmark_add_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BookmarkAddRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,6c0,0.55-0.45,1-1,1h-1v1c0,0.55-0.45,1-1,1s-1-0.45-1-1V7h-1c-0.55,0-1-0.45-1-1s0.45-1,1-1h1V4c0-0.55,0.45-1,1-1 s1,0.45,1,1v1h1C20.55,5,21,5.45,21,6z M19,19.48c0,0.72-0.73,1.2-1.39,0.92L12,18l-5.61,2.4C5.73,20.69,5,20.2,5,19.48V5 c0-1.1,0.9-2,2-2l7,0c-0.63,0.84-1,1.87-1,3c0,2.76,2.24,5,5,5c0.34,0,0.68-0.03,1-0.1V19.48z"/>
        </SvgIcon>
    }
}
