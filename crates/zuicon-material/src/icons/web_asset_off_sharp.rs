// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WebAssetOffSharp)]
pub fn web_asset_off_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WebAssetOffSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6.83,4H22v15.17l-2-2V8h-9.17L6.83,4z M20.49,23.31L17.17,20H2V4.83L0.69,3.51L2.1,2.1l19.8,19.8L20.49,23.31z M15.17,18 l-10-10H4v10H15.17z"/>
        </SvgIcon>
    }
}
