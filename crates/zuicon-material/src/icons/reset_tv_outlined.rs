// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ResetTvOutlined)]
pub fn reset_tv_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ResetTvOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M22,8V5c0-1.1-0.9-2-2-2H4C2.9,3,2,3.9,2,5v12c0,1.1,0.9,2,2,2h4v2h8v-2h4c1.1,0,1.99-0.9,1.99-2v-5H22c0-1.1-0.9-2-2-2 h-7.17l1.83-1.83l-1.41-1.41C9.69,10.31,10.88,9.12,9,11c2.06,2.06,0.9,0.9,4.24,4.24l1.41-1.41L12.83,12H20v5H4V5h16v3H22z"/>
        </SvgIcon>
    }
}
