// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(UpcomingOutlined)]
pub fn upcoming_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("UpcomingOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17.6,10.81L16.19,9.4l3.56-3.55l1.41,1.41C21.05,7.29,17.6,10.81,17.6,10.81z M13,3h-2v5h2V3z M6.4,10.81L7.81,9.4 L4.26,5.84L2.84,7.26C2.95,7.29,6.4,10.81,6.4,10.81z M20,14h-3.42c-0.77,1.76-2.54,3-4.58,3s-3.81-1.24-4.58-3H4v5h16V14 M20,12 c1.1,0,2,0.9,2,2v5c0,1.1-0.9,2-2,2H4c-1.1,0-2-0.9-2-2v-5c0-1.1,0.9-2,2-2h5c0,1.66,1.34,3,3,3s3-1.34,3-3H20z"/>
        </SvgIcon>
    }
}
