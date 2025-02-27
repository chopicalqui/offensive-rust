# getpriv-dll

A simple DLL that retrieves the current user's privileges and writes them to a log file
(`getpriv-dll.log`) in the current working directory. The output is similar to the `whoami /priv`
command.

## Usage

### Building & Running

1. Compile the crate using the following command:

    ```shell
    cargo build --release --target x86_64-pc-windows-msvc
    ```

    ```shell
    cargo build --release --target i686-pc-windows-msvc
    ```

2. Run the compiled DLL using `rundll32.exe`:

    ```shell
    rundll32.exe .\target\i686-pc-windows-msvc\release\getpriv_dll.dll, DllMain
    ```

    ```shell
    rundll32.exe .\target\x86_64-pc-windows-msvc\release\getpriv_dll.dll, DllMain
    ```

3. Verify the output:

    ```shell
    cat .\getpriv-dll.log
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

Each time the DLL is executed, it appends the current user's privileges to `getpriv-dll.log`.

## Notes

- Ensure that the DLL is executed with the appropriate privileges to retrieve all available user
  privileges.
- The log file is written in the current working directory of the process that loaded the DLL.
- The DLL can be used for debugging or auditing user privilege configurations.
