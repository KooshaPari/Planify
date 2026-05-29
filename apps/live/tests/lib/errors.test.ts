/**
 * Copyright (c) 2023-present Plane Software, Inc. and contributors
 * SPDX-License-Identifier: AGPL-3.0-only
 * See the LICENSE file for details.
 */

import { describe, it, expect } from "vitest";
import type { AxiosError, AxiosResponse } from "axios";
import { AppError } from "@/lib/errors";

function makeAxiosError(overrides: Partial<AxiosError> & { responseData?: unknown } = {}): AxiosError {
  const { responseData, ...rest } = overrides;
  return {
    isAxiosError: true,
    message: "Network Error",
    name: "AxiosError",
    config: { method: "get", url: "/api/test" },
    response: responseData !== undefined
      ? ({
          data: responseData,
          status: 400,
          statusText: "Bad Request",
          headers: {},
          config: {},
        } as unknown as AxiosResponse)
      : undefined,
    ...rest,
  } as unknown as AxiosError;
}

describe("AppError", () => {
  describe("string constructor", () => {
    it("creates AppError with the given message", () => {
      const err = new AppError("something went wrong");
      expect(err).toBeInstanceOf(AppError);
      expect(err.message).toBe("something went wrong");
      expect(err.name).toBe("AppError");
    });

    it("merges optional data fields", () => {
      const err = new AppError("bad request", { statusCode: 400, code: "BAD_REQUEST" });
      expect(err.statusCode).toBe(400);
      expect(err.code).toBe("BAD_REQUEST");
    });
  });

  describe("AxiosError constructor", () => {
    it("extracts message from response body when present", () => {
      const axiosErr = makeAxiosError({ responseData: { message: "Page not found" } });
      const err = new AppError(axiosErr);
      expect(err.message).toBe("Page not found");
      expect(err.statusCode).toBe(400);
    });

    it("falls back to axiosError.message when response body has no message", () => {
      const axiosErr = makeAxiosError({ responseData: { detail: "some detail" } });
      const err = new AppError(axiosErr);
      expect(err.message).toBe("Network Error");
    });

    it("falls back to axiosError.message when response body is a non-object (string)", () => {
      const axiosErr = makeAxiosError({ responseData: "plain text error" });
      const err = new AppError(axiosErr);
      // responseData is a string, not an object — should use axiosError.message
      expect(err.message).toBe("Network Error");
    });

    it("falls back to axiosError.message when there is no response", () => {
      const axiosErr = makeAxiosError();
      const err = new AppError(axiosErr);
      expect(err.message).toBe("Network Error");
      expect(err.statusCode).toBeUndefined();
    });

    it("sets method, url, and code from axiosError config", () => {
      const axiosErr = makeAxiosError({ code: "ECONNREFUSED" });
      const err = new AppError(axiosErr);
      expect(err.method).toBe("GET");
      expect(err.url).toBe("/api/test");
      expect(err.code).toBe("ECONNREFUSED");
    });
  });

  describe("DOMException (AbortError) constructor", () => {
    it("sets code to ABORT_ERROR", () => {
      const domErr = new DOMException("Aborted", "AbortError");
      const err = new AppError(domErr);
      expect(err.code).toBe("ABORT_ERROR");
    });
  });

  describe("standard Error constructor", () => {
    it("wraps a standard Error preserving message and code", () => {
      const stdErr = new TypeError("invalid type");
      const err = new AppError(stdErr);
      expect(err.message).toBe("invalid type");
      expect(err.code).toBe("TypeError");
    });
  });

  describe("AppError passthrough", () => {
    it("returns the same AppError instance unchanged", () => {
      const original = new AppError("original");
      const result = new AppError(original);
      expect(result).toBe(original);
    });
  });

  describe("unknown error fallback", () => {
    it("uses a safe fallback message for truly unknown types", () => {
      const err = new AppError(42);
      expect(err.message).toBe("Unknown error occurred");
    });
  });
});
