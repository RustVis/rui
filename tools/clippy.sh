#!/bin/bash

# Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
# Use of this source is governed by Lesser General Public License that can be found
# in the LICENSE file.

set -xe

cargo clippy --all-targets --fix --allow-dirty --allow-staged
