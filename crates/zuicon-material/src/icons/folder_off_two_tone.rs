// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FolderOffTwoTone)]
pub fn folder_off_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FolderOffTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0.69,3.51l1.56,1.56C2.1,5.35,2.01,5.66,2.01,6L2,18c0,1.1,0.9,2,2,2h13.17l3.31,3.31l1.41-1.41L2.1,2.1L0.69,3.51z M15.17,18H4V6.83L15.17,18z"/><path d="M20,6h-8l-2-2H7.17l4,4H20v9.17l1.76,1.76C21.91,18.65,22,18.34,22,18V8C22,6.9,21.1,6,20,6z"/>
        </SvgIcon>
    }
}
