// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ScreenshotSharp)]
pub fn screenshot_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ScreenshotSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M5,1v22h14V1H5z M17,18H7V6h10V18z M9.5,8.5H12V7H8v4h1.5V8.5z M12,17h4v-4h-1.5v2.5H12V17z"/>
        </SvgIcon>
    }
}
