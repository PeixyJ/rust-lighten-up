# Rust 嵌入式开发 Micro:bit

## 前言

一直都很想玩嵌入式开发,但是又不想学 `C` 这种那么难的语言,所以基于这次学习 `Rust` 想玩一下嵌入式开发.所以记录的内容都是我在看[`micro::bit v2 Embedded Discovery Book`](https://docs.rust-embedded.org/discovery-mb2/#microbit-v2-embedded-discovery-book)这份教材所遇到的问题.

## 环境

1. Micro:bit v2 板子 x1
2. Mac OS
3. 数据线一根要确定能与电脑相连

## 硬件信息

以下信息在**其他**分类章节中可以寻找到内容

芯片信息: `nRF52833` 

芯片封装: `nRF52833_xxAA` 

> 这块为什么 nRF52833 芯片 后面 需要添加  xxAA 不太确定有知道的可以给我回复

交叉编译目标: `thumbv7em-none-eabihf` 适合 `Micro:bit v2` 从文档可以找到对应的交叉编译目标

硬件内存点位: Ram `0x00000000`  Flash `0x20000000`

硬件内存长度: Ram`128KB` Flash `512KB`

## 安装软件

### 安装Rust [官网](https://www.rust-lang.org/zh-CN/)

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**校验安装**

```
# 查看 Rust 版本
➜  ~ rustup show
Default host: aarch64-apple-darwin
rustup home:  /Users/peixinyi/.rustup

stable-aarch64-apple-darwin (default)
rustc 1.81.0 (eeb90cda1 2024-09-04)

# 查看 Cargo 版本
➜  ~ cargo -V
cargo 1.81.0 (2dbb1af80 2024-08-20)
```

### 创建项目

进入控制台输入

```shell
cargo new lighten

#    Creating binary (application) `lighten` package
#note: see more `Cargo.toml` keys and their definitions at https://doc.rustlang.org/cargo/reference/manifest.html
```

**项目结构**

```shell
.
├── Cargo.toml
└── src
    └── main.rs

2 directories, 2 files
```

### 添加交叉编译能力

新增 `编译目标` 

```shell
rustup target add thumbv7em-none-eabihf
```

查看`编译目标`

```shell
rust show

Default host: aarch64-apple-darwin
rustup home:  /Users/peixinyi/.rustup

installed targets for active toolchain
--------------------------------------

aarch64-apple-darwin
thumbv7em-none-eabi

active toolchain
----------------

stable-aarch64-apple-darwin (default)
rustc 1.81.0 (eeb90cda1 2024-09-04)
```

### 修改启动类

打开`src/main.rs`

```rust
#![no_std]
#![no_main]

fn main() -> ! {
    loop {}
}
```

`#![no_std]`

- `#![no_std]` 属性告诉 Rust 编译器编译程序时不使用标准库（`std`）。
- 嵌入式系统通常运行在没有操作系统的硬件上，因此不需要标准库提供的功能。

`#![no_main]`

- `#![no_main]` 属性告诉 Rust 编译器程序不包含标准的 `main` 函数。
- 在 Rust 中，`main` 函数是程序的入口点，即程序开始执行的地方。

### 添加依赖

查找 `板条箱` https://crates.io/,在这里我们尝试导入一个 `cortex-m-rt`

`crates` https://crates.io/search?q=cortex-m-rt

`docs.rs` https://docs.rs/cortex-m-rt/latest/cortex_m_rt/

从文档中我们可以看到需要在项目中添加 `memory.x`

#### **新增依赖 `cortex-m-rt`**

```shell
➜  lighten git:(master) ✗ cargo add cortex-m-rt
    Updating `ustc` index
      Adding cortex-m-rt v0.7.3 to dependencies
             Features:
             - device
             - set-sp
             - set-vtor
     Locking 6 packages to latest compatible versions
      Adding cortex-m-rt v0.7.3
      Adding cortex-m-rt-macros v0.7.0
      Adding proc-macro2 v1.0.87
      Adding quote v1.0.37
      Adding syn v1.0.109 (latest: v2.0.79)
      Adding unicode-ident v1.0.13
```

##### 新增文件 `memory.x`

```
MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 512K
  RAM : ORIGIN = 0x20000000, LENGTH = 128K
}
```

##### 修改main

```rust
#![no_std]
#![no_main]

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    loop {}
}
```

在此处添加了 `#[entry]` 标识此处为一个入口

### 编译程序

```
cargo rustc --target thumbv7em-none-eabi -- \
      -C link-arg=-nostartfiles -C link-arg=-Tlink.x
```

但是执行的时候会出现异常 告诉我们缺少了一个解决异常(恐慌)的程序

我们暂时不出来他,就让他异常就行

```
error: `#[panic_handler]` function required, but not found
```

##### 简化编译脚本

```shell
mkdir .cargo
touch .cargo/config.toml
```

写入配置

```
[build]
target = "thumbv7em-none-eabihf"

[target.thumbv7em-none-eabihf]
rustflags = [
    "-C", "link-arg=-Tlink.x",
]
```

这样我们就可以直接使用 `cargo build` 进行编译.

##### 解决恐慌

项目导入 `panic-halt` 其可以帮我们快速的解决恐慌问题.

```
cargo add panic-halt
```

在main.rs中引入 `panic-halt` 只需要添加 `use panic_halt as _;` 就可以了.

```
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    loop {}
}

```

**检查是否正常**

```
cargo check       # 检查代码
cargo build       # 编译代码
cargo size -- -Ax # 查看包大小

lighten-up  :
section                size         addr
.vector_table         0x400          0x0
.text                  0x8c        0x400
.rodata                   0        0x48c
.data                     0   0x20000000
.gnu.sgstubs              0        0x4a0
.bss                      0   0x20000000
.uninit                   0   0x20000000
.debug_abbrev        0x1198          0x0
.debug_info         0x22642          0x0
.debug_aranges       0x1330          0x0
.debug_ranges       0x19c80          0x0
.debug_str          0x3b5a0          0x0
.comment               0x40          0x0
.ARM.attributes        0x3a          0x0
.debug_frame         0x4120          0x0
.debug_line         0x1ee2c          0x0
.debug_loc             0x29          0x0
Total               0x9d1a5
```

### 将程序写入设备

#### 安装 embed

```
cargo install embed
```

配置写入目标芯片

```
touch Embed.toml

[default.general]
chip = "nRF52833_xxAA"
```

#### 写入程序

插入 `MicrobitV2 ` 至电脑 

```
cargo embed

   Compiling lighten v0.1.0 (/Users/peixinyi/Desktop/lighten)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.97s
      Config default
      Target /Users/peixinyi/Desktop/lighten/target/thumbv7em-none-eabihf/debug/lighten
      Erasing ✔ [00:00:00] [###########################################################################################] 4.00 KiB/4.00 KiB @ 28.64 KiB/s (eta 0s )
  Programming ✔ [00:00:00] [###########################################################################################] 4.00 KiB/4.00 KiB @ 15.85 KiB/s (eta 0s )    Finished in 0.421s

```

此时你应该看到了你的 Microbit 已经被刷写掉了默认程序.

### 添加 rtt 调试

#### 添加 rtt 

安装 `rat-target` https://crates.io/crates/rtt-target

```
 cargo add rtt-target
```

从文档中看到,他还需要我们实现一个  [`critical-section`](https://github.com/rust-embedded/critical-section) 这里我们可以直接使用 `cortex-m` 为我们提供的能力

```
cargo add cortex-m --features critical-section-single-core
```

#### 配置embed.toml

```
[default.general]
chip = "nRF52833_xxAA"

[default.rtt]
enabled = true
```

#### 修改`main.rs`

```
#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello, world!");
    loop {
        rprintln!("Looping...");
        for _ in 0..100000 {
            nop();
        }
    }
}
```

####  写入程序

```shell
cargo embed                                                                                                                                                     
#Hello, world!
#Looping...
#Looping...
#Looping...
#Looping...
#Looping...
#Looping...
#Looping...
#Looping...
#Looping...
#Looping...
```

### 点亮LED

#### 导入 `microbit-v2`

这是官方为`rust`写`crate`

```shell
cargo add microbit-v2
```

#### 修改 `main.rs`

```rust
#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use microbit::Board;
use microbit::display::blocking::Display;
use microbit::hal::Timer;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Starting up...");
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let led = [
        [0, 1, 0, 0, 0],
        [0, 1, 0, 0, 0],
        [0, 1, 0, 0, 0],
        [0, 1, 0, 0, 0],
        [0, 1, 0, 0, 0]
    ];
    loop {
        rprintln!("Looping...");
        display.show(&mut timer, led, 1000);
        display.clear();
        timer.delay_ms(1000);
    }
}
```

**编译代码**

```shell
rust embed

#help: there is a method `delay_ns` with a similar name
#   |
#30 |         timer.delay_ns(1000);
#   |               ~~~~~~~~
#
#error: aborting due to 1 previous error; 1 warning emitted
#
#For more information about this error, try `rustc --explain E0599`.
#       Error Failed to run cargo build: exit code = Some(101).
```

#### 修复问题

导入 `embedded_hal`

```
cargo add embedded_hal
```

修改 `main.rs`

添加以下代码

```rust
use embedded_hal::delay::DelayNs;
```

```rust
#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use microbit::Board;
use microbit::display::blocking::Display;
use microbit::hal::Timer;
use panic_halt as _;
use embedded_hal::delay::DelayNs;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Starting up...");
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let led = [
        [0, 1, 0, 0, 0],
        [0, 1, 0, 0, 0],
        [0, 1, 0, 0, 0],
        [0, 1, 0, 0, 0],
        [0, 1, 0, 0, 0]
    ];
    loop {
        rprintln!("Looping...");
        display.show(&mut timer, led, 1000);
        display.clear();
        timer.delay_ms(1000);
    }
}
```



## 使用GDB 调试

TODO



## 其他

### 寻找交叉编译目标 

以 `Micro:bit v2` 举例

### 找到设备的信息

https://tech.microbit.org/hardware/ 从该网页可以看到其 核心是 [Arm Cortex-M4 32 bit processor with FPU](https://developer.arm.com/ip-products/processors/cortex-m/cortex-m4) 进入点击查看页面 系统架构`Architecture`可以看到最终的是 `Armv7E-M`

### 找到交叉编译目标

https://doc.rust-lang.org/beta/rustc/platform-support.html 进入 `Rustc`文档的 `Platform Support` 节点找到 `Armv7E-M`通过表格可以看到最终的编译目标[`thumbv7em-none-eabi`](https://doc.rust-lang.org/beta/rustc/platform-support/thumbv7em-none-eabi.html)

### 找到Memory map

从 https://tech.microbit.org/hardware/ 进入 Model [Nordic nRF52833](https://www.nordicsemi.com/Products/Low-power-short-range-wireless/nRF52833) 查看文档 选择右侧的 [Documentation](https://docs.nordicsemi.com/category/nrf52833-category) 选择 `Resources` 下的 [Product Specification](https://docs.nordicsemi.com/bundle/ps_nrf52833/page/keyfeatures_html5.html) 下载 [nRF52833 Product Specification v1.7](https://docs-be.nordicsemi.com/bundle/ps_nrf52833/attach/nRF52833_PS_v1.7.pdf?_LANG=enus) 

打开文档后找到 `Memory map` 就可以看到 `Data Ram`和 `Flash` 的地址了 分别是 `0x00000000` 和 `0x20000000`

### 找到内存和闪存芯片长度

进入 https://tech.microbit.org/hardware/ 即可以看到 

|标题 |长度 |
| --------- | ----- |
| Flash ROM | 512KB |
| RAM       | 128KB |

## 附件

* [Rust Micro:bit 板条箱](https://github.com/nrf-rs/microbit/tree/main) 
* [Rust Mb2 教材](https://docs.rust-embedded.org/discovery-mb2/)
* [Rust 项目存储库](https://github.com/rust-embedded/discovery-mb2/)
* [cortex-m-quickstart 项目快速启动](https://github.com/rust-embedded/cortex-m-quickstart)
* [Embedded Rust setup explained](https://www.youtube.com/watch?v=TOAynddiu5M)
* https://tech.microbit.org/hardware/
* https://docs.rs/cortex-m-rt/latest/cortex_m_rt/
* https://docs.rs/panic-halt/latest/panic_halt/
* https://developer.arm.com/Processors/Cortex-M4
* https://doc.rust-lang.org/beta/rustc/platform-support.html
* https://docs.rs/microbit-v2/0.15.1/microbit/display/blocking/index.html
* https://docs.rs/cortex-m/0.7.7/cortex_m/
