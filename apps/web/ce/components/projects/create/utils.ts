/**
 * Copyright (c) 2023-present Plane Software, Inc. and contributors
 * SPDX-License-Identifier: AGPL-3.0-only
 * See the LICENSE file for details.
 */

import { RANDOM_EMOJI_CODES } from "@plane/constants";
import type { IProject } from "@plane/types";
import { getRandomCoverImage } from "@/helpers/cover-image.helper";

const getRandomEmojiIndex = () => {
  const randomValues = new Uint32Array(1);
  crypto.getRandomValues(randomValues);
  return Math.floor((randomValues[0] / (0x100000000 + 1)) * RANDOM_EMOJI_CODES.length);
};

export const getProjectFormValues = (): Partial<IProject> => ({
  cover_image_url: getRandomCoverImage(),
  description: "",
  logo_props: {
    in_use: "emoji",
    emoji: {
      value: RANDOM_EMOJI_CODES[getRandomEmojiIndex()],
    },
  },
  identifier: "",
  name: "",
  network: 2,
  project_lead: null,
});
