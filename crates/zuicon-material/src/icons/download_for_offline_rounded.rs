// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DownloadForOfflineRounded)]
pub fn download_for_offline_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DownloadForOfflineRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,2C6.49,2,2,6.49,2,12s4.49,10,10,10s10-4.49,10-10S17.51,2,12,2z M11,10V7c0-0.55,0.45-1,1-1h0c0.55,0,1,0.45,1,1v3 h1.79c0.45,0,0.67,0.54,0.35,0.85l-2.79,2.79c-0.2,0.2-0.51,0.2-0.71,0l-2.79-2.79C8.54,10.54,8.76,10,9.21,10H11z M16,17H8 c-0.55,0-1-0.45-1-1v0c0-0.55,0.45-1,1-1h8c0.55,0,1,0.45,1,1v0C17,16.55,16.55,17,16,17z"/>
        </SvgIcon>
    }
}
