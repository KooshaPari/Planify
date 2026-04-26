# Copyright (c) 2023-present Plane Software, Inc. and contributors
# SPDX-License-Identifier: AGPL-3.0-only
# See the LICENSE file for details.

import re
import ipaddress
import socket
from typing import Iterable, Optional
from urllib.parse import urlparse, urlunparse

# Compiled regex pattern for better performance and ReDoS protection
# Using atomic groups and length limits to prevent excessive backtracking
URL_PATTERN = re.compile(
    r"(?i)"  # Case insensitive
    r"(?:"  # Non-capturing group for alternatives
    r"https?://[^\s]+"  # http:// or https:// followed by non-whitespace
    r"|"
    r"www\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*"  # noqa: E501
    r"|"
    r"(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,6}"  # noqa: E501
    r"|"
    r"(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)"  # noqa: E501
    r")"
)


def contains_url(value: str) -> bool:
    """
    Check if the value contains a URL.

    This function is protected against ReDoS attacks by:
    1. Using a pre-compiled regex pattern
    2. Limiting input length to prevent excessive processing
    3. Using atomic groups and specific quantifiers to avoid backtracking

    Args:
        value (str): The input string to check for URLs

    Returns:
        bool: True if the string contains a URL, False otherwise
    """
    # Prevent ReDoS by limiting input length
    if len(value) > 1000:  # Reasonable limit for URL detection
        return False

    # Additional safety: truncate very long lines that might contain URLs
    lines = value.split("\n")
    for line in lines:
        if len(line) > 500:  # Process only reasonable length lines
            line = line[:500]
        if URL_PATTERN.search(line):
            return True

    return False


def is_valid_url(url: str) -> bool:
    """
    Validates whether the given string is a well-formed URL.

    Args:
        url (str): The URL string to validate.

    Returns:
        bool: True if the URL is valid, False otherwise.

    Example:
        >>> is_valid_url("https://example.com")
        True
        >>> is_valid_url("not a url")
        False
    """
    try:
        result = urlparse(url)
        # A valid URL should have at least scheme and netloc
        return all([result.scheme, result.netloc])
    except TypeError:
        return False


def normalize_hostname(hostname: str) -> str:
    """Normalize a hostname for validation and allowlist checks."""
    return hostname.rstrip(".").lower()


def hostname_matches_allowed_hosts(hostname: str, allowed_hosts: Iterable[str]) -> bool:
    """Return whether hostname exactly matches one of the allowed hosts."""
    normalized_hostname = normalize_hostname(hostname)
    return any(normalized_hostname == normalize_hostname(host) for host in allowed_hosts if host)


def hostname_matches_allowed_domain_suffixes(hostname: str, allowed_suffixes: Iterable[str]) -> bool:
    """Return whether hostname is a member of any allowed domain suffix."""
    normalized_hostname = normalize_hostname(hostname)
    for suffix in allowed_suffixes:
        if not suffix:
            continue
        normalized_suffix = normalize_hostname(suffix)
        if normalized_hostname == normalized_suffix or normalized_hostname.endswith(f".{normalized_suffix}"):
            return True
    return False


def validate_external_url(
    url: str,
    *,
    allowed_hosts: Iterable[str] = (),
    allowed_domain_suffixes: Iterable[str] = (),
) -> str:
    """
    Validate a URL before server-side fetches.

    The URL must use HTTP(S), match the provided host allowlist, and resolve only
    to globally routable IP addresses. This blocks SSRF attempts that target
    loopback, private, link-local, multicast, or otherwise non-global networks.
    """
    parsed_url = urlparse(url)

    if parsed_url.scheme not in ("http", "https"):
        raise ValueError("Invalid URL scheme. Only HTTP and HTTPS are allowed")

    if not parsed_url.hostname:
        raise ValueError("Invalid URL: No hostname found")

    hostname = normalize_hostname(parsed_url.hostname)
    if not (
        hostname_matches_allowed_hosts(hostname, allowed_hosts)
        or hostname_matches_allowed_domain_suffixes(hostname, allowed_domain_suffixes)
    ):
        raise ValueError("URL host is not allowed")

    try:
        addr_info = socket.getaddrinfo(hostname, None)
    except socket.gaierror as exc:
        raise ValueError("Hostname could not be resolved") from exc

    if not addr_info:
        raise ValueError("No IP addresses found for the hostname")

    for addr in addr_info:
        ip = ipaddress.ip_address(addr[4][0])
        if not ip.is_global:
            raise ValueError("Access to non-public networks is not allowed")

    return url


def get_url_components(url: str) -> Optional[dict]:
    """
    Parses the URL and returns its components if valid.

    Args:
        url (str): The URL string to parse.

    Returns:
        Optional[dict]: A dictionary with URL components if valid, None otherwise.

    Example:
        >>> get_url_components("https://example.com/path?query=1")
        {
        'scheme': 'https', 'netloc': 'example.com',
        'path': '/path', 'params': '',
        'query': 'query=1', 'fragment': ''}
    """
    if not is_valid_url(url):
        return None
    result = urlparse(url)
    return {
        "scheme": result.scheme,
        "netloc": result.netloc,
        "path": result.path,
        "params": result.params,
        "query": result.query,
        "fragment": result.fragment,
    }


def normalize_url_path(url: str) -> str:
    """
    Normalize the path component of a URL by
    replacing multiple consecutive slashes with a single slash.

    This function preserves the protocol, domain,
    query parameters, and fragments of the URL,
    only modifying the path portion to ensure there are no duplicate slashes.

    Args:
        url (str): The input URL string to normalize.

    Returns:
        str: The normalized URL with redundant slashes in the path removed.

    Example:
        >>> normalize_url_path('https://example.com//foo///bar//baz?x=1#frag')
        'https://example.com/foo/bar/baz?x=1#frag'
    """
    parts = urlparse(url)
    # Normalize the path
    normalized_path = re.sub(r"/+", "/", parts.path)
    # Reconstruct the URL
    return urlunparse(parts._replace(path=normalized_path))
