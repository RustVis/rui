// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BrowserUpdatedOutlined)]
pub fn browser_updated_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BrowserUpdatedOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,13v3c0,1.1-0.9,2-2,2h-3l1,1v2H6v-2l1-1H4c-1.1,0-2-0.9-2-2V5c0-1.1,0.9-2,2-2l8,0v2L4,5v11h16v-3H22z M15,15l-5-5 l1.41-1.41L14,11.17V3h2v8.17l2.59-2.58L20,10L15,15z"/>
        </SvgIcon>
    }
}
