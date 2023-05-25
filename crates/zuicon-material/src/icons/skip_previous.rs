// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SkipPrevious)]
pub fn skip_previous(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SkipPrevious"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0z" fill="none"/><path d="M6 6h2v12H6zm3.5 6l8.5 6V6z"/>
        </SvgIcon>
    }
}
