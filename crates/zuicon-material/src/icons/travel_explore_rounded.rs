// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TravelExploreRounded)]
pub fn travel_explore_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TravelExploreRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19.3,16.9c0.58-1.01,0.95-2.23,0.51-3.65c-0.53-1.72-2.04-3.05-3.84-3.22c-2.87-0.28-5.23,2.07-4.95,4.95 c0.18,1.79,1.5,3.31,3.22,3.84c1.43,0.44,2.64,0.07,3.65-0.51l2.5,2.5c0.39,0.39,1.01,0.39,1.4,0l0,0c0.39-0.39,0.39-1.01,0-1.4 L19.3,16.9z M15.5,17c-1.4,0-2.5-1.1-2.5-2.5s1.1-2.5,2.5-2.5s2.5,1.1,2.5,2.5S16.9,17,15.5,17z M12,20v2C6.48,22,2,17.52,2,12 C2,6.48,6.48,2,12,2c4.84,0,8.87,3.44,9.8,8h-2.07c-0.64-2.46-2.4-4.47-4.73-5.41V5c0,1.1-0.9,2-2,2h-2v2c0,0.55-0.45,1-1,1H8v2h2v3 H9l-4.79-4.79C4.08,10.79,4,11.38,4,12C4,16.41,7.59,20,12,20z"/>
        </SvgIcon>
    }
}
