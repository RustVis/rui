// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FolderOffRounded)]
pub fn folder_off_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FolderOffRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2.81,2.81c-0.39-0.39-1.02-0.39-1.41,0C1,3.2,1,3.83,1.39,4.22l0.85,0.85C2.1,5.35,2.01,5.66,2.01,6L2,18 c0,1.1,0.9,2,2,2h13.17l2.61,2.61c0.39,0.39,1.02,0.39,1.41,0c0.39-0.39,0.39-1.02,0-1.41L2.81,2.81z"/><path d="M20,6h-8l-1.41-1.41C10.21,4.21,9.7,4,9.17,4H6.83l14.93,14.93C21.91,18.65,22,18.34,22,18V8C22,6.9,21.1,6,20,6z"/>
        </SvgIcon>
    }
}
