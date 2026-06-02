# Copyright (c) 2023-present Plane Software, Inc. and contributors
# SPDX-License-Identifier: AGPL-3.0-only
# See the LICENSE file for details.

import random
import string

import secrets


def get_random_color():
    """
    Get a random color in hex format
    """
    secure_random = secrets.SystemRandom()
    return "#" + "".join(secure_random.choice("0123456789abcdef") for _ in range(6))
