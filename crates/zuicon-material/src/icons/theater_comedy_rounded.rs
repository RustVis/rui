// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TheaterComedyRounded)]
pub fn theater_comedy_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TheaterComedyRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,2h-8c-1.1,0-2,0.9-2,2v3.5h1.5c1.1,0,2,0.9,2,2v4.95c1.04,0.48,2.24,0.68,3.5,0.47c2.93-0.49,5-3.17,5-6.14V4 C23,2.9,22.1,2,21,2z M14,6.5c0-0.55,0.45-1,1-1s1,0.45,1,1s-0.45,1-1,1S14,7.05,14,6.5z M18.85,10.88h-3.72 c-0.38,0-0.63-0.41-0.44-0.75C15.08,9.47,15.96,9,16.99,9s1.91,0.47,2.3,1.14C19.48,10.47,19.23,10.88,18.85,10.88z M19,7.5 c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1S19.55,7.5,19,7.5z"/><path d="M11,9H3c-1.1,0-2,0.9-2,2v4.79c0,3.05,2.19,5.77,5.21,6.16C9.87,22.42,13,19.57,13,16v-5C13,9.9,12.1,9,11,9z M4,13.5 c0-0.55,0.45-1,1-1s1,0.45,1,1s-0.45,1-1,1S4,14.05,4,13.5z M9.3,16.75c-0.38,0.67-1.27,1.14-2.3,1.14s-1.91-0.47-2.3-1.14 C4.51,16.41,4.76,16,5.14,16h3.72C9.24,16,9.49,16.41,9.3,16.75z M9,14.5c-0.55,0-1-0.45-1-1s0.45-1,1-1s1,0.45,1,1 S9.55,14.5,9,14.5z"/>
        </SvgIcon>
    }
}
