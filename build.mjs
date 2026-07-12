import { spawnSync } from "node:child_process";
import process from "node:process";
import os from "node:os";
import path from "node:path";

const isWindows = process.platform === "win32";
const cargoBin = path.join(os.homedir(), ".cargo", "bin");

process.env.PATH = `${cargoBin}${path.delimiter}${process.env.PATH ?? ""}`;

function run(command, args, options = {}) {
    console.log(`> ${command} ${args.join(" ")}`);

    const result = spawnSync(command, args, {
        stdio: "inherit",
        shell: isWindows,
        env: process.env,
        ...options,
    });

    if (result.error) {
        console.error(result.error);
        process.exit(1);
    }

    if (result.status !== 0) {
        process.exit(result.status ?? 1);
    }
}

function exists(command) {
    const result = spawnSync(command, ["--version"], {
        stdio: "ignore",
        shell: isWindows,
        env: process.env,
    });

    return result.status === 0;
}

// Cloudflare 构建环境没有 Rust 时安装
if (!exists("cargo")) {
    if (isWindows) {
        console.error("Rust is not installed. Install it from rustup.rs first.");
        process.exit(1);
    }

    run("sh", [
        "-c",
        "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal",
    ]);
}

run("rustup", ["target", "add", "wasm32-unknown-unknown"]);

if (!exists("worker-build")) {
    run("cargo", [
        "install",
        "worker-build",
        "--version",
        "^0.8",
        "--locked",
    ]);
}

run("worker-build", ["--release"]);

run("pnpm", ["install", "--frozen-lockfile"], {
    cwd: "src-vite",
});

run("pnpm", ["run", "build"], {
    cwd: "src-vite",
});