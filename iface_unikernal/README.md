Building a Rust unikernel that can be booted from a USB involves several steps and tools. Here's an outline:

1. **Rust Environment Setup**: Install the latest stable Rust toolchain using `rustup`, Rust's official installer and version management tool.

   ```shell
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Selecting a Unikernel Library**: Choose a Rust-based unikernel library. One popular choice is `rusty-hermit`, which is designed for building Rust applications as unikernels.

    - Add `rusty-hermit` as a dependency in your `Cargo.toml`.

      ```toml
      [dependencies]
      hermit-sys = "0.3"
      ```

3. **Application Development**: Write your Rust application. For a basic example:

   ```rust
   fn main() {
       println!("Hello, unikernel!");
   }
   ```

4. **Building the Unikernel**: Compile your application into a unikernel binary. Use `cargo` to build for the `x86_64-unknown-hermit` target.

   ```shell
   cargo build --target x86_64-unknown-hermit
   ```

   Ensure you have the correct target installed:

   ```shell
   rustup target add x86_64-unknown-hermit
   ```

5. **Creating a Bootable Image**: Convert the unikernel binary into a bootable image. This often requires using tools like `rusty-loader` or `mkisofs` to create an ISO image.

6. **Writing to USB**: Use a tool like `dd` (on Unix systems) or Rufus (on Windows) to write the bootable image to a USB drive.

   ```shell
   sudo dd if=path/to/unikernel.iso of=/dev/sdX bs=4M status=progress
   ```

   Replace `/dev/sdX` with the correct device identifier for your USB drive.

7. **Booting**: Boot from the USB drive on a compatible machine or in a virtual environment that supports unikernels.

Remember, the exact steps may vary based on the unikernel framework you choose and your development environment. Always refer to the documentation of the specific unikernel project you are using.