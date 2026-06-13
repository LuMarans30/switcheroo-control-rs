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

### 🚜 Refactor

- *(common)* Remove dead code ([`dea3e51`](https://github.com/LuMarans30/switcheroo-control-rs/commit/dea3e5134134252befcc08061d215dfea1a4ae10))
- *(common)* Cleaner Display for GpuDevice ([`7df571a`](https://github.com/LuMarans30/switcheroo-control-rs/commit/7df571abfbcf72be0419fdfb105841da511e5783))
- *(common)* Use manual zvariant impls and clean EnvVar API ([`1ec2eea`](https://github.com/LuMarans30/switcheroo-control-rs/commit/1ec2eeae016937758c996d2072b43fc631d3aec6))
- *(daemon)* Improve already-running daemon error message ([`783c5f8`](https://github.com/LuMarans30/switcheroo-control-rs/commit/783c5f85f73bae2c46145bba29f23ebe4e1edb3b))
- *(daemon)* Extract udev and hardware event loops ([`74383d6`](https://github.com/LuMarans30/switcheroo-control-rs/commit/74383d684a658f8d4a7947729c4e80dcb8541dbe))
- *(cli)* Improve function names ([`c97630d`](https://github.com/LuMarans30/switcheroo-control-rs/commit/c97630d4d7f7d554ef12305dc0af1fa12478ee2e))
- *(daemon)* Use PropertiesChanged for D-Bus signal emission ([`114dd9a`](https://github.com/LuMarans30/switcheroo-control-rs/commit/114dd9a2ef60d7e6156dae6f99d625c104652355))
- *(cleanup)* Use LazyLock for static initialization ([`b4173ae`](https://github.com/LuMarans30/switcheroo-control-rs/commit/b4173ae1025c22379995a1a664c00e939e791069))
- *(common)* Split lib.rs into env_var and gpu_device modules ([`41c0ad0`](https://github.com/LuMarans30/switcheroo-control-rs/commit/41c0ad02e3554286d73be20aa55250c898d419d4))
- *(daemon)* Use matches! for daemon running check ([`f0c0457`](https://github.com/LuMarans30/switcheroo-control-rs/commit/f0c04570ae829bfbfa6e9d9cc4c0e63a9f7fe3c4))

### 📚 Documentation

- *(readme)* Improve README ([`4621f17`](https://github.com/LuMarans30/switcheroo-control-rs/commit/4621f17652c91c8984868f0c3cea033412dfd421))
- *(readme)* Add WIP to title ([`88da80b`](https://github.com/LuMarans30/switcheroo-control-rs/commit/88da80bef9aec27dc7dc56fa58f44a220961bd16))
- *(detection)* Explain why unsafe FFI call is sound ([`f024722`](https://github.com/LuMarans30/switcheroo-control-rs/commit/f024722430d5effd4f76917edeeb2f038ff71ca3))
- *(readme)* Update following major changes ([`4fd37c2`](https://github.com/LuMarans30/switcheroo-control-rs/commit/4fd37c2ddf727aad18d49210f38442b01a799aaf))
- *(readme)* Add architectural Mermaid diagram ([`512cd85`](https://github.com/LuMarans30/switcheroo-control-rs/commit/512cd8586903415c82f88fe0b97baa015e1091f6))

### ⚡ Performance

- *(detection)* Optimize discrete GPU detection with direct udev FFI ([`eedfd97`](https://github.com/LuMarans30/switcheroo-control-rs/commit/eedfd974c1539591b150ad09a11212e78a92e6a7))

### ⚙️ Miscellaneous Tasks

- Add rust-clippy analysis workflow ([`92b9c77`](https://github.com/LuMarans30/switcheroo-control-rs/commit/92b9c775571af8bce04a68a67cbaaa009475edea))
