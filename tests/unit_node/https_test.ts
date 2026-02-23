// Copyright 2018-2026 the Deno authors. MIT license.

import https from "node:https";
import tls from "node:tls";
import { assert, assertEquals, fail } from "../unit/test_util.ts";
import type { AddressInfo } from "node:net";

Deno.test("[node/https] Server.address() includes family property", async () => {
  const certFile = "tests/testdata/tls/localhost.crt";
  const keyFile = "tests/testdata/tls/localhost.key";

  // Test IPv4
  {
    const { promise, resolve } = Promise.withResolvers<void>();
    const server = https.createServer({
      cert: Deno.readTextFileSync(certFile),
      key: Deno.readTextFileSync(keyFile),
    }, (_req, res) => {
      res.end("ok");
    });

    server.listen(0, "127.0.0.1", () => {
      const addr = server.address() as AddressInfo;
      assertEquals(addr.address, "127.0.0.1");
      assertEquals(addr.family, "IPv4");
      assertEquals(typeof addr.port, "number");
      server.close(() => resolve());
    });

    await promise;
  }

  // Test IPv6
  {
    const { promise, resolve } = Promise.withResolvers<void>();
    const server = https.createServer({
      cert: Deno.readTextFileSync(certFile),
      key: Deno.readTextFileSync(keyFile),
    }, (_req, res) => {
      res.end("ok");
    });

    server.listen(0, "::1", () => {
      const addr = server.address() as AddressInfo;
      assertEquals(addr.address, "::1");
      assertEquals(addr.family, "IPv6");
      assertEquals(typeof addr.port, "number");
      server.close(() => resolve());
    });

    await promise;
  }
});

Deno.test({
  name:
    "request.socket.authorized is true when successfully requested to https server",
  async fn() {
    const server = Deno.serve({
      port: 0,
      cert: Deno.readTextFileSync("tests/testdata/tls/localhost.crt"),
      key: Deno.readTextFileSync("tests/testdata/tls/localhost.key"),
      onListen({ port }) {
        const req = https.request(`https://localhost:${port}`, (res) => {
          // deno-lint-ignore no-explicit-any
          assert((req.socket as any).authorized);
          res.destroy();
          server.shutdown();
        });
      },
    }, () => {
      return new Response("hi");
    });
    await server.finished;
  },
});

Deno.test({
  name: "[node/https] custom Agent with async oncreate via tls.connect",
  async fn() {
    const cert = Deno.readTextFileSync("tests/testdata/tls/localhost.crt");
    const key = Deno.readTextFileSync("tests/testdata/tls/localhost.key");

    const server = Deno.serve({
      port: 0,
      cert,
      key,
      onListen({ port }) {
        class CustomAgent extends https.Agent {
          // @ts-expect-error we know this is the correct signature for createConnection
          override createConnection(options, oncreate): net.Socket {
            const socket = tls.connect({
              port: options.port as number,
              host: options.host as string,
              servername: (options.servername || options.host) as string,
              rejectUnauthorized: false,
            });
            socket.on("connect", () => {
              oncreate(null, socket);
            });
          }
        }

        const agent = new CustomAgent();
        const req = https.request(
          `https://localhost:${port}`,
          { agent, rejectUnauthorized: false },
          (res) => {
            let data = "";
            res.on("data", (chunk: string) => data += chunk);
            res.on("end", async () => {
              assertEquals(res.statusCode, 200);
              assertEquals(data, "ok");
              await server.shutdown();
            });
          },
        );

        req.on("error", async (err) => {
          await server.shutdown();
          fail(`Request failed: ${err}`);
        });

        req.end();
      },
    }, () => {
      return new Response("ok");
    });

    await server.finished;
  },
});
