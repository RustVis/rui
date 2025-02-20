// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SignalCellularNullOutlined)]
pub fn signal_cellular_null_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SignalCellularNullOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M20 6.83V20H6.83L20 6.83M22 2L2 22h20V2z"/>
        </SvgIcon>
    }
}
