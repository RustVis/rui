// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BookmarkBorderSharp)]
pub fn bookmark_border_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BookmarkBorderSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M19 3H5v18l7-3 7 3V3zm-2 15l-5-2.18L7 18V5h10v13z"/>
        </SvgIcon>
    }
}
