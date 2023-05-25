// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FastfoodRounded)]
pub fn fastfood_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FastfoodRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M21.9 5H18V2c0-.55-.45-1-1-1s-1 .45-1 1v3h-3.9c-.59 0-1.05.51-1 1.1l.12 1.21C14.9 8.16 18 10.77 18 15l.02 8h1.7c.84 0 1.53-.65 1.63-1.47L22.89 6.1c.06-.59-.4-1.1-.99-1.1zM15 21H2c-.55 0-1 .45-1 1s.45 1 1 1h13c.55 0 1-.45 1-1s-.45-1-1-1zM2.1 15h12.8c.62 0 1.11-.56.99-1.16-.65-3.23-4.02-4.85-7.39-4.85s-6.73 1.62-7.39 4.85c-.12.6.38 1.16.99 1.16zM15 17H2c-.55 0-1 .45-1 1s.45 1 1 1h13c.55 0 1-.45 1-1s-.45-1-1-1z"/>
        </SvgIcon>
    }
}
