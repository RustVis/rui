// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Man4Rounded)]
pub fn man_4_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Man4Rounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M13.75,7h-3.5C9.04,7,8.11,8.07,8.27,9.26L9.82,20.7c0.1,0.74,0.74,1.3,1.49,1.3h1.38c0.75,0,1.39-0.55,1.49-1.3 l1.56-11.44C15.89,8.07,14.96,7,13.75,7z"/>
        </SvgIcon>
    }
}
