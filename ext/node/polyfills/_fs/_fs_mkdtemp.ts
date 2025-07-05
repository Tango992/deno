// Copyright 2018-2025 the Deno authors. MIT license.
// Copyright Node.js contributors. All rights reserved. MIT License.

import { TextDecoder, TextEncoder } from "ext:deno_web/08_text_encoding.js";
import { existsSync } from "ext:deno_node/_fs/_fs_exists.ts";
import { mkdir, mkdirSync } from "ext:deno_node/_fs/_fs_mkdir.ts";
import { ERR_INVALID_ARG_TYPE } from "ext:deno_node/internal/errors.ts";
import { promisify } from "ext:deno_node/internal/util.mjs";
import { primordials } from "ext:core/mod.js";
import { getOptions } from "ext:deno_node/internal/fs/utils.mjs";

const {
  Array,
  SafeArrayIterator,
  MathRandom,
  MathFloor,
  ArrayPrototypeJoin,
  ArrayPrototypeMap,
} = primordials;

export type mkdtempCallback = (
  err: Error | null,
  directory?: string,
) => void;

// https://nodejs.org/dist/latest-v15.x/docs/api/fs.html#fs_fs_mkdtemp_prefix_options_callback
export function mkdtemp(prefix: string, callback: mkdtempCallback): void;
export function mkdtemp(
  prefix: string,
  options: { encoding: string } | string,
  callback: mkdtempCallback,
): void;
export function mkdtemp(
  prefix: string,
  optionsOrCallback: { encoding: string } | string | mkdtempCallback,
  maybeCallback?: mkdtempCallback,
) {
  const callback: mkdtempCallback | undefined =
    typeof optionsOrCallback == "function" ? optionsOrCallback : maybeCallback;
  if (!callback) {
    throw new ERR_INVALID_ARG_TYPE("callback", "function", callback);
  }

  const encoding = getOptions(optionsOrCallback).encoding;
  const path = tempDirPath(prefix);

  mkdir(
    path,
    { recursive: false, mode: 0o700 },
    (err: Error | null | undefined) => {
      if (err) callback(err);
      else callback(null, decode(path, encoding));
    },
  );
}

export const mkdtempPromise = promisify(mkdtemp) as (
  prefix: string,
  options?: { encoding: string } | string,
) => Promise<string>;

// https://nodejs.org/dist/latest-v15.x/docs/api/fs.html#fs_fs_mkdtempsync_prefix_options
export function mkdtempSync(
  prefix: string,
  options?: { encoding: string } | string,
): string {
  const encoding = getOptions(options).encoding;
  const path = tempDirPath(prefix);

  mkdirSync(path, { recursive: false, mode: 0o700 });
  return decode(path, encoding);
}

function decode(str: string, encoding?: string): string {
  if (!encoding) return str;
  else {
    const decoder = new TextDecoder(encoding);
    const encoder = new TextEncoder();
    return decoder.decode(encoder.encode(str));
  }
}

const CHARS = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
function randomName(): string {
  return ArrayPrototypeJoin(
    ArrayPrototypeMap(
      [...new SafeArrayIterator(Array(6))],
      () => CHARS[MathFloor(MathRandom() * CHARS.length)],
    ),
    "",
  );
}

function tempDirPath(prefix: string): string {
  let path: string;
  do {
    path = prefix + randomName();
  } while (existsSync(path));

  return path;
}
