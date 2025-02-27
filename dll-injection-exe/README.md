# dll-injection-exe

Rust application that injects a DLL file into another process.

# Usage

## Building & Running

1. Compile the Crate using the following command:

    ```shell
    cargo build --release
    ```

2. Run the compiled DLL via command:

    ```shell
    cargo run -- --name ..\messagebox-dll\target\release\messagebox_dll.dll --pid 14128
    ```

    Note: Probably you want to use an absolute path for the DLL to be injected.

## [Optional] Signing the Binary

1. Create self-signed certificate

    ```shell
    openssl req -x509 -newkey rsa:3072 -keyout key.pem -out cert.pem -sha256 -days 365
    ```

    ```shell
    openssl pkcs12 -inkey key.pem -in cert.pem -export -out sign.pfx
    ```

2. Signing the binary. This requires the [Windows SDK](https://developer.microsoft.com/en-us/windows/downloads/windows-sdk/)

    ```shell
    signtool sign /f sign.pfx /p <pfx-password> /t http://timestamp.digicert.com /fd sha256 target\release\dll-injection.exe
    ```

# Misc

Information about Microsoft's Rust for Windows library, refer to:

https://microsoft.github.io/windows-docs-rs/doc/windows/
