// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TranscribeRounded)]
pub fn transcribe_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TranscribeRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22.54,10.28c-0.34-0.82-0.34-1.72,0-2.54c0.19-0.45,0.1-0.96-0.24-1.3l-0.1-0.1c-0.56-0.56-1.51-0.44-1.88,0.26 c-0.8,1.48-0.79,3.24,0.03,4.79c0.37,0.69,1.31,0.83,1.86,0.27l0.1-0.1C22.65,11.23,22.73,10.72,22.54,10.28z M18.82,15.11 L18.82,15.11c0.4-0.4,0.46-1.02,0.13-1.48c-1.97-2.73-1.96-6.39,0.01-9.23c0.32-0.47,0.26-1.1-0.14-1.5l0,0 c-0.5-0.5-1.34-0.46-1.78,0.1c-2.73,3.54-2.73,8.36,0.02,12C17.49,15.56,18.33,15.61,18.82,15.11z M9,13c2.21,0,4-1.79,4-4 c0-2.21-1.79-4-4-4S5,6.79,5,9C5,11.21,6.79,13,9,13z M15.39,15.56C13.71,14.7,11.53,14,9,14c-2.53,0-4.71,0.7-6.39,1.56 C1.61,16.07,1,17.1,1,18.22L1,20c0,0.55,0.45,1,1,1h14c0.55,0,1-0.45,1-1l0-1.78C17,17.1,16.39,16.07,15.39,15.56z"/>
        </SvgIcon>
    }
}
