// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AssignmentLateRounded)]
pub fn assignment_late_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AssignmentLateRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M19,3h-4.18C14.4,1.84,13.3,1,12,1S9.6,1.84,9.18,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5 C21,3.9,20.1,3,19,3z M12,2.75c0.41,0,0.75,0.34,0.75,0.75c0,0.41-0.34,0.75-0.75,0.75s-0.75-0.34-0.75-0.75 C11.25,3.09,11.59,2.75,12,2.75z M12,13L12,13c-0.55,0-1-0.45-1-1V8c0-0.55,0.45-1,1-1h0c0.55,0,1,0.45,1,1v4 C13,12.55,12.55,13,12,13z M13,16c0,0.55-0.45,1-1,1s-1-0.45-1-1c0-0.55,0.45-1,1-1S13,15.45,13,16z"/>
        </SvgIcon>
    }
}
