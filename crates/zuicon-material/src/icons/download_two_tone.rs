// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DownloadTwoTone)]
pub fn download_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DownloadTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M13 9V5h-2v6H9.83L12 13.17 14.17 11H13z" opacity=".3"/><path d="M15 9V3H9v6H5l7 7 7-7h-4zm-3 4.17L9.83 11H11V5h2v6h1.17L12 13.17zM5 18h14v2H5z"/>
        </SvgIcon>
    }
}
