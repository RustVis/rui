// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AutoFixHighSharp)]
pub fn auto_fix_high_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AutoFixHighSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18.41,9.83l-4.24-4.24L1.59,18.17l4.24,4.24L18.41,9.83z M14.21,11.21l-1.41-1.41l1.38-1.38l1.41,1.41L14.21,11.21z"/>
        </SvgIcon>
    }
}
