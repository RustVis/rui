// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(GpsOffRounded)]
pub fn gps_off_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("GpsOffRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M22 13c.55 0 1-.45 1-1s-.45-1-1-1h-1.06c-.46-4.17-3.77-7.48-7.94-7.94V2c0-.55-.45-1-1-1s-1 .45-1 1v1.06c-.98.11-1.91.38-2.77.78l1.53 1.53C10.46 5.13 11.22 5 12 5c3.87 0 7 3.13 7 7 0 .79-.13 1.54-.37 2.24l1.53 1.53c.4-.86.67-1.79.78-2.77H22zm-1.56 5.88L5.12 3.56c-.39-.39-1.02-.39-1.41 0-.39.39-.39 1.02 0 1.41L5.04 6.3C3.97 7.62 3.26 9.23 3.06 11H2c-.55 0-1 .45-1 1s.45 1 1 1h1.06c.46 4.17 3.77 7.48 7.94 7.94V22c0 .55.45 1 1 1s1-.45 1-1v-1.06c1.77-.2 3.38-.91 4.69-1.98l1.33 1.33c.39.39 1.02.39 1.41 0 .4-.39.4-1.02.01-1.41zM12 19c-3.87 0-7-3.13-7-7 0-1.61.55-3.09 1.46-4.27l9.81 9.81C15.09 18.45 13.61 19 12 19z"/>
        </SvgIcon>
    }
}
