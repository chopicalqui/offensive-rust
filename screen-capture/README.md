# screen-capture

Rust application that implements screen capturing.

# Usage

## Building & Running (DLL)

1. Compile the Crate using the following command:

    ```shell
    cargo build --release
    ```

2. Run the compiled DLL via command:

    ```shell
    rundll32.exe .\target\release\screen_capture_lib.dll, ScreenCapture c:\temp
    ```

## Building & Running (EXE)

Create a single screenshot.

```shell
cargo run --release -- --file c:\temp\screenshot.bmp
```

Create one screenshot per second.

```shell
cargo run --release -- --dir c:\temp
```
