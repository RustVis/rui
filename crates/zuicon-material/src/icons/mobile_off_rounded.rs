// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MobileOffRounded)]
pub fn mobile_off_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MobileOffRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M17 16.44L3.61 3.05c-.39-.39-1.02-.39-1.41 0-.39.39-.39 1.02 0 1.41L5 7.27V21c0 1.1.9 2 2 2h10c1.02 0 1.85-.77 1.98-1.75L20 22.27c.39.39 1.02.39 1.41 0s.39-1.02 0-1.41L19 18.44l-2-2zM7 19V9.27L16.73 19H7zM17 5v8.61l2 2V3c0-1.1-.9-2-2-2H7c-.71 0-1.33.37-1.68.93L8.39 5H17z"/>
        </SvgIcon>
    }
}
