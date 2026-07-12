import { spawnSync } from "node:child_process";
import process from "node:process";

const isWindows = process.platform === "win32";

function run(command, args, options = {}) {
    console.log(`> ${command} ${args.join(" ")}`);

    const result = spawnSync(command, args, {
        stdio: "inherit",
        shell: isWindows,
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

run("rustup", ["target", "add", "wasm32-unknown-unknown"]);

const check = spawnSync("worker-build", ["--version"], {
    stdio: "ignore",
    shell: isWindows,
});

if (check.status !== 0) {
    run("cargo", [
        "install",
        "worker-build",
        "--version",
        "^0.8",
        "--locked",
    ]);
}

run("worker-build", ["--release"]);

run(
    "pnpm",
    ["install", "--frozen-lockfile"],
    {
        cwd: "src-vite",
    },
);

run(
    "pnpm",
    ["run", "build"],
    {
        cwd: "src-vite",
    },
);