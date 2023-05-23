// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use crate::styles::color::Color;

#[must_use]
pub const fn color_class(color: &Color) -> &'static str {
    match color {
        Color::Primary => "ZuBadge-colorPrimary",
        Color::Secondary => "ZuBadge-colorSecondary",
        Color::Error => "ZuBadge-colorError",
        Color::Info => "ZuBadge-colorInfo",
        Color::Warning => "ZuBadge-colorWarning",
        Color::Success => "ZuBadge-colorSuccess",
        _ => "",
    }
}
