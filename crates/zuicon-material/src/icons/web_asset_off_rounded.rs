// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WebAssetOffRounded)]
pub fn web_asset_off_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WebAssetOffRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6.83,4H20c1.11,0,2,0.9,2,2v12c0,0.34-0.09,0.66-0.23,0.94L20,17.17V8h-9.17L6.83,4z M19.78,22.61L17.17,20H4 c-1.11,0-2-0.9-2-2V6c0-0.34,0.08-0.66,0.23-0.94L1.39,4.22C1,3.83,1,3.2,1.39,2.81l0,0c0.39-0.39,1.02-0.39,1.41,0l18.38,18.38 c0.39,0.39,0.39,1.02,0,1.41l0,0C20.8,23,20.17,23,19.78,22.61z M15.17,18l-10-10H4v10H15.17z"/>
        </SvgIcon>
    }
}
