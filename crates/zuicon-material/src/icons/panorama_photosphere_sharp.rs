// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PanoramaPhotosphereSharp)]
pub fn panorama_photosphere_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PanoramaPhotosphereSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M23,8.84c-0.54-0.43-1.23-0.81-1.99-1.15C19.4,4.33,15.98,2,12,2C8.02,2,4.6,4.33,2.99,7.68C2.23,8.03,1.54,8.4,1,8.84 v6.33c0.54,0.43,1.23,0.81,1.99,1.15C4.6,19.67,8.02,22,12,22c3.98,0,7.4-2.33,9.01-5.68c0.76-0.34,1.45-0.72,1.99-1.15V8.84z M12,4c2.37,0,4.49,1.04,5.95,2.68C16.17,6.25,14.15,6,12,6C9.85,6,7.83,6.25,6.05,6.68C7.51,5.04,9.63,4,12,4z M12,20 c-2.37,0-4.49-1.04-5.95-2.68C7.83,17.75,9.85,18,12,18s4.17-0.25,5.95-0.68C16.49,18.96,14.37,20,12,20z"/>
        </SvgIcon>
    }
}
