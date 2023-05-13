// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

/// The size of the component.
///
/// If using a number, the pixel unit is assumed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Size {
    XSmall,
    Small,
    Middle,
    Large,
    XLarge,

    Num(i32),
}

impl Default for Size {
    fn default() -> Self {
        Self::Middle
    }
}
