// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AddBusinessSharp)]
pub fn add_business_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AddBusinessSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M15,17h2v-3h1v-2l-1-5H2l-1,5v2h1v6h9v-6h4V17z M9,18H4v-4h5V18z"/>
        </SvgIcon>
    }
}
