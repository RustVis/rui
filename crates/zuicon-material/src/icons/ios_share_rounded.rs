// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(IosShareRounded)]
pub fn ios_share_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("IosShareRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18,8h-2c-0.55,0-1,0.45-1,1v0c0,0.55,0.45,1,1,1h2v11H6V10h2c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H6c-1.1,0-2,0.9-2,2 v11c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V10C20,8.9,19.1,8,18,8z"/><path d="M12,16L12,16c0.55,0,1-0.45,1-1V5h1.79c0.45,0,0.67-0.54,0.35-0.85l-2.79-2.79c-0.2-0.2-0.51-0.2-0.71,0L8.85,4.15 C8.54,4.46,8.76,5,9.21,5H11v10C11,15.55,11.45,16,12,16z"/>
        </SvgIcon>
    }
}
