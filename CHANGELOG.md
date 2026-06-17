## [0.1.3] - 2026-06-17

### 🚀 Features

- Add systemd and dbus files ([`de8c60e`](https://github.com/LuMarans30/switcheroo-control-rs/commit/de8c60e3ff9efb9ffb32f0f47c52c19de96c1abf))

### 🐛 Bug Fixes

- *(daemon)* Handle probing errors ([`ccf451b`](https://github.com/LuMarans30/switcheroo-control-rs/commit/ccf451ba579994fc69d8750ebda9b3e7ec6c87e6))
- *(client)* Handle SwitcherooProxy DESTINATION error better ([`44bf8fe`](https://github.com/LuMarans30/switcheroo-control-rs/commit/44bf8fee9ea23b85604f60272e35f50d4109f62c))
- *(daemon)* Continue run_udev_monitor when channel is full ([`0d1a354`](https://github.com/LuMarans30/switcheroo-control-rs/commit/0d1a3549ebc59a408c654b81f4b267dd4d86c2ed))
- *(client)* Return error if the first argument is not set ([`233fbc3`](https://github.com/LuMarans30/switcheroo-control-rs/commit/233fbc3eb06d637ab46b8e7d6668e08158988eda))
- Remove comment from D-Bus conf ([`6251703`](https://github.com/LuMarans30/switcheroo-control-rs/commit/6251703c1d6fb5f08b5d24bedd0c693ccf42589d))

### ⚡ Performance

- *(daemon)* Allocate DISCRETE_TAG only once ([`4dd092d`](https://github.com/LuMarans30/switcheroo-control-rs/commit/4dd092d11cc7398ca2c37b178213cd220ff20c1d))

### ⚙️ Miscellaneous Tasks

- *(release)* Update cliff.toml and CHANGELOG ([`7b4831b`](https://github.com/LuMarans30/switcheroo-control-rs/commit/7b4831bbf4316438347b2fa8a73cf051346ad461))
- *(client)* Simplify args code ([`1c95cd8`](https://github.com/LuMarans30/switcheroo-control-rs/commit/1c95cd839d0a7377d3092581aff22a0c0be5c6d0))
- *(common)* Improve code readability ([`8a844e5`](https://github.com/LuMarans30/switcheroo-control-rs/commit/8a844e5e9b54f0c5fef2e63255d4f6640bf0bf14))
## [0.1.2] - 2026-06-13

### ⚙️ Miscellaneous Tasks

- *(readme)* Add concise READMEs for workspace members ([`cc7c5db`](https://github.com/LuMarans30/switcheroo-control-rs/commit/cc7c5dbf5708f30093dfa0e225727af98db1cc39))
## [0.1.1] - 2026-06-13

### 🚀 Features

- *(cli,daemon)* Add version command and about description ([`889b14a`](https://github.com/LuMarans30/switcheroo-control-rs/commit/889b14a6c5f483979a75fe336e7b8e7ff107eb55))

### 🐛 Bug Fixes

- *(ci)* Replace deprecated toolchain actions ([`a02058e`](https://github.com/LuMarans30/switcheroo-control-rs/commit/a02058e6097881f203040c26e49d61fbf2672855))
- *(ci)* Upgrade checkout action to v6 ([`1b3de90`](https://github.com/LuMarans30/switcheroo-control-rs/commit/1b3de9002e608ded5123d532c6474c425c1fbaca))

### ⚙️ Miscellaneous Tasks

- *(release)* Add cliff.toml and CHANGELOG.md ([`772aff7`](https://github.com/LuMarans30/switcheroo-control-rs/commit/772aff74e0d99742dbdf607dcb4e3ce9707c07b0))
- *(release)* Add commit SHA links to changelog template ([`fa84400`](https://github.com/LuMarans30/switcheroo-control-rs/commit/fa84400d29f5e3d9d257f5f044b722c19c63d1ed))
- *(release)* Update CHANGELOG ([`2f4bc91`](https://github.com/LuMarans30/switcheroo-control-rs/commit/2f4bc919feca0cd75d4f7680323bbbc826ad26f9))
- *(release)* Skip doc and refactor sections ([`8d8a69c`](https://github.com/LuMarans30/switcheroo-control-rs/commit/8d8a69c150501420f64622387c35e7db4887105f))
- *(cargo)* Update repository link ([`3752502`](https://github.com/LuMarans30/switcheroo-control-rs/commit/37525027168b3d86e6b56890a04c626081f85433))
- *(lints)* Add pedantic clippy configuration in workspace ([`22d1d7e`](https://github.com/LuMarans30/switcheroo-control-rs/commit/22d1d7ea076e883e390fae323bf20922608d9667))
- *(clippy)* Add pedantic warnings into workflow ([`b8823e9`](https://github.com/LuMarans30/switcheroo-control-rs/commit/b8823e9e61d908ac72075785d6cd352bed2145c1))
- Add crates descriptions ([`84c0a41`](https://github.com/LuMarans30/switcheroo-control-rs/commit/84c0a418cc479ffd0e57449f3cfb1b02027d1e86))
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
