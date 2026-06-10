// SPDX-License-Identifier: GPL-3.0-or-later

use switcheroo_common::GpuDevice;

use zbus::{Result, proxy};

#[proxy(
    interface = "net.hadess.SwitcherooControl",
    default_service = "net.hadess.SwitcherooControl",
    default_path = "/net/hadess/SwitcherooControl"
)]
pub trait Switcheroo {
    #[zbus(property)]
    fn has_dual_gpu(&self) -> Result<bool>;

    #[zbus(property, name = "NumGPUs")]
    fn num_gpus(&self) -> Result<u32>;

    #[zbus(property, name = "GPUs")]
    fn gpus(&self) -> Result<Vec<GpuDevice>>;
}
