/**
 * Copyright (c) 2023-present Plane Software, Inc. and contributors
 * SPDX-License-Identifier: AGPL-3.0-only
 * See the LICENSE file for details.
 */

import { logger } from "@plane/logger";
import { AppError } from "@/lib/errors";

export const logProcessError = (label: string, reason: unknown): AppError => {
  const error = new AppError(reason);
  logger.error(label, error);
  return error;
};
