// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HvacOutlined)]
pub fn hvac_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HvacOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,3H5C3.9,3,3,3.9,3,5v14c0,1.1,0.9,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.1,3,19,3z M19,19H5V5h14V19z"/><path d="M12,18c3.31,0,6-2.69,6-6s-2.69-6-6-6s-6,2.69-6,6S8.69,18,12,18z M15.44,10c0.26,0.45,0.44,0.96,0.51,1.5h-7.9 c0.07-0.54,0.24-1.05,0.51-1.5H15.44z M15.95,12.5c-0.07,0.54-0.24,1.05-0.51,1.5H8.56c-0.26-0.45-0.44-0.96-0.51-1.5H15.95z M9.38,15h5.24c-0.7,0.61-1.61,1-2.62,1S10.09,15.61,9.38,15z M14.62,9H9.38c0.7-0.61,1.61-1,2.62-1S13.91,8.39,14.62,9z"/>
        </SvgIcon>
    }
}
