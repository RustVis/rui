// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RoomServiceOutlined)]
pub fn room_service_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("RoomServiceOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M18.98 17H2v2h20v-2zM21 16c-.27-4.07-3.25-7.4-7.16-8.21.1-.24.16-.51.16-.79 0-1.1-.9-2-2-2s-2 .9-2 2c0 .28.06.55.16.79C6.25 8.6 3.27 11.93 3 16h18zm-9-6.42c2.95 0 5.47 1.83 6.5 4.41h-13c1.03-2.58 3.55-4.41 6.5-4.41z"/>
        </SvgIcon>
    }
}
