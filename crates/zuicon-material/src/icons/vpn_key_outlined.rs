// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VpnKeyOutlined)]
pub fn vpn_key_outlined(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="VpnKeyOutlined"
            view_box={props.view_box.clone()}
            >
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M22 19h-6v-4h-2.68c-1.14 2.42-3.6 4-6.32 4-3.86 0-7-3.14-7-7s3.14-7 7-7c2.72 0 5.17 1.58 6.32 4H24v6h-2v4zm-4-2h2v-4h2v-2H11.94l-.23-.67C11.01 8.34 9.11 7 7 7c-2.76 0-5 2.24-5 5s2.24 5 5 5c2.11 0 4.01-1.34 4.71-3.33l.23-.67H18v4zM7 15c-1.65 0-3-1.35-3-3s1.35-3 3-3 3 1.35 3 3-1.35 3-3 3zm0-4c-.55 0-1 .45-1 1s.45 1 1 1 1-.45 1-1-.45-1-1-1z"/>
        </SvgIcon>
    }
}
