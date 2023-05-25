// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ChaletRounded)]
pub fn chalet_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ChaletRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M10,15c-0.55,0-1,0.45-1,1v4H6c-0.55,0-1-0.45-1-1v-3.67l-0.38,0.38c-0.39,0.39-1.02,0.39-1.41,0l0,0 c-0.39-0.39-0.39-1.02,0-1.41l6.09-6.09c0.39-0.39,1.02-0.39,1.41,0l6.09,6.09c0.39,0.39,0.39,1.02,0,1.41v0 c-0.39,0.39-1.02,0.39-1.41,0L15,15.33V19c0,0.55-0.45,1-1,1h-3v-4C11,15.45,10.55,15,10,15z M17.5,7.5v0.89l-1.08,1.08 c-0.18,0.18-0.21,0.48-0.05,0.69c0.19,0.23,0.53,0.24,0.74,0.04l0.39-0.39v0.69c0,0.28,0.22,0.5,0.5,0.5h0c0.28,0,0.5-0.22,0.5-0.5 V9.81l0.39,0.39c0.21,0.21,0.55,0.19,0.74-0.04c0.17-0.2,0.14-0.5-0.05-0.69L18.5,8.39c0,0,0-0.89,0-0.89h0.89l1.08,1.08 c0.18,0.18,0.48,0.21,0.69,0.05c0.23-0.19,0.24-0.53,0.04-0.74L20.81,7.5h0.69C21.78,7.5,22,7.28,22,7v0c0-0.28-0.22-0.5-0.5-0.5 l-0.69,0l0.39-0.39c0.21-0.21,0.19-0.55-0.04-0.74c-0.2-0.17-0.5-0.14-0.69,0.05L19.39,6.5c0,0-0.89,0-0.89,0V5.61l1.08-1.08 c0.18-0.18,0.21-0.48,0.05-0.69c-0.19-0.23-0.53-0.24-0.74-0.04L18.5,4.19V3.5C18.5,3.22,18.28,3,18,3h0c-0.28,0-0.5,0.22-0.5,0.5 v0.69L17.11,3.8c-0.21-0.21-0.55-0.19-0.74,0.04c-0.17,0.2-0.14,0.5,0.05,0.69l1.08,1.08c0,0,0,0.89,0,0.89h-0.89l-1.08-1.08 c-0.18-0.18-0.48-0.21-0.69-0.05c-0.23,0.19-0.24,0.53-0.04,0.74l0.39,0.39H14.5C14.22,6.5,14,6.72,14,7v0c0,0.28,0.22,0.5,0.5,0.5 l0.69,0L14.8,7.89c-0.21,0.21-0.19,0.55,0.04,0.74c0.2,0.17,0.5,0.14,0.69-0.05l1.08-1.08C16.61,7.5,17.5,7.5,17.5,7.5z"/>
        </SvgIcon>
    }
}
