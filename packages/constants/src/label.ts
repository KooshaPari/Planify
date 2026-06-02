/**
 * Copyright (c) 2023-present Plane Software, Inc. and contributors
 * SPDX-License-Identifier: AGPL-3.0-only
 * See the LICENSE file for details.
 */

export const LABEL_COLOR_OPTIONS = [
  "#FF6900",
  "#FCB900",
  "#7BDCB5",
  "#00D084",
  "#8ED1FC",
  "#0693E3",
  "#ABB8C3",
  "#EB144C",
  "#F78DA7",
  "#9900EF",
];

const getRandomColorIndex = (length: number) => {
  const randomValues = new Uint32Array(1);
  crypto.getRandomValues(randomValues);
  return Math.floor((randomValues[0] / (0x100000000 + 1)) * length);
};

export const getRandomLabelColor = () => {
  return LABEL_COLOR_OPTIONS[getRandomColorIndex(LABEL_COLOR_OPTIONS.length)];
};
