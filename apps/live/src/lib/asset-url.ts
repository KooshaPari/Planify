/**
 * Copyright (c) 2023-present Plane Software, Inc. and contributors
 * SPDX-License-Identifier: AGPL-3.0-only
 * See the LICENSE file for details.
 */

const STORAGE_HOST_PATTERNS = [
  /^[a-z0-9.-]+\.s3[.-][a-z0-9-]+\.amazonaws\.com$/i,
  /^s3[.-][a-z0-9-]+\.amazonaws\.com$/i,
  /^[a-z0-9.-]+\.digitaloceanspaces\.com$/i,
  /^storage\.googleapis\.com$/i,
];

/**
 * Allow fetches only to the API host or known object-storage hostnames.
 */
export function isAllowedAssetFetchUrl(url: string, apiBaseUrl: string): boolean {
  try {
    const parsed = new URL(url);
    if (parsed.protocol !== "https:" && parsed.protocol !== "http:") {
      return false;
    }

    const host = parsed.hostname.toLowerCase();
    const apiHost = new URL(apiBaseUrl).hostname.toLowerCase();

    if (host === apiHost) {
      return true;
    }

    return STORAGE_HOST_PATTERNS.some((pattern) => pattern.test(host));
  } catch {
    return false;
  }
}
