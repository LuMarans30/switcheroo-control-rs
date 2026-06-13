## [0.1.0] - 2026-06-13

### 🚀 Features

- *(drm)* Add ioctl probes for discrete GPU detection ([`0526456`](https://github.com/LuMarans30/switcheroo-control-rs/commit/0526456398694b7ef83a8c02d4cb88796f6fa84c))
- *(daemon)* Handle SIGINT/SIGTERM gracefully ([`b62ce65`](https://github.com/LuMarans30/switcheroo-control-rs/commit/b62ce655d2f7570af8c5f95f7e5de787cb284d5a))
- *(cli)* Add implicit subcommands ([`11cc438`](https://github.com/LuMarans30/switcheroo-control-rs/commit/11cc4381dec2cb13d7961b8b0d809a89b28e98ef))
- *(daemon)* Add AllowReplacement flag to match upstream ([`39d2f2b`](https://github.com/LuMarans30/switcheroo-control-rs/commit/39d2f2bd398e39708778c4f9750d121feb1b24b0))
- *(daemon)* Implement daemon replacement and optimize udev handling ([`31286d0`](https://github.com/LuMarans30/switcheroo-control-rs/commit/31286d0731de013e8dac446801d0bbfd90b2d2ab))
- *(common)* Add YesNo newtype for boolean formatting ([`40bcbb1`](https://github.com/LuMarans30/switcheroo-control-rs/commit/40bcbb1b2c907e18a992198bea3b714dddc445d1))
- *(i915)* Implement ioctl probe and fix misclassification ([`99795de`](https://github.com/LuMarans30/switcheroo-control-rs/commit/99795ded211ccc65a6fb7f58cf7582734b8f81f2))

### 🐛 Bug Fixes

- *(detection)* Filter by render nodes instead of card nodes ([`fd19efc`](https://github.com/LuMarans30/switcheroo-control-rs/commit/fd19efcd616edc88560e9a36f2e50e6f82eb06d3))
- *(common)* Correct zvariant properties renaming ([`1a8b3d3`](https://github.com/LuMarans30/switcheroo-control-rs/commit/1a8b3d345ea422ff01cdec703f11488a408fed91))
- *(detection)* Skip GPUs with empty env to match upstream ([`48b0c21`](https://github.com/LuMarans30/switcheroo-control-rs/commit/48b0c212ceddccb3e83d237a5400f9c27065bc4e))
- *(detection)* Improve default card fallback logic ([`c5e642d`](https://github.com/LuMarans30/switcheroo-control-rs/commit/c5e642db1dd88f8032f24c10337d79be600e46f4))
- *(daemon)* Prevent D-Bus signal flooding ([`f01fb62`](https://github.com/LuMarans30/switcheroo-control-rs/commit/f01fb62670b93844d57c0c132d5219d2eb1a36ef))
- *(detection)* Avoid panic on empty cards list ([`e492d48`](https://github.com/LuMarans30/switcheroo-control-rs/commit/e492d480eb79bd2c6ec22d3b1c7b70c10b212907))
- *(xe)* Correct Xe probe reading wrong config field ([`3983c89`](https://github.com/LuMarans30/switcheroo-control-rs/commit/3983c895e936e4b4e1ed61688e79061cec106709))
- *(daemon)* Improve graceful exit for already-running daemon ([`83db86a`](https://github.com/LuMarans30/switcheroo-control-rs/commit/83db86a704dd8251afa5ce563a0b5aedb4e9c807))
- *(cli)* Handle missing switcheroo daemon gracefully ([`0889c04`](https://github.com/LuMarans30/switcheroo-control-rs/commit/0889c04f31941111163d87a8ffefe1fbf93c4c5a))
- *(daemon)* Exit 1 when daemon is already running ([`38a9806`](https://github.com/LuMarans30/switcheroo-control-rs/commit/38a98060db6af431474a9a282464cbe30eaf0348))
- *(cli)* Let exec() handle executable errors ([`8a5ea11`](https://github.com/LuMarans30/switcheroo-control-rs/commit/8a5ea1169699e7d79a78eed10191765006f45177))
- *(amdgpu)* Correct ioctl direction flags for device info query ([`b068c53`](https://github.com/LuMarans30/switcheroo-control-rs/commit/b068c537a687c9980eb9519870b1ff4ee5abcf48))
- *(cleanup)* Add GTX to regex ([`1bf862b`](https://github.com/LuMarans30/switcheroo-control-rs/commit/1bf862bb69518809948ca0e78077fe69bc393627))
- *(daemon)* Minor bug fixes to amdgpu and startup logic ([`6266db7`](https://github.com/LuMarans30/switcheroo-control-rs/commit/6266db71ef13773a50fc7578b140eaf81e1291d1))

### ⚡ Performance

- *(detection)* Optimize discrete GPU detection with direct udev FFI ([`eedfd97`](https://github.com/LuMarans30/switcheroo-control-rs/commit/eedfd974c1539591b150ad09a11212e78a92e6a7))

### ⚙️ Miscellaneous Tasks

- Add rust-clippy analysis workflow ([`92b9c77`](https://github.com/LuMarans30/switcheroo-control-rs/commit/92b9c775571af8bce04a68a67cbaaa009475edea))
