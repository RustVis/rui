// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LinkedCameraRounded)]
pub fn linked_camera_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LinkedCameraRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M16.6,2.37c2.1,0.27,3.77,1.93,4.03,4.03C20.67,6.74,20.95,7,21.29,7l0,0c0.39,0,0.71-0.34,0.66-0.73 c-0.33-2.72-2.5-4.89-5.22-5.22C16.34,1,16,1.32,16,1.71l0,0C16,2.05,16.26,2.33,16.6,2.37z"/><path d="M19.23,6.19C18.93,5,18,4.07,16.81,3.77C16.4,3.67,16,3.99,16,4.42l0,0c0,0.29,0.19,0.57,0.48,0.64 c0.72,0.18,1.29,0.74,1.46,1.46C18.01,6.81,18.28,7,18.58,7l0,0C19.01,7,19.33,6.6,19.23,6.19z"/><path d="M17,8c0-1.1-0.9-2-2-2V4c0-0.55-0.45-1-1-1H9.88C9.32,3,8.78,3.24,8.4,3.65L7.17,5H4C2.9,5,2,5.9,2,7v12 c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2v-9c0-1.1-0.9-2-2-2H17z M12,17.5c-2.48,0-4.5-2.02-4.5-4.5S9.52,8.5,12,8.5 s4.5,2.02,4.5,4.5S14.48,17.5,12,17.5z"/>
        </SvgIcon>
    }
}
