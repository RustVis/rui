// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(OfflineShareRounded)]
pub fn offline_share_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("OfflineShareRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M5,5L5,5C4.45,5,4,5.45,4,6v15c0,1.1,0.9,2,2,2h9c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H6V6C6,5.45,5.55,5,5,5z"/><path d="M18,1h-8C8.9,1,8,1.9,8,3v14c0,1.1,0.9,2,2,2h8c1.1,0,2-0.9,2-2V3C20,1.9,19.1,1,18,1z M18,15h-8V5h8V15z"/><path d="M12.5,10.25h2v0.54c0,0.45,0.54,0.67,0.85,0.35l1.29-1.29c0.2-0.2,0.2-0.51,0-0.71l-1.29-1.29 c-0.31-0.31-0.85-0.09-0.85,0.35v0.54H12c-0.55,0-1,0.45-1,1v1.5c0,0.41,0.34,0.75,0.75,0.75h0c0.41,0,0.75-0.34,0.75-0.75V10.25 z"/>
        </SvgIcon>
    }
}
