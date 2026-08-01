# zookeeper.log

Format: `zookeeper` · chars 4942→979 · events 40 · important 20 · duplicates 34 · batches 1

## Raw

```text
2015-07-29 19:04:29,071 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@688] - Send worker leaving thread
2015-07-29 19:04:29,079 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@679] - Interrupted while waiting for message on queue
2015-07-29 19:13:17,524 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@688] - Send worker leaving thread
2015-07-29 19:13:24,282 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@762] - Connection broken for id 188978561024, my id = 1, error =
2015-07-29 19:13:27,721 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@762] - Connection broken for id 188978561024, my id = 1, error =
2015-07-29 19:13:34,382 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@679] - Interrupted while waiting for message on queue
2015-07-29 19:13:37,626 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@688] - Send worker leaving thread
2015-07-29 19:13:44,301 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@688] - Send worker leaving thread
2015-07-29 19:13:47,731 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@762] - Connection broken for id 188978561024, my id = 1, error =
2015-07-29 19:13:54,399 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@762] - Connection broken for id 188978561024, my id = 1, error =
2015-07-29 19:14:04,406 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@679] - Interrupted while waiting for message on queue
2015-07-29 19:14:07,559 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@765] - Interrupting SendWorker
2015-07-29 19:14:07,653 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@688] - Send worker leaving thread
2015-07-29 19:14:24,329 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@765] - Interrupting SendWorker
2015-07-29 19:14:37,585 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@679] - Interrupted while waiting for message on queue
2015-07-29 19:14:47,593 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@765] - Interrupting SendWorker
2015-07-29 19:14:54,354 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@688] - Send worker leaving thread
2015-07-29 19:15:24,476 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@679] - Interrupted while waiting for message on queue
2015-07-29 19:15:37,647 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@765] - Interrupting SendWorker
2015-07-29 19:15:37,648 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@688] - Send worker leaving thread
2015-07-29 17:41:44,747 - INFO  [QuorumPeer[myid=1]/0:0:0:0:0:0:0:0:2181:FastLeaderElection@774] - Notification time out: 3200
2015-07-29 19:04:12,394 - INFO  [/<IP>:3888:QuorumCnxManager$Listener@493] - Received connection request /<IP>:45307
2015-07-29 19:13:24,370 - INFO  [/<IP>:3888:QuorumCnxManager$Listener@493] - Received connection request /<IP>:57707
2015-07-29 19:13:54,220 - INFO  [/<IP>:3888:QuorumCnxManager$Listener@493] - Received connection request /<IP>:45382
2015-07-29 19:14:44,256 - INFO  [/<IP>:3888:QuorumCnxManager$Listener@493] - Received connection request /<IP>:45440
2015-07-29 19:15:57,854 - INFO  [/<IP>:3888:QuorumCnxManager$Listener@493] - Received connection request /<IP>:57895
2015-07-29 19:16:44,440 - INFO  [/<IP>:3888:QuorumCnxManager$Listener@493] - Received connection request /<IP>:47727
2015-07-29 19:17:57,939 - INFO  [/<IP>:3888:QuorumCnxManager$Listener@493] - Received connection request /<IP>:58035
2015-07-29 19:18:14,511 - INFO  [/<IP>:3888:QuorumCnxManager$Listener@493] - Received connection request /<IP>:47838
2015-07-29 19:19:04,661 - INFO  [/<IP>:3888:QuorumCnxManager$Listener@493] - Received connection request /<IP>:58116
2015-07-29 19:21:36,502 - INFO  [/<IP>:3888:QuorumCnxManager$Listener@493] - Received connection request /<IP>:45957
2015-07-29 19:21:46,728 - INFO  [/<IP>:3888:QuorumCnxManager$Listener@493] - Received connection request /<IP>:58303
2015-07-29 19:21:49,960 - INFO  [/<IP>:3888:QuorumCnxManager$Listener@493] - Received connection request /<IP>:48096
2015-07-29 19:22:03,324 - INFO  [/<IP>:3888:QuorumCnxManager$Listener@493] - Received connection request /<IP>:48141
2015-07-29 19:22:20,143 - INFO  [/<IP>:3888:QuorumCnxManager$Listener@493] - Received connection request /<IP>:58421
2015-07-29 19:22:26,617 - INFO  [/<IP>:3888:QuorumCnxManager$Listener@493] - Received connection request /<IP>:46128
2015-07-29 19:22:26,830 - INFO  [/<IP>:3888:QuorumCnxManager$Listener@493] - Received connection request /<IP>:58452
2015-07-29 19:22:36,659 - INFO  [/<IP>:3888:QuorumCnxManager$Listener@493] - Received connection request /<IP>:46173
2015-07-29 19:22:40,083 - INFO  [/<IP>:3888:QuorumCnxManager$Listener@493] - Received connection request /<IP>:48280
2015-07-29 19:23:00,237 - INFO  [/<IP>:3888:QuorumCnxManager$Listener@493] - Received connection request /<IP>:58565

```

## Parsed

```text
[line 1 · WARNING · repeated 7 times]
2015-07-29 19:04:29,071 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@688] - Send worker leaving thread

[line 2 · WARNING · repeated 5 times]
2015-07-29 19:04:29,079 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@679] - Interrupted while waiting for message on queue

[line 4 · WARNING · repeated 4 times]
2015-07-29 19:13:24,282 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@762] - Connection broken for id 188978561024, my id = 1, error =

[line 12 · WARNING · repeated 4 times]
2015-07-29 19:14:07,559 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@765] - Interrupting SendWorker

[line 21 · INFO]
2015-07-29 17:41:44,747 - INFO  [QuorumPeer[myid=1]/0:0:0:0:0:0:0:0:2181:FastLeaderElection@774] - Notification time out: 3200

[line 22 · INFO · repeated 19 times]
2015-07-29 19:04:12,394 - INFO  [/<IP>:3888:QuorumCnxManager$Listener@493] - Received connection request /<IP>:45307
```

## Sent batches

### Batch 1

```text
[line 1 · WARNING · repeated 7 times]
2015-07-29 19:04:29,071 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@688] - Send worker leaving thread

[line 2 · WARNING · repeated 5 times]
2015-07-29 19:04:29,079 - WARN  [SendWorker:188978561024:QuorumCnxManager$SendWorker@679] - Interrupted while waiting for message on queue

[line 4 · WARNING · repeated 4 times]
2015-07-29 19:13:24,282 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@762] - Connection broken for id 188978561024, my id = 1, error =

[line 12 · WARNING · repeated 4 times]
2015-07-29 19:14:07,559 - WARN  [RecvWorker:188978561024:QuorumCnxManager$RecvWorker@765] - Interrupting SendWorker

[line 21 · INFO]
2015-07-29 17:41:44,747 - INFO  [QuorumPeer[myid=1]/0:0:0:0:0:0:0:0:2181:FastLeaderElection@774] - Notification time out: 3200

[line 22 · INFO · repeated 19 times]
2015-07-29 19:04:12,394 - INFO  [/<IP>:3888:QuorumCnxManager$Listener@493] - Received connection request /<IP>:45307
```
