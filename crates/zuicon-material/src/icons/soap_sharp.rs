// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SoapSharp)]
pub fn soap_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SoapSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M14.25,6C14.66,6,15,6.34,15,6.75S14.66,7.5,14.25,7.5S13.5,7.16,13.5,6.75S13.84,6,14.25,6 M14.25,4.5 C13.01,4.5,12,5.51,12,6.75S13.01,9,14.25,9s2.25-1.01,2.25-2.25S15.49,4.5,14.25,4.5L14.25,4.5z M20,5.5c0.28,0,0.5,0.22,0.5,0.5 S20.28,6.5,20,6.5S19.5,6.28,19.5,6S19.72,5.5,20,5.5 M20,4c-1.1,0-2,0.9-2,2s0.9,2,2,2s2-0.9,2-2S21.1,4,20,4L20,4z M18,2.5 C18,3.33,17.33,4,16.5,4C15.67,4,15,3.33,15,2.5S15.67,1,16.5,1C17.33,1,18,1.67,18,2.5z M1,12.68V23h18v-2.5h-7v-1h9V17h-9v-1h10 v-2.5H12v-1h8V10H8.86l1.88-3.3L9.12,5L1,12.68z"/>
        </SvgIcon>
    }
}
