# hpc.log

Format: `hpc-failure` · chars 4073→1595 · events 40 · important 20 · duplicates 28 · batches 1

## Raw

```text
51338 node-3 node psu 1106496000 1 psu failure\ ambient=28
191898 node-238 node psu 1131240275 1 psu failure\ ambient=28
236618 node-104 node psu 1132434391 1 psu failure\ ambient=28
341834 node-118 node psu 1140312091 1 psu failure\ ambient=28
347972 node-118 node psu 1140430530 1 psu failure\ ambient=31
147394 Interconnect-0N00 switch_module temphigh 1129812510 1 Temperature (41C) exceeds warning threshold
147494 Interconnect-0N00 switch_module temphigh 1129813980 1 Temperature (41C) exceeds warning threshold
365140 node-69 unix.hw net.niff.down 1085075228 1 NIFF: node node-69 detected a failed network connection on network <IP> via interface alt0
401608 node-162 unix.hw net.niff.down 1142550442 1 NIFF: node node-162 detected a failed network connection on network <IP> via interface alt0
180537 node-D0 clusterfilesystem fdmn.panic 1131228351 1 ServerFileSystem: An ServerFileSystem domain panic has occurred on storage442
30700 Interconnect-1T00 switch_module bcast-error 1076189965 1 Link error
115576 Interconnect-1T00 switch_module bcast-error 1077793578 1 Link error
115153 Interconnect-0T00 switch_module bcast-error 1077757190 1 Link error
259323 Interconnect-1T00 switch_module bcast-error 1079076297 1 Link error
285855 Interconnect-1T00 switch_module bcast-error 1080298218 1 Link in reset
285823 Interconnect-0T00 switch_module bcast-error 1080292236 1 Link error
289491 Interconnect-0T00 switch_module bcast-error 1080825593 1 Link error
288385 Interconnect-0T00 switch_module bcast-error 1080668603 1 Link error
295965 Interconnect-1T00 switch_module bcast-error 1081534663 1 Link error
293098 Interconnect-1T00 switch_module bcast-error 1081296321 1 Link error
134681 node-246 unix.hw state_change.unavailable 1077804742 1 Component State Change: Component \042SCSI-WWID:01000010:6005-08b4-0001-00c6-0006-3000-003d-0000\042 is in the unavailable state (HWID=1973)
350766 node-109 unix.hw state_change.unavailable 1084680778 1 Component State Change: Component \042alt0\042 is in the unavailable state (HWID=3180)
344518 node-246 unix.hw state_change.unavailable 1084270955 1 Component State Change: Component \042alt0\042 is in the unavailable state (HWID=5089)
344448 node-153 unix.hw state_change.unavailable 1084270952 1 Component State Change: Component \042alt0\042 is in the unavailable state (HWID=4088)
366633 node-200 unix.hw state_change.unavailable 1085100843 1 Component State Change: Component \042alt0\042 is in the unavailable state (HWID=2538)
366463 node-122 unix.hw state_change.unavailable 1085084674 1 Component State Change: Component \042alt0\042 is in the unavailable state (HWID=2480)
438190 node-228 unix.hw state_change.unavailable 1097194780 1 Component State Change: Component \042alt0\042 is in the unavailable state (HWID=3713)
225111 node-10 unix.hw state_change.unavailable 1117296789 1 Component State Change: Component \042alt0\042 is in the unavailable state (HWID=3891)
360778 node-130 unix.hw state_change.unavailable 1141108031 1 Component State Change: Component \042alt0\042 is in the unavailable state (HWID=2478)
401569 node-169 unix.hw state_change.unavailable 1142550406 1 Component State Change: Component \042alt0\042 is in the unavailable state (HWID=2969)
401855 node-187 unix.hw state_change.unavailable 1142553646 1 Component State Change: Component \042alt0\042 is in the unavailable state (HWID=4159)
460773 node-199 unix.hw state_change.unavailable 1145552100 1 Component State Change: Component \042alt0\042 is in the unavailable state (HWID=2608)
2568643 node-70 action start 1074119817 1 clusterAddMember  (command 1902)
2570772 node-124 action start 1074123150 1 clusterAddMember  (command 1900)
2571927 node-28 action start 1074125371 1 risBoot  (command 1903)
2572286 node-17 action start 1074126278 1 bootGenvmunix  (command 1903)
2575909 node-162 action start 1074178193 1 boot  (command 1911)
2576195 node-181 action start 1074178628 1 boot  (command 1910)
2599298 node-198 action start 1074297419 1 boot  (command 1978)
2600743 node-57 action start 1074298084 1 boot  (command 1967)

```

## Parsed

```text
[line 1 · ERROR · repeated 5 times]
51338 node-3 node psu 1106496000 1 psu failure\ ambient=28

[line 6 · WARNING · repeated 2 times]
147394 Interconnect-0N00 switch_module temphigh 1129812510 1 Temperature (41C) exceeds warning threshold

[line 8 · ERROR · repeated 2 times]
365140 node-69 unix.hw net.niff.down 1085075228 1 NIFF: node node-69 detected a failed network connection on network <IP> via interface alt0

[line 10 · CRITICAL]
180537 node-D0 clusterfilesystem fdmn.panic 1131228351 1 ServerFileSystem: An ServerFileSystem domain panic has occurred on storage442

[line 11 · ERROR · repeated 9 times]
30700 Interconnect-1T00 switch_module bcast-error 1076189965 1 Link error

[line 15 · ERROR]
285855 Interconnect-1T00 switch_module bcast-error 1080298218 1 Link in reset

[line 21 · UNKNOWN]
134681 node-246 unix.hw state_change.unavailable 1077804742 1 Component State Change: Component \042SCSI-WWID:01000010:6005-08b4-0001-00c6-0006-3000-003d-0000\042 is in the unavailable state (HWID=1973)

[line 22 · UNKNOWN · repeated 11 times]
350766 node-109 unix.hw state_change.unavailable 1084680778 1 Component State Change: Component \042alt0\042 is in the unavailable state (HWID=3180)

[line 33 · UNKNOWN · repeated 2 times]
2568643 node-70 action start 1074119817 1 clusterAddMember  (command 1902)

[line 35 · UNKNOWN]
2571927 node-28 action start 1074125371 1 risBoot  (command 1903)

[line 36 · UNKNOWN]
2572286 node-17 action start 1074126278 1 bootGenvmunix  (command 1903)

[line 37 · UNKNOWN · repeated 4 times]
2575909 node-162 action start 1074178193 1 boot  (command 1911)
```

## Sent batches

### Batch 1

```text
[line 1 · ERROR · repeated 5 times]
51338 node-3 node psu 1106496000 1 psu failure\ ambient=28

[line 6 · WARNING · repeated 2 times]
147394 Interconnect-0N00 switch_module temphigh 1129812510 1 Temperature (41C) exceeds warning threshold

[line 8 · ERROR · repeated 2 times]
365140 node-69 unix.hw net.niff.down 1085075228 1 NIFF: node node-69 detected a failed network connection on network <IP> via interface alt0

[line 10 · CRITICAL]
180537 node-D0 clusterfilesystem fdmn.panic 1131228351 1 ServerFileSystem: An ServerFileSystem domain panic has occurred on storage442

[line 11 · ERROR · repeated 9 times]
30700 Interconnect-1T00 switch_module bcast-error 1076189965 1 Link error

[line 15 · ERROR]
285855 Interconnect-1T00 switch_module bcast-error 1080298218 1 Link in reset

[line 21 · UNKNOWN]
134681 node-246 unix.hw state_change.unavailable 1077804742 1 Component State Change: Component \042SCSI-WWID:01000010:6005-08b4-0001-00c6-0006-3000-003d-0000\042 is in the unavailable state (HWID=1973)

[line 22 · UNKNOWN · repeated 11 times]
350766 node-109 unix.hw state_change.unavailable 1084680778 1 Component State Change: Component \042alt0\042 is in the unavailable state (HWID=3180)

[line 33 · UNKNOWN · repeated 2 times]
2568643 node-70 action start 1074119817 1 clusterAddMember  (command 1902)

[line 35 · UNKNOWN]
2571927 node-28 action start 1074125371 1 risBoot  (command 1903)

[line 36 · UNKNOWN]
2572286 node-17 action start 1074126278 1 bootGenvmunix  (command 1903)

[line 37 · UNKNOWN · repeated 4 times]
2575909 node-162 action start 1074178193 1 boot  (command 1911)
```
