// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AirlineStopsRounded)]
pub fn airline_stops_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AirlineStopsRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M15,18c0,0.55-0.45,1-1,1h-4c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1h1c-0.47-4.21-3.89-7.55-8.12-7.96 C2.37,8.99,2,8.56,2,8.05c0-0.59,0.52-1.06,1.11-1C7.03,7.44,10.37,9.87,12,13.3c1.13-2.43,2.99-4.25,4.78-5.52l-1.92-1.92 C14.54,5.54,14.76,5,15.21,5h5.29C20.78,5,21,5.22,21,5.5v5.29c0,0.45-0.54,0.67-0.85,0.35l-1.94-1.94C15.93,10.78,13.45,13.3,13,17 h1C14.55,17,15,17.45,15,18z"/>
        </SvgIcon>
    }
}
