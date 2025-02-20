// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FormatStrikethroughSharp)]
pub fn format_strikethrough_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FormatStrikethroughSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M10 19h4v-3h-4v3zM5 4v3h5v3h4V7h5V4H5zM3 14h18v-2H3v2z"/>
        </SvgIcon>
    }
}
