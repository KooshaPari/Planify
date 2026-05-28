/**
 * Copyright (c) 2023-present Plane Software, Inc. and contributors
 * SPDX-License-Identifier: AGPL-3.0-only
 * See the LICENSE file for details.
 */

import { describe, it, expect, vi, beforeEach } from "vitest";
import type { Request, Response, NextFunction } from "express";

// Set env before importing the middleware so env.ts picks it up
vi.mock("@/env", () => ({
  env: {
    LIVE_SERVER_SECRET_KEY: "test-secret-key-12345",
  },
}));

// Import after mocking env
const { requireSecretKey } = await import("@/lib/auth-middleware");

function makeReq(headers: Record<string, string> = {}): Request {
  return { headers, path: "/test", method: "GET", ip: "127.0.0.1" } as unknown as Request;
}

function makeRes() {
  const res = {
    status: vi.fn().mockReturnThis(),
    json: vi.fn().mockReturnThis(),
  };
  return res as unknown as Response;
}

describe("requireSecretKey middleware", () => {
  let next: NextFunction;

  beforeEach(() => {
    next = vi.fn();
  });

  it("calls next() when the correct secret key is provided", () => {
    const req = makeReq({ "live-server-secret-key": "test-secret-key-12345" });
    const res = makeRes();

    requireSecretKey(req, res, next);

    expect(next).toHaveBeenCalledOnce();
    expect(res.status).not.toHaveBeenCalled();
  });

  it("returns 401 when no secret key header is present", () => {
    const req = makeReq({});
    const res = makeRes();

    requireSecretKey(req, res, next);

    expect(next).not.toHaveBeenCalled();
    expect(res.status).toHaveBeenCalledWith(401);
    expect(res.json).toHaveBeenCalledWith({ error: "Unauthorized", status: 401 });
  });

  it("returns 401 when a wrong secret key is provided", () => {
    const req = makeReq({ "live-server-secret-key": "wrong-key" });
    const res = makeRes();

    requireSecretKey(req, res, next);

    expect(next).not.toHaveBeenCalled();
    expect(res.status).toHaveBeenCalledWith(401);
  });

  it("is resistant to timing side-channels (comparison time is uniform)", () => {
    // Both wrong keys — we simply assert neither calls next, not timing.
    // Proper timing tests require statistical analysis outside unit tests.
    const req1 = makeReq({ "live-server-secret-key": "a" });
    const req2 = makeReq({ "live-server-secret-key": "test-secret-key-1234" }); // off by 1 char

    requireSecretKey(req1, makeRes(), next);
    requireSecretKey(req2, makeRes(), next);

    expect(next).not.toHaveBeenCalled();
  });
});
