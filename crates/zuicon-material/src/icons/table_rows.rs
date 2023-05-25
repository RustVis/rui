// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TableRows)]
pub fn table_rows(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TableRows"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,8H3V4h18V8z M21,10H3v4h18V10z M21,16H3v4h18V16z"/>
        </SvgIcon>
    }
}
