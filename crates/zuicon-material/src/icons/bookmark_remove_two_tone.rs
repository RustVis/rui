// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BookmarkRemoveTwoTone)]
pub fn bookmark_remove_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BookmarkRemoveTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17,17.97l0-7.07c-2.28-0.46-4-2.48-4-4.9c0-0.34,0.03-0.68,0.1-1L7,5v12.97l5-2.14L17,17.97z" opacity=".3"/><path d="M21,7h-6V5h6V7z M17,17.97l-5-2.14l-5,2.14V5l6.1,0c0.15-0.74,0.46-1.42,0.9-2L7,3C5.9,3,5,3.9,5,5v16l7-3l7,3l0-10.1 c-0.32,0.07-0.66,0.1-1,0.1c-0.34,0-0.68-0.03-1-0.1L17,17.97z"/>
        </SvgIcon>
    }
}
