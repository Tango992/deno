// Copyright 2018-2025 the Deno authors. MIT license.
import { core, primordials } from "ext:core/mod.js";
import { op_node_cp, op_node_cp_sync } from "ext:core/ops";
import {
  getValidatedPath,
  validateCpOptions,
} from "ext:deno_node/internal/fs/utils.mjs";
import { promisify } from "ext:deno_node/internal/util.mjs";
import {
  CallbackWithError,
  makeCallback,
} from "ext:deno_node/_fs/_fs_common.ts";
import { ERR_INVALID_RETURN_VALUE } from "ext:deno_node/internal/errors.ts";

const { isPromise } = core;
const { PromisePrototypeThen } = primordials;

export interface CopyOptionsBase {
  dereference: boolean;
  errorOnExist: boolean;
  force: boolean;
  mode: number;
  preserveTimestamps: boolean;
  recursive: boolean;
  verbatimSymlinks: boolean;
}

export interface CopySyncOptions extends CopyOptionsBase {
  filter?(source: string, destination: string): boolean;
}

export interface CopyOptions extends CopyOptionsBase {
  filter?(source: string, destination: string): boolean | Promise<boolean>;
}

export function cpSync(
  src: string | URL,
  dest: string | URL,
  options?: CopySyncOptions,
): void {
  options = validateCpOptions(options) as CopySyncOptions;
  const srcPath = getValidatedPath(src, "src").toString();
  const destPath = getValidatedPath(dest, "dest").toString();

  if (options.filter) {
    const shouldCopy = options.filter(srcPath, destPath);
    if (isPromise(shouldCopy)) {
      throw new ERR_INVALID_RETURN_VALUE("filter", "boolean", shouldCopy);
    }
    if (!shouldCopy) {
      return;
    }
  }

  op_node_cp_sync(srcPath, destPath);
}

export function cp(
  src: string | URL,
  dest: string | URL,
  callback: CallbackWithError,
): void;
export function cp(
  src: string | URL,
  dest: string | URL,
  options: CopyOptions,
  callback: CallbackWithError,
): void;
export function cp(
  src: string | URL,
  dest: string | URL,
  options?: CopyOptions | CallbackWithError,
  callback?: CallbackWithError,
) {
  if (typeof options === "function") {
    callback = options;
    options = undefined;
  }
  callback = makeCallback(callback);

  options = validateCpOptions(options);
  const srcPath = getValidatedPath(src, "src").toString();
  const destPath = getValidatedPath(dest, "dest").toString();

  PromisePrototypeThen(
    op_node_cp(
      srcPath,
      destPath,
    ),
    () => callback(null),
    callback,
  );
}

export const cpPromise = promisify(cp);
