/**
 * Copyright (c) 2023-present Plane Software, Inc. and contributors
 * SPDX-License-Identifier: AGPL-3.0-only
 * See the LICENSE file for details.
 */

import { LUCIDE_ICONS_LIST } from "..";

const getRandomIconIndex = (length: number) => {
  const randomValues = new Uint32Array(1);
  crypto.getRandomValues(randomValues);
  return Math.floor((randomValues[0] / (0x100000000 + 1)) * length);
};

/**
 * Returns a random icon name from the LUCIDE_ICONS_LIST array
 */
export const getRandomIconName = (): string =>
  LUCIDE_ICONS_LIST[getRandomIconIndex(LUCIDE_ICONS_LIST.length)].name;
