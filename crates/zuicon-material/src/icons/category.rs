// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Category)]
pub fn category(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Category"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0z" fill="none"/><path d="M12 2l-5.5 9h11z"/><path d="M3 13.5h8v8H3z"/>
        </SvgIcon>
    }
}
