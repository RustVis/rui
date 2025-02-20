// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Paragliding)]
pub fn paragliding(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Paragliding"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,17c-1.1,0-2-0.9-2-2s0.9-2,2-2s2,0.9,2,2S13.1,17,12,17z M8.52,17.94C8.04,17.55,7,16.76,7,14H5 c0,2.7,0.93,4.41,2.3,5.5c0.5,0.4,1.1,0.7,1.7,0.9L9,24h6v-3.6c0.6-0.2,1.2-0.5,1.7-0.9c1.37-1.09,2.3-2.8,2.3-5.5h-2 c0,2.76-1.04,3.55-1.52,3.94C14.68,18.54,14,19,12,19S9.32,18.54,8.52,17.94z M12,0C5.92,0,1,1.9,1,4.25v3.49 C1,8.55,1.88,9,2.56,8.57C2.7,8.48,2.84,8.39,3,8.31L5,13h2l1.5-6.28C9.6,6.58,10.78,6.5,12,6.5s2.4,0.08,3.5,0.22L17,13h2l2-4.69 c0.16,0.09,0.3,0.17,0.44,0.26C22.12,9,23,8.55,23,7.74V4.25C23,1.9,18.08,0,12,0z M5.88,11.24L4.37,7.69 c0.75-0.28,1.6-0.52,2.53-0.71L5.88,11.24z M18.12,11.24L17.1,6.98c0.93,0.19,1.78,0.43,2.53,0.71L18.12,11.24z"/>
        </SvgIcon>
    }
}
