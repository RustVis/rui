// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SetMealRounded)]
pub fn set_meal_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SetMealRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20.3,17.6L3.83,18.46c-0.41,0.02-0.77-0.3-0.79-0.71l0,0c-0.02-0.41,0.3-0.77,0.71-0.79l16.48-0.86 c0.41-0.02,0.77,0.3,0.79,0.71v0C21.04,17.22,20.72,17.58,20.3,17.6z M20.25,19.48H3.75C3.34,19.48,3,19.82,3,20.23l0,0 c0,0.41,0.34,0.75,0.75,0.75h16.5c0.41,0,0.75-0.34,0.75-0.75l0,0C21,19.82,20.66,19.48,20.25,19.48z M22,5v7c0,1.1-0.9,2-2,2H4 c-1.1,0-2-0.9-2-2V5c0-1.1,0.9-2,2-2h16C21.1,3,22,3.9,22,5z M19.12,6.09c-1.25,0.27-2.19,1.11-2.33,2.14 C16.15,7.5,14.06,5.5,10.25,5.5c-3.44,0-5.48,1.63-6.31,2.49c-0.28,0.29-0.28,0.74,0,1.03c0.83,0.86,2.87,2.49,6.31,2.49 c3.81,0,5.9-2,6.54-2.73c0.14,1.02,1.08,1.86,2.33,2.14c0.46,0.1,0.88-0.28,0.88-0.74V6.84C20,6.37,19.57,5.99,19.12,6.09z"/>
        </SvgIcon>
    }
}
