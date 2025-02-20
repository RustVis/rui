// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FlakyRounded)]
pub fn flaky_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FlakyRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12.16,15.72c-0.29-0.29-0.29-0.77,0-1.06l0,0c0.29-0.29,0.77-0.29,1.06,0 l0.82,0.82l1.96-1.96c0.29-0.29,0.77-0.29,1.06,0l0,0c0.29,0.29,0.29,0.77,0,1.06l-2.65,2.65c-0.19,0.19-0.51,0.2-0.7,0 L12.16,15.72z M12,2C6.5,2,2,6.5,2,12s4.5,10,10,10s10-4.5,10-10S17.5,2,12,2z M7.87,6.81l0.88,0.88l0.88-0.88 c0.29-0.29,0.77-0.29,1.06,0l0,0c0.29,0.29,0.29,0.77,0,1.06L9.81,8.75l0.88,0.88c0.29,0.29,0.29,0.77,0,1.06l0,0 c-0.29,0.29-0.77,0.29-1.06,0L8.75,9.81l-0.88,0.88c-0.29,0.29-0.77,0.29-1.06,0l0,0c-0.29-0.29-0.29-0.77,0-1.06l0.88-0.88 L6.81,7.87c-0.29-0.29-0.29-0.77,0-1.06l0,0C7.1,6.51,7.57,6.51,7.87,6.81z M12,20c-2.2,0-4.2-0.9-5.7-2.3L17.7,6.3 C19.1,7.8,20,9.8,20,12C20,16.4,16.4,20,12,20z" fill-rule="evenodd"/>
        </SvgIcon>
    }
}
