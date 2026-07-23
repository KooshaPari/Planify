/**
 * Copyright (c) 2023-present Plane Software, Inc. and contributors
 * SPDX-License-Identifier: AGPL-3.0-only
 * See the LICENSE file for details.
 */

import { beforeEach, describe, expect, it, vi } from "vitest";
import { AppError } from "@/lib/errors";

const loggerError = vi.hoisted(() => vi.fn());

vi.mock("@plane/logger", () => ({
  logger: {
    error: loggerError,
  },
}));

const { logProcessError } = await import("@/lib/process-errors");

describe("logProcessError", () => {
  beforeEach(() => {
    loggerError.mockClear();
  });

  it("normalizes unknown rejection reasons into AppError instances", () => {
    const error = logProcessError("[UNHANDLED_REJECTION]", { code: "E_FAIL" });

    expect(error).toBeInstanceOf(AppError);
    expect(error.message).toBe("Unknown error occurred");
    expect(loggerError).toHaveBeenCalledWith("[UNHANDLED_REJECTION]", error);
  });

  it("preserves existing AppError instances without wrapping them again", () => {
    const original = new AppError("already normalized");

    const error = logProcessError("[UNCAUGHT_EXCEPTION]", original);

    expect(error).toBe(original);
    expect(loggerError).toHaveBeenCalledWith("[UNCAUGHT_EXCEPTION]", original);
  });
});
