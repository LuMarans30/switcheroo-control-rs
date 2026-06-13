## [0.1.0] - 2026-06-13

### 🚀 Features

- *(drm)* Add ioctl probes for discrete GPU detection
- *(daemon)* Handle SIGINT/SIGTERM gracefully
- *(cli)* Add implicit subcommands
- *(daemon)* Add AllowReplacement flag to match upstream
- *(daemon)* Implement daemon replacement and optimize udev handling
- *(common)* Add YesNo newtype for boolean formatting
- *(i915)* Implement ioctl probe and fix misclassification

### 🐛 Bug Fixes

- *(detection)* Filter by render nodes instead of card nodes
- *(common)* Correct zvariant properties renaming
- *(detection)* Skip GPUs with empty env to match upstream
- *(detection)* Improve default card fallback logic
- *(daemon)* Prevent D-Bus signal flooding
- *(detection)* Avoid panic on empty cards list
- *(xe)* Correct Xe probe reading wrong config field
- *(daemon)* Improve graceful exit for already-running daemon
- *(cli)* Handle missing switcheroo daemon gracefully
- *(daemon)* Exit 1 when daemon is already running
- *(cli)* Let exec() handle executable errors
- *(amdgpu)* Correct ioctl direction flags for device info query
- *(cleanup)* Add GTX to regex
- *(daemon)* Minor bug fixes to amdgpu and startup logic

### 🚜 Refactor

- *(common)* Remove dead code
- *(common)* Cleaner Display for GpuDevice
- *(common)* Use manual zvariant impls and clean EnvVar API
- *(daemon)* Improve already-running daemon error message
- *(daemon)* Extract udev and hardware event loops
- *(cli)* Improve function names
- *(daemon)* Use PropertiesChanged for D-Bus signal emission
- *(cleanup)* Use LazyLock for static initialization
- *(common)* Split lib.rs into env_var and gpu_device modules
- *(daemon)* Use matches! for daemon running check

### 📚 Documentation

- *(readme)* Improve README
- *(readme)* Add WIP to title
- *(detection)* Explain why unsafe FFI call is sound
- *(readme)* Update following major changes
- *(readme)* Add architectural Mermaid diagram

### ⚡ Performance

- *(detection)* Optimize discrete GPU detection with direct udev FFI

### ⚙️ Miscellaneous Tasks

- Add rust-clippy analysis workflow
