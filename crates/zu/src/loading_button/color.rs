// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use crate::styles::color::Color;

pub const fn root_class(color: Color) -> &'static str {
    match color {
        Color::Primary => "ZuLoadingButton-colorPrimary",
        Color::Secondary => "ZuLoadingButton-colorSecondary",
        Color::Success => "ZuLoadingButton-colorSuccess",
        Color::Info => "ZuLoadingButton-colorInfo",
        Color::Warning => "ZuLoadingButton-colorWarning",
        Color::Error => "ZuLoadingButton-colorError",
        Color::Inherit => "ZuLoadingButton-colorInherit",
        Color::Default => "",
    }
}
