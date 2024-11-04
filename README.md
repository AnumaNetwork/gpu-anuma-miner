# Anuma-miner
[![Build status](https://github.com/AnumaNetwork/gpu-anuma-miner/workflows/ci/badge.svg)](https://github.com/AnumaNetwork/gpu-anuma-miner/actions)
[![Latest version](https://img.shields.io/crates/v/anuma-miner.svg)](https://crates.io/crates/anuma-miner)
![License](https://img.shields.io/crates/l/anuma-miner.svg)
[![dependency status](https://deps.rs/repo/github/AnumaNetwork/gpu-anuma-miner/status.svg)](https://deps.rs/repo/github/AnumaNetwork/gpu-anuma-miner)

[![Discord](https://discordapp.com/api/guilds/599153230659846165/embed.png)](https://discord.gg/8ZT93HctbQ)
[![Telegram](https://img.shields.io/badge/Telegram-2CA5E0?style=for-the-badge&logo=telegram&logoColor=white)](https://t.me/+I_nxFsUGsAk5ZDA0)


## Installation
### From Sources
Install via `cargo install` not supported for latest version.

The regular version is still available at
```sh
cargo install anuma-miner
```

### From Git Sources

If you are looking to build from the repository (for debug / extension), note that the plugins are additional
packages in the workspace. To compile a specific package, run the following command or any subset of it

```sh
git clone git@github.com:AnumaNetwork/gpu-anuma-miner.git
cd anuma-miner
cargo build --release -p anuma-miner -p anumacuda -p anumaopencl
```
And, the miner (and plugins) will be in `targets/release`. You can replace the last line with
```sh
cargo build --release --all
```

### From Binaries
The [release page](https://github.com/AnumaNetwork/gpu-anuma-miner/releases) includes precompiled binaries for Linux, and Windows (for the GPU version).

### Removing Plugins
To remove a plugin simply remove the corresponding `dll`/`so` for the directory of the miner. 

* `libanumacuda.so`, `libanumacuda.dll`: Cuda support for Anuma-Miner
* `libanumaopencl.so`, `libanumaopencl.dll`: OpenCL support for Anuma-Miner

# Usage
To start mining you need to run [anumad](https://github.com/AnumaNetwork/anumad) and have an address to send the rewards to.


Help:
```
anuma-miner 
A Anuma high performance CPU miner

USAGE:
    anuma-miner [OPTIONS] --mining-address <MINING_ADDRESS>

OPTIONS:
    -a, --mining-address <MINING_ADDRESS>                  The Anuma address for the miner reward
        --cuda-device <CUDA_DEVICE>                        Which CUDA GPUs to use [default: all]
        --cuda-disable                                     Disable cuda workers
        --cuda-lock-core-clocks <CUDA_LOCK_CORE_CLOCKS>    Lock core clocks eg: ,1200, [default: 0]
        --cuda-lock-mem-clocks <CUDA_LOCK_MEM_CLOCKS>      Lock mem clocks eg: ,810, [default: 0]
        --cuda-no-blocking-sync                            Actively wait for result. Higher CPU usage, but less red blocks. Can have lower workload.
        --cuda-power-limits <CUDA_POWER_LIMITS>            Lock power limits eg: ,150, [default: 0]
        --cuda-workload <CUDA_WORKLOAD>                    Ratio of nonces to GPU possible parrallel run [default: 64]
        --cuda-workload-absolute                           The values given by workload are not ratio, but absolute number of nonces [default: false]
    -d, --debug                                            Enable debug logging level
        --devfund-percent <DEVFUND_PERCENT>                The percentage of blocks to send to the devfund (minimum 2%) [default: 2]
        --experimental-amd                                 Uses SMID instructions in AMD. Miner will crash if instruction is not supported
    -h, --help                                             Print help information
        --mine-when-not-synced                             Mine even when anumad says it is not synced
        --nonce-gen <NONCE_GEN>                            The random method used to generate nonces. Options: (i) xoshiro (ii) lean [default: lean]
        --opencl-amd-disable                               Disables AMD mining (does not override opencl-enable)
        --opencl-device <OPENCL_DEVICE>                    Which OpenCL GPUs to use on a specific platform
        --opencl-enable                                    Enable opencl, and take all devices of the chosen platform
        --opencl-no-amd-binary                             Disable fetching of precompiled AMD kernel (if exists)
        --opencl-platform <OPENCL_PLATFORM>                Which OpenCL platform to use (limited to one per executable)
        --opencl-workload <OPENCL_WORKLOAD>                Ratio of nonces to GPU possible parrallel run in OpenCL [default: 512]
        --opencl-workload-absolute                         The values given by workload are not ratio, but absolute number of nonces in OpenCL [default: false]
    -p, --port <PORT>                                      Anumad port [default: Mainnet = 12412, Testnet = 12513]
    -s, --anumad-address <ANUMAD_ADDRESS>                  The IP of the anumad instance [default: 127.0.0.1]
    -t, --threads <NUM_THREADS>                            Amount of CPU miner threads to launch [default: 0]
        --testnet                                          Use testnet instead of mainnet [default: false]
```

To start mining you just need to run the following:

`./anuma-miner --mining-address anuma:XXXXX`

This will run the miner on all the available CPU cores.

# Devfund

The devfund is a fund managed by the Anuma community in order to fund Anuma development <br>
A miner that wants to mine higher percentage into the dev-fund can pass the following flags: <br>
`--devfund-precent=XX.YY` to mine only XX.YY% of the blocks into the devfund.

**This version automatically sets the devfund donation to the community designated address. 
Due to community decision, the minimum amount in the precompiled binaries is 2%**

# Donation Addresses

**Anuma Devs**: `anuma:qrg03ql6rnfvwd8a3z04j5a33l6f5d0ua8fkysftx8w7kacf0lsrqzxq2k2ud`


