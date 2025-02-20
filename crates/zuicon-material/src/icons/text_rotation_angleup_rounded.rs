// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TextRotationAngleupRounded)]
pub fn text_rotation_angleup_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TextRotationAngleupRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0zm0 0h24v24H0V0zm0 0h24v24H0V0z" fill="none"/><path d="M17.61 9.85l.56.56-8.48 8.49c-.39.39-.39 1.02 0 1.41.39.39 1.02.39 1.41 0l8.49-8.49.56.56c.31.32.85.1.85-.34V9.5c0-.28-.22-.5-.5-.5h-2.54c-.44 0-.66.54-.35.85zm-9.13 2.9l3.54-3.54 1.6.67c.36.15.77.07 1.05-.21.49-.49.32-1.31-.32-1.57L5.26 4.5c-.43-.16-.91-.06-1.23.26-.32.32-.42.8-.25 1.23l3.61 9.09c.25.64 1.08.81 1.57.32.28-.28.36-.69.21-1.05l-.69-1.6zm-.82-1.72L5.43 6.16l4.87 2.23-2.64 2.64z"/>
        </SvgIcon>
    }
}
