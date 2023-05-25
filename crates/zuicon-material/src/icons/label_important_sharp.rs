// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LabelImportantSharp)]
pub fn label_important_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LabelImportantSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M4 18.99h12.04L21 12l-4.97-7H4l5 7-5 6.99z"/>
        </SvgIcon>
    }
}
