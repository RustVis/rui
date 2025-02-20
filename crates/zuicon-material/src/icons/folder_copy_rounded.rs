// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FolderCopyRounded)]
pub fn folder_copy_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FolderCopyRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2,6L2,6C1.45,6,1,6.45,1,7v12c0,1.1,0.9,2,2,2h16c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H3V7C3,6.45,2.55,6,2,6z"/><path d="M21,4h-7l-1.41-1.41C12.21,2.21,11.7,2,11.17,2H7C5.9,2,5.01,2.9,5.01,4L5,15c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V6 C23,4.9,22.1,4,21,4z"/>
        </SvgIcon>
    }
}
