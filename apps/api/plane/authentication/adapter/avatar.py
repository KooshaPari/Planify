# Copyright (c) 2023-present Plane Software, Inc. and contributors
# SPDX-License-Identifier: AGPL-3.0-only
# See the LICENSE file for details.

from urllib.parse import urljoin, urlparse

import requests

from plane.utils.url import validate_external_url


MAX_AVATAR_REDIRECTS = 3

PROVIDER_AVATAR_HOSTS = {
    "github": ("avatars.githubusercontent.com",),
    "gitlab": ("secure.gravatar.com", "www.gravatar.com"),
}

PROVIDER_AVATAR_DOMAIN_SUFFIXES = {
    "google": ("googleusercontent.com",),
    "github": ("githubusercontent.com",),
    "gitlab": ("gravatar.com",),
}


def _hostname_from_url(url: str | None) -> str | None:
    if not url:
        return None
    return urlparse(url).hostname


def get_avatar_url_policy(provider: str, provider_urls: tuple[str | None, ...] = ()) -> tuple[set[str], set[str]]:
    """Build the exact-host and suffix policy for provider avatar downloads."""
    allowed_hosts = set(PROVIDER_AVATAR_HOSTS.get(provider, ()))
    allowed_suffixes = set(PROVIDER_AVATAR_DOMAIN_SUFFIXES.get(provider, ()))

    for url in provider_urls:
        hostname = _hostname_from_url(url)
        if hostname:
            allowed_hosts.add(hostname)

    return allowed_hosts, allowed_suffixes


def validate_avatar_url(provider: str, avatar_url: str, provider_urls: tuple[str | None, ...] = ()) -> str:
    """Validate an OAuth avatar URL before fetching it from the server."""
    allowed_hosts, allowed_suffixes = get_avatar_url_policy(provider, provider_urls)
    return validate_external_url(
        avatar_url,
        allowed_hosts=allowed_hosts,
        allowed_domain_suffixes=allowed_suffixes,
    )


def get_validated_avatar_response(
    provider: str,
    avatar_url: str,
    *,
    provider_urls: tuple[str | None, ...] = (),
    headers: dict | None = None,
    timeout: int = 10,
) -> requests.Response:
    """Fetch a provider avatar after validating the initial URL and each redirect."""
    current_url = validate_avatar_url(provider, avatar_url, provider_urls)

    for _ in range(MAX_AVATAR_REDIRECTS + 1):
        response = requests.get(
            current_url,
            timeout=timeout,
            headers=headers or {},
            stream=True,
            allow_redirects=False,
        )

        if not response.is_redirect:
            return response

        redirect_url = response.headers.get("Location")
        if not redirect_url:
            return response

        current_url = validate_avatar_url(provider, urljoin(current_url, redirect_url), provider_urls)

    raise ValueError("Too many avatar download redirects")
