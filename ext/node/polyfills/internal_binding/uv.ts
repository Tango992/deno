// Copyright 2018-2026 the Deno authors. MIT license.
// Copyright Joyent, Inc. and other Node contributors.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to permit
// persons to whom the Software is furnished to do so, subject to the
// following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN
// NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
// USE OR OTHER DEALINGS IN THE SOFTWARE.

// This module ports:
// - https://github.com/nodejs/node/blob/master/src/uv.cc
// - https://github.com/nodejs/node/blob/master/deps/uv
//
// See also: http://docs.libuv.org/en/v1.x/errors.html#error-constants

// TODO(petamoriken): enable prefer-primordials for node polyfills
// deno-lint-ignore-file prefer-primordials

import { osType } from "ext:deno_node/_util/os.ts";
import { uvTranslateSysError } from "ext:deno_node/internal_binding/_libuv_winerror.ts";
import { op_node_uv_error_map } from "ext:core/ops";

// The error map is built from Rust via libuv-compatible errno definitions,
// conditionally compiled per platform. This replaces the per-OS hardcoded
// arrays that were previously maintained in this file.
export const errorMap: Map<number, [string, string]> = op_node_uv_error_map();

// Reverse map: error name -> numeric code
export const codeMap = new Map<string, number>();
errorMap.forEach(([name], code) => {
  codeMap.set(name, code);
});

export function mapSysErrnoToUvErrno(sysErrno: number): number {
  if (osType === "windows") {
    const code = uvTranslateSysError(sysErrno);
    return codeMap.get(code) ?? -sysErrno;
  } else {
    return -sysErrno;
  }
}

export function errname(errno: number): string {
  const err = errorMap.get(errno);
  if (err) {
    return err[0];
  }
  return `UNKNOWN (${errno})`;
}

export function getErrorMessage(errno: number): string {
  const err = errorMap.get(errno);

  if (err) {
    return err[1];
  }
  return `UNKNOWN (${errno})`;
}

export function getErrorMap(): Map<number, [string, string]> {
  return errorMap;
}
