# SPDX-FileCopyrightText: 2023-present Plane Software, Inc.
# SPDX-License-Identifier: LicenseRef-Plane-Commercial
#
# Licensed under the Plane Commercial License (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
# https://plane.so/legals/eula
#
# DO NOT remove or modify this notice.
# NOTICE: Proprietary and confidential. Unauthorized use or distribution is prohibited.

# Python imports
import re

# Django imports
from django.utils.html import strip_tags


def generate_plain_text_from_html(html_content):
    """
    Generate clean plain text from HTML email template.
    Removes all HTML tags, CSS styles, and excessive whitespace.

    Args:
        html_content (str): The HTML content to convert to plain text

    Returns:
        str: Clean plain text without HTML tags, styles, or excessive whitespace
    """
    # Remove style tags and their content
    style_open = html_content.lower().find("<style")
    while style_open != -1:
        style_close = html_content.lower().find("</style>", style_open)
        if style_close == -1:
            break
        html_content = html_content[:style_open] + html_content[style_close + len("</style>") :]
        style_open = html_content.lower().find("<style")

    # Strip HTML tags
    text_content = strip_tags(html_content)

    # Remove excessive empty lines
    while "\n\n\n" in text_content:
        text_content = text_content.replace("\n\n\n", "\n\n")

    # Ensure there's a leading and trailing whitespace
    text_content = "\n\n" + text_content.lstrip().rstrip() + "\n\n"

    return text_content
