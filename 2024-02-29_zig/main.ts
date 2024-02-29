import { serveDir } from "https://deno.land/std@0.217.0/http/file_server.ts"

const dir = await Deno.makeTempDir();

await Deno.copyFile("index.html", `${dir}/index.html`);
await Deno.copyFile("zig-out/lib/main.wasm", `${dir}/main.wasm`);


Deno.serve(8000, (request) => {
    return serveDir(request, {
        fsRoot: dir,
    });
});

