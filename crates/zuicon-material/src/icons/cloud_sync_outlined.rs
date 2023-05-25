// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CloudSyncOutlined)]
pub fn cloud_sync_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CloudSyncOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21.5,14.98c-0.02,0-0.03,0-0.05,0.01C21.2,13.3,19.76,12,18,12c-1.4,0-2.6,0.83-3.16,2.02C13.26,14.1,12,15.4,12,17 c0,1.66,1.34,3,3,3l6.5-0.02c1.38,0,2.5-1.12,2.5-2.5S22.88,14.98,21.5,14.98z M21.51,18L21.51,18L15,18c-0.55,0-1-0.45-1-1 s0.45-1,1-1h1.25v-0.25c0-0.97,0.78-1.75,1.75-1.75s1.75,0.78,1.75,1.75V17c0,0,1.75,0,1.76,0c0.28,0,0.5,0.22,0.5,0.5 C22,17.77,21.78,18,21.51,18z M10,4.26v2.09C7.67,7.18,6,9.39,6,12c0,1.77,0.78,3.34,2,4.44V14h2v6H4v-2h2.73 C5.06,16.54,4,14.4,4,12C4,8.27,6.55,5.15,10,4.26z M20,6h-2.73c1.43,1.26,2.41,3.01,2.66,5l-2.02,0 C17.68,9.64,16.98,8.45,16,7.56V10h-2V4h6V6z"/>
        </SvgIcon>
    }
}
