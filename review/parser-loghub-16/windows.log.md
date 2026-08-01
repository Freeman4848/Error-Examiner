# windows.log

Format: `windows-cbs` · chars 5147→4038 · events 40 · important 0 · duplicates 14 · batches 1

## Raw

```text
2016-09-28 04:30:31, Info                  CBS    SQM: Failed to start upload with file pattern: C:\Windows\servicing\sqm\*_std.sqm, flags: 0x2 [HRESULT = 0x80004005 - E_FAIL]
2016-09-28 04:30:31, Info                  CBS    SQM: Failed to start standard sample upload. [HRESULT = 0x80004005 - E_FAIL]
2016-09-28 04:30:31, Info                  CBS    SQM: Warning: Failed to upload all unsent reports. [HRESULT = 0x80004005 - E_FAIL]
2016-09-28 04:30:31, Info                  CBS    Warning: Unrecognized packageExtended attribute.
2016-09-28 04:30:31, Info                  CBS    Expecting attribute name [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
2016-09-28 04:30:31, Info                  CBS    Warning: Unrecognized packageExtended attribute.
2016-09-28 04:30:31, Info                  CBS    Expecting attribute name [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
2016-09-28 04:30:31, Info                  CBS    Warning: Unrecognized packageExtended attribute.
2016-09-28 04:30:31, Info                  CBS    Expecting attribute name [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
2016-09-28 04:30:31, Info                  CBS    Warning: Unrecognized packageExtended attribute.
2016-09-28 04:30:31, Info                  CBS    Expecting attribute name [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
2016-09-28 04:30:31, Info                  CBS    Warning: Unrecognized packageExtended attribute.
2016-09-28 04:30:31, Info                  CBS    Warning: Unrecognized packageExtended attribute.
2016-09-28 04:30:31, Info                  CBS    Expecting attribute name [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]
2016-09-28 04:30:31, Info                  CBS    Warning: Unrecognized packageExtended attribute.
2016-09-28 04:30:30, Info                  CBS    Loaded Servicing Stack v6.1.7601.23505 with Core: C:\Windows\winsxs\amd64_microsoft-windows-servicingstack_31bf3856ad364e35_6.1.7601.23505_none_681aa442f6fed7f0\cbscore.dll
2016-09-28 04:30:31, Info                  CSI    00000001@2016/9/27:20:30:31.455 WcpInitialize (wcp.dll version <IP>) called (stack @0x7fed806eb5d @0x7fef9fb9b6d @0x7fef9f8358f @0xff83e97c @0xff83d799 @0xff83db2f)
2016-09-28 04:30:31, Info                  CSI    00000002@2016/9/27:20:30:31.458 WcpInitialize (wcp.dll version <IP>) called (stack @0x7fed806eb5d @0x7fefa006ade @0x7fef9fd2984 @0x7fef9f83665 @0xff83e97c @0xff83d799)
2016-09-28 04:30:31, Info                  CSI    00000003@2016/9/27:20:30:31.458 WcpInitialize (wcp.dll version <IP>) called (stack @0x7fed806eb5d @0x7fefa1c8728 @0x7fefa1c8856 @0xff83e474 @0xff83d7de @0xff83db2f)
2016-09-28 04:30:31, Info                  CBS    Ending TrustedInstaller initialization.
2016-09-28 04:30:31, Info                  CBS    Starting the TrustedInstaller main loop.
2016-09-28 04:30:31, Info                  CBS    TrustedInstaller service starts successfully.
2016-09-28 04:30:31, Info                  CBS    SQM: Initializing online with Windows opt-in: False
2016-09-28 04:30:31, Info                  CBS    SQM: Cleaning up report files older than 10 days.
2016-09-28 04:30:31, Info                  CBS    SQM: Requesting upload of all unsent reports.
2016-09-28 04:30:31, Info                  CBS    SQM: Queued 0 file(s) for upload with pattern: C:\Windows\servicing\sqm\*_all.sqm, flags: 0x6
2016-09-28 04:30:31, Info                  CBS    No startup processing required, TrustedInstaller service was not set as autostart, or else a reboot is still pending.
2016-09-28 04:30:31, Info                  CBS    NonStart: Checking to ensure startup processing was not required.
2016-09-28 04:30:31, Info                  CSI    00000004 IAdvancedInstallerAwareStore_ResolvePendingTransactions (call 1) (flags = 00000004, progress = NULL, phase = 0, pdwDisposition = @0xb6fd90
2016-09-28 04:30:31, Info                  CSI    00000005 Creating NT transaction (seq 1), objectname [6]"(null)"
2016-09-28 04:30:31, Info                  CSI    00000006 Created NT transaction (seq 1) result 0x00000000, handle @0x214
2016-09-28 04:30:31, Info                  CSI    00000007@2016/9/27:20:30:31.462 CSI perf trace:
2016-09-28 04:30:31, Info                  CBS    NonStart: Success, startup processing not required as expected.
2016-09-28 04:30:31, Info                  CBS    Startup processing thread terminated normally
2016-09-28 04:30:31, Info                  CSI    00000008 CSI Store 4991456 (0x00000000004c29e0) initialized

```

## Parsed

```text
[line 1 · INFO]
2016-09-28 04:30:31, Info                  CBS    SQM: Failed to start upload with file pattern: C:\Windows\servicing\sqm\*_std.sqm, flags: 0x2 [HRESULT = 0x80004005 - E_FAIL]

[line 2 · INFO]
2016-09-28 04:30:31, Info                  CBS    SQM: Failed to start standard sample upload. [HRESULT = 0x80004005 - E_FAIL]

[line 3 · INFO]
2016-09-28 04:30:31, Info                  CBS    SQM: Warning: Failed to upload all unsent reports. [HRESULT = 0x80004005 - E_FAIL]

[line 4 · INFO · repeated 7 times]
2016-09-28 04:30:31, Info                  CBS    Warning: Unrecognized packageExtended attribute.

[line 5 · INFO · repeated 5 times]
2016-09-28 04:30:31, Info                  CBS    Expecting attribute name [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]

[line 6 · INFO · repeated 5 times]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]

[line 21 · INFO]
2016-09-28 04:30:30, Info                  CBS    Loaded Servicing Stack v6.1.7601.23505 with Core: C:\Windows\winsxs\amd64_microsoft-windows-servicingstack_31bf3856ad364e35_6.1.7601.23505_none_681aa442f6fed7f0\cbscore.dll

[line 22 · INFO]
2016-09-28 04:30:31, Info                  CSI    00000001@2016/9/27:20:30:31.455 WcpInitialize (wcp.dll version <IP>) called (stack @0x7fed806eb5d @0x7fef9fb9b6d @0x7fef9f8358f @0xff83e97c @0xff83d799 @0xff83db2f)

[line 23 · INFO]
2016-09-28 04:30:31, Info                  CSI    00000002@2016/9/27:20:30:31.458 WcpInitialize (wcp.dll version <IP>) called (stack @0x7fed806eb5d @0x7fefa006ade @0x7fef9fd2984 @0x7fef9f83665 @0xff83e97c @0xff83d799)

[line 24 · INFO]
2016-09-28 04:30:31, Info                  CSI    00000003@2016/9/27:20:30:31.458 WcpInitialize (wcp.dll version <IP>) called (stack @0x7fed806eb5d @0x7fefa1c8728 @0x7fefa1c8856 @0xff83e474 @0xff83d7de @0xff83db2f)

[line 25 · INFO]
2016-09-28 04:30:31, Info                  CBS    Ending TrustedInstaller initialization.

[line 26 · INFO]
2016-09-28 04:30:31, Info                  CBS    Starting the TrustedInstaller main loop.

[line 27 · INFO]
2016-09-28 04:30:31, Info                  CBS    TrustedInstaller service starts successfully.

[line 28 · INFO]
2016-09-28 04:30:31, Info                  CBS    SQM: Initializing online with Windows opt-in: False

[line 29 · INFO]
2016-09-28 04:30:31, Info                  CBS    SQM: Cleaning up report files older than 10 days.

[line 30 · INFO]
2016-09-28 04:30:31, Info                  CBS    SQM: Requesting upload of all unsent reports.

[line 31 · INFO]
2016-09-28 04:30:31, Info                  CBS    SQM: Queued 0 file(s) for upload with pattern: C:\Windows\servicing\sqm\*_all.sqm, flags: 0x6

[line 32 · INFO]
2016-09-28 04:30:31, Info                  CBS    No startup processing required, TrustedInstaller service was not set as autostart, or else a reboot is still pending.

[line 33 · INFO]
2016-09-28 04:30:31, Info                  CBS    NonStart: Checking to ensure startup processing was not required.

[line 34 · INFO]
2016-09-28 04:30:31, Info                  CSI    00000004 IAdvancedInstallerAwareStore_ResolvePendingTransactions (call 1) (flags = 00000004, progress = NULL, phase = 0, pdwDisposition = @0xb6fd90

[line 35 · INFO]
2016-09-28 04:30:31, Info                  CSI    00000005 Creating NT transaction (seq 1), objectname [6]"(null)"

[line 36 · INFO]
2016-09-28 04:30:31, Info                  CSI    00000006 Created NT transaction (seq 1) result 0x00000000, handle @0x214

[line 37 · INFO]
2016-09-28 04:30:31, Info                  CSI    00000007@2016/9/27:20:30:31.462 CSI perf trace:

[line 38 · INFO]
2016-09-28 04:30:31, Info                  CBS    NonStart: Success, startup processing not required as expected.

[line 39 · INFO]
2016-09-28 04:30:31, Info                  CBS    Startup processing thread terminated normally

[line 40 · INFO]
2016-09-28 04:30:31, Info                  CSI    00000008 CSI Store 4991456 (0x00000000004c29e0) initialized
```

## Sent batches

### Batch 1

```text
[line 1 · INFO]
2016-09-28 04:30:31, Info                  CBS    SQM: Failed to start upload with file pattern: C:\Windows\servicing\sqm\*_std.sqm, flags: 0x2 [HRESULT = 0x80004005 - E_FAIL]

[line 2 · INFO]
2016-09-28 04:30:31, Info                  CBS    SQM: Failed to start standard sample upload. [HRESULT = 0x80004005 - E_FAIL]

[line 3 · INFO]
2016-09-28 04:30:31, Info                  CBS    SQM: Warning: Failed to upload all unsent reports. [HRESULT = 0x80004005 - E_FAIL]

[line 4 · INFO · repeated 7 times]
2016-09-28 04:30:31, Info                  CBS    Warning: Unrecognized packageExtended attribute.

[line 5 · INFO · repeated 5 times]
2016-09-28 04:30:31, Info                  CBS    Expecting attribute name [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]

[line 6 · INFO · repeated 5 times]
2016-09-28 04:30:31, Info                  CBS    Failed to get next element [HRESULT = 0x800f080d - CBS_E_MANIFEST_INVALID_ITEM]

[line 21 · INFO]
2016-09-28 04:30:30, Info                  CBS    Loaded Servicing Stack v6.1.7601.23505 with Core: C:\Windows\winsxs\amd64_microsoft-windows-servicingstack_31bf3856ad364e35_6.1.7601.23505_none_681aa442f6fed7f0\cbscore.dll

[line 22 · INFO]
2016-09-28 04:30:31, Info                  CSI    00000001@2016/9/27:20:30:31.455 WcpInitialize (wcp.dll version <IP>) called (stack @0x7fed806eb5d @0x7fef9fb9b6d @0x7fef9f8358f @0xff83e97c @0xff83d799 @0xff83db2f)

[line 23 · INFO]
2016-09-28 04:30:31, Info                  CSI    00000002@2016/9/27:20:30:31.458 WcpInitialize (wcp.dll version <IP>) called (stack @0x7fed806eb5d @0x7fefa006ade @0x7fef9fd2984 @0x7fef9f83665 @0xff83e97c @0xff83d799)

[line 24 · INFO]
2016-09-28 04:30:31, Info                  CSI    00000003@2016/9/27:20:30:31.458 WcpInitialize (wcp.dll version <IP>) called (stack @0x7fed806eb5d @0x7fefa1c8728 @0x7fefa1c8856 @0xff83e474 @0xff83d7de @0xff83db2f)

[line 25 · INFO]
2016-09-28 04:30:31, Info                  CBS    Ending TrustedInstaller initialization.

[line 26 · INFO]
2016-09-28 04:30:31, Info                  CBS    Starting the TrustedInstaller main loop.

[line 27 · INFO]
2016-09-28 04:30:31, Info                  CBS    TrustedInstaller service starts successfully.

[line 28 · INFO]
2016-09-28 04:30:31, Info                  CBS    SQM: Initializing online with Windows opt-in: False

[line 29 · INFO]
2016-09-28 04:30:31, Info                  CBS    SQM: Cleaning up report files older than 10 days.

[line 30 · INFO]
2016-09-28 04:30:31, Info                  CBS    SQM: Requesting upload of all unsent reports.

[line 31 · INFO]
2016-09-28 04:30:31, Info                  CBS    SQM: Queued 0 file(s) for upload with pattern: C:\Windows\servicing\sqm\*_all.sqm, flags: 0x6

[line 32 · INFO]
2016-09-28 04:30:31, Info                  CBS    No startup processing required, TrustedInstaller service was not set as autostart, or else a reboot is still pending.

[line 33 · INFO]
2016-09-28 04:30:31, Info                  CBS    NonStart: Checking to ensure startup processing was not required.

[line 34 · INFO]
2016-09-28 04:30:31, Info                  CSI    00000004 IAdvancedInstallerAwareStore_ResolvePendingTransactions (call 1) (flags = 00000004, progress = NULL, phase = 0, pdwDisposition = @0xb6fd90

[line 35 · INFO]
2016-09-28 04:30:31, Info                  CSI    00000005 Creating NT transaction (seq 1), objectname [6]"(null)"

[line 36 · INFO]
2016-09-28 04:30:31, Info                  CSI    00000006 Created NT transaction (seq 1) result 0x00000000, handle @0x214

[line 37 · INFO]
2016-09-28 04:30:31, Info                  CSI    00000007@2016/9/27:20:30:31.462 CSI perf trace:

[line 38 · INFO]
2016-09-28 04:30:31, Info                  CBS    NonStart: Success, startup processing not required as expected.

[line 39 · INFO]
2016-09-28 04:30:31, Info                  CBS    Startup processing thread terminated normally

[line 40 · INFO]
2016-09-28 04:30:31, Info                  CSI    00000008 CSI Store 4991456 (0x00000000004c29e0) initialized
```
