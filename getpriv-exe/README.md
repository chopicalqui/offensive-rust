# getpriv-exe

A simple application that retrieves the current user's privileges and writes them to the standard
output. The output is similar to the `whoami /priv` command.

## Usage

### Building & Running

1. Compile the crate using the following command:

    ```shell
    cargo build --release
    ```

    or

    ```shell
    cargo build --release --target x86_64-pc-windows-msvc
    ```

    or

    ```shell
    cargo build --release --target i686-pc-windows-msvc
    ```


2. Run the application:

    ```shell
    cargo run
    ```

    Example output:

    ```
    User privileges (5):
    SeShutdownPrivilege                     : Disabled
    SeChangeNotifyPrivilege                 : Enabled
    SeUndockPrivilege                       : Disabled
    SeIncreaseWorkingSetPrivilege           : Disabled
    SeTimeZonePrivilege                     : Disabled
    ```

## Notes

- Ensure that the DLL is executed with the appropriate privileges to retrieve all available user privileges.
