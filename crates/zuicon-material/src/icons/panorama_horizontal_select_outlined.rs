// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PanoramaHorizontalSelectOutlined)]
pub fn panorama_horizontal_select_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PanoramaHorizontalSelectOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,5.5c-3.89,0-6.95-0.84-8.69-1.43C2.67,3.85,2,4.33,2,5.02L2,19c0,0.68,0.66,1.17,1.31,0.95 C5.36,19.26,8.1,18.5,12,18.5c3.87,0,6.66,0.76,8.69,1.45C21.34,20.16,22,19.68,22,19l0-14c0-0.68-0.66-1.17-1.31-0.95 C18.66,4.73,15.86,5.5,12,5.5z"/>
        </SvgIcon>
    }
}
