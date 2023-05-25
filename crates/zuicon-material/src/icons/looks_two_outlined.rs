// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LooksTwoOutlined)]
pub fn looks_two_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LooksTwoOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14zm-4-4h-4v-2h2c1.1 0 2-.89 2-2V9c0-1.11-.9-2-2-2H9v2h4v2h-2c-1.1 0-2 .89-2 2v4h6v-2z"/>
        </SvgIcon>
    }
}
