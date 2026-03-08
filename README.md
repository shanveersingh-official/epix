# Epix
**A sovereign 32-bit monolithic kernel for legacy x86 hardware.**

Epix is a memory-safe kernel built from the ground up in Rust. It rejects the complexity of modern 64-bit "Long Mode" to maintain a universal compatibility profile for legacy BIOS/MBR environments. It is designed for absolute hardware sovereignty on x86 silicon manufactured over the last 30 years.



## 🛠 Technical Specifications
* **Architecture:** i686 (32-bit Protected Mode).
* **Boot Protocol:** MBR (Master Boot Record) / Legacy BIOS.
* **Compatibility:** Universal x86 (Intel 80386 through modern 32-bit compatible chips).
* **Memory Model:** Flat segmentation via the Global Descriptor Table (GDT).
* **Interrupts:** Manual PIC 8259 remapping with a 32-bit Interrupt Descriptor Table (IDT).



## 🏗 Kernel Architecture
Epix is a modular monolith. It operates without the overhead of 4-level paging or long-mode transitions, ensuring the fastest possible path from the BIOS handoff to kernel execution.

* **VGA Driver:** Direct Memory-Mapped I/O (MMIO) to the legacy buffer at 0xb8000.
* **Hardware ISRs:** Optimized Interrupt Service Routines for the 32-bit GPR (General Purpose Register) set.
* **Rash Shell:** A kernel-space interface for low-level system debugging and hardware state control.

## 🚀 Build System
Epix uses a custom target specification to ensure the compiler generates 32-bit machine code without assuming a standard C library or operating system environment.

### 1. Requirements

~~~bash
rustup component add rust-src llvm-tools-preview
cargo install bootimage
~~~

### 2. Binary Generation

~~~bash
# Targeting i686 (32-bit) logic
cargo bootimage --target i686-epix.json
~~~



## 💾 Hardware Deployment
Epix is distributed as a sector-aligned disk image. It can be written directly to any MBR-compatible media:

~~~bash
# Direct block-level write to target media
sudo dd if=target/i686-epix/debug/bootimage-epix.bin of=/dev/sdX status=progress
~~~

## ⚖️ License
MIT License.
