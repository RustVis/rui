// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WallpaperRounded)]
pub fn wallpaper_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WallpaperRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M4 5c0-.55.45-1 1-1h5c.55 0 1-.45 1-1s-.45-1-1-1H4c-1.1 0-2 .9-2 2v6c0 .55.45 1 1 1s1-.45 1-1V5zm5.61 8.49l-2.96 3.7c-.26.33-.03.81.39.81H17c.41 0 .65-.47.4-.8l-2-2.67c-.2-.27-.6-.27-.8 0l-1.63 2.18-2.58-3.22c-.2-.25-.58-.25-.78 0zM17 8.5c0-.83-.67-1.5-1.5-1.5S14 7.67 14 8.5s.67 1.5 1.5 1.5S17 9.33 17 8.5zM20 2h-6c-.55 0-1 .45-1 1s.45 1 1 1h5c.55 0 1 .45 1 1v5c0 .55.45 1 1 1s1-.45 1-1V4c0-1.1-.9-2-2-2zm0 17c0 .55-.45 1-1 1h-5c-.55 0-1 .45-1 1s.45 1 1 1h6c1.1 0 2-.9 2-2v-6c0-.55-.45-1-1-1s-1 .45-1 1v5zM3 13c-.55 0-1 .45-1 1v6c0 1.1.9 2 2 2h6c.55 0 1-.45 1-1s-.45-1-1-1H5c-.55 0-1-.45-1-1v-5c0-.55-.45-1-1-1z"/>
        </SvgIcon>
    }
}
