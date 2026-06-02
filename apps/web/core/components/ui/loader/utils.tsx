/**
 * Copyright (c) 2023-present Plane Software, Inc. and contributors
 * SPDX-License-Identifier: AGPL-3.0-only
 * See the LICENSE file for details.
 */

export const getRandomInt = (min: number, max: number) => {
  const randomValues = new Uint32Array(1);
  crypto.getRandomValues(randomValues);
  return Math.floor((randomValues[0] / (0x100000000 + 1)) * (max - min + 1)) + min;
};

const getSecureRandomInt = (min: number, max: number) => getRandomInt(min, max);

export const getSecureRandomLength = (lengthArray: string[]) => {
  if (lengthArray.length === 0) return "";
  return `${lengthArray[getSecureRandomInt(0, lengthArray.length - 1)]}`;
};

export const getRandomLength = (lengthArray: string[]) => {
  const randomIndex = getSecureRandomInt(0, lengthArray.length - 1);
  return `${lengthArray[randomIndex]}`;
};

export { getSecureRandomInt, getSecureRandomLength };
