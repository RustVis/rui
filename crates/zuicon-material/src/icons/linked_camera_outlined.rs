// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LinkedCameraOutlined)]
pub fn linked_camera_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LinkedCameraOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M20 9v11H4V8h4.05l1.83-2H15V4H9L7.17 6H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V9h-2zm.67-1.01H22C21.99 4.68 19.31 2 16 2v1.33c2.58 0 4.66 2.08 4.67 4.66zm-2.67 0h1.33c-.01-1.84-1.49-3.32-3.33-3.32V6c1.11 0 1.99.89 2 1.99zM7 14c0 2.76 2.24 5 5 5s5-2.24 5-5-2.24-5-5-5-5 2.24-5 5zm8 0c0 1.65-1.35 3-3 3s-3-1.35-3-3 1.35-3 3-3 3 1.34 3 3z"/>
        </SvgIcon>
    }
}
