// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ScoreboardRounded)]
pub fn scoreboard_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ScoreboardRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17.5,13.5H16v-3h1.5V13.5z M16,2c-0.55,0-1,0.45-1,1v1H9V3c0-0.55-0.45-1-1-1S7,2.45,7,3v1H4C2.9,4,2,4.9,2,6v12 c0,1.1,0.9,2,2,2h16c1.1,0,2-0.9,2-2V6c0-1.1-0.9-2-2-2h-3V3C17,2.45,16.55,2,16,2z M9.5,14.25C9.5,14.66,9.16,15,8.75,15H6 c-0.55,0-1-0.45-1-1v-1.5c0-0.55,0.45-1,1-1h2v-1H5.75C5.34,10.5,5,10.16,5,9.75S5.34,9,5.75,9H8.5c0.55,0,1,0.45,1,1v1.5 c0,0.55-0.45,1-1,1h-2v1h2.25C9.16,13.5,9.5,13.84,9.5,14.25z M19,14c0,0.55-0.45,1-1,1h-2.5c-0.55,0-1-0.45-1-1v-4 c0-0.55,0.45-1,1-1H18c0.55,0,1,0.45,1,1V14z M12.75,6.75c0,0.41-0.34,0.75-0.75,0.75s-0.75-0.34-0.75-0.75S11.59,6,12,6 S12.75,6.34,12.75,6.75z M12.75,10.25c0,0.41-0.34,0.75-0.75,0.75s-0.75-0.34-0.75-0.75S11.59,9.5,12,9.5S12.75,9.84,12.75,10.25z M12.75,13.75c0,0.41-0.34,0.75-0.75,0.75s-0.75-0.34-0.75-0.75S11.59,13,12,13S12.75,13.34,12.75,13.75z M12.75,17.25 c0,0.41-0.34,0.75-0.75,0.75s-0.75-0.34-0.75-0.75S11.59,16.5,12,16.5S12.75,16.84,12.75,17.25z"/>
        </SvgIcon>
    }
}
