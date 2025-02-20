// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BorderColorSharp)]
pub fn border_color_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BorderColorSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,24H2v-4h20V24z M13.06,5.19l3.75,3.75L7.75,18H4v-3.75L13.06,5.19z M17.88,7.87 l-3.75-3.75l2.53-2.54l3.75,3.75L17.88,7.87z" enable-background="new"/>
        </SvgIcon>
    }
}
