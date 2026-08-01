# spark.log

Format: `spark` · chars 3898→3107 · events 40 · important 0 · duplicates 15 · batches 1

## Raw

```text
17/06/09 20:10:40 INFO executor.CoarseGrainedExecutorBackend: Registered signal handlers for [TERM, HUP, INT]
17/06/09 20:10:40 INFO spark.SecurityManager: Changing view acls to: yarn,curi
17/06/09 20:10:40 INFO spark.SecurityManager: Changing modify acls to: yarn,curi
17/06/09 20:10:40 INFO spark.SecurityManager: SecurityManager: authentication disabled; ui acls disabled; users with view permissions: Set(yarn, curi); users with modify permissions: Set(yarn, curi)
17/06/09 20:10:41 INFO spark.SecurityManager: Changing view acls to: yarn,curi
17/06/09 20:10:41 INFO spark.SecurityManager: Changing modify acls to: yarn,curi
17/06/09 20:10:41 INFO spark.SecurityManager: SecurityManager: authentication disabled; ui acls disabled; users with view permissions: Set(yarn, curi); users with modify permissions: Set(yarn, curi)
17/06/09 20:10:41 INFO slf4j.Slf4jLogger: Slf4jLogger started
17/06/09 20:10:41 INFO Remoting: Starting remoting
17/06/09 20:10:41 INFO Remoting: Remoting started; listening on addresses :[akka.tcp://sparkExecutorActorSystem@mesos-slave-07:55904]
17/06/09 20:10:41 INFO util.Utils: Successfully started service 'sparkExecutorActorSystem' on port 55904.
17/06/09 20:10:41 INFO storage.DiskBlockManager: Created local directory at /opt/hdfs/nodemanager/usercache/curi/appcache/application_1485248649253_0147/blockmgr-<UUID>
17/06/09 20:10:41 INFO storage.MemoryStore: MemoryStore started with capacity 17.7 GB
17/06/09 20:10:42 INFO executor.CoarseGrainedExecutorBackend: Connecting to driver: spark://CoarseGrainedScheduler@<IP>:48069
17/06/09 20:10:42 INFO executor.CoarseGrainedExecutorBackend: Successfully registered with driver
17/06/09 20:10:42 INFO executor.Executor: Starting executor ID 5 on host mesos-slave-07
17/06/09 20:10:42 INFO util.Utils: Successfully started service 'org.apache.spark.network.netty.NettyBlockTransferService' on port 40984.
17/06/09 20:10:42 INFO netty.NettyBlockTransferService: Server created on 40984
17/06/09 20:10:42 INFO storage.BlockManagerMaster: Trying to register BlockManager
17/06/09 20:10:42 INFO storage.BlockManagerMaster: Registered BlockManager
17/06/09 20:10:45 INFO executor.CoarseGrainedExecutorBackend: Got assigned task 0
17/06/09 20:10:45 INFO executor.CoarseGrainedExecutorBackend: Got assigned task 1
17/06/09 20:10:45 INFO executor.CoarseGrainedExecutorBackend: Got assigned task 2
17/06/09 20:10:45 INFO executor.CoarseGrainedExecutorBackend: Got assigned task 3
17/06/09 20:10:45 INFO executor.Executor: Running task 0.0 in stage 0.0 (TID 0)
17/06/09 20:10:45 INFO executor.Executor: Running task 2.0 in stage 0.0 (TID 2)
17/06/09 20:10:45 INFO executor.Executor: Running task 1.0 in stage 0.0 (TID 1)
17/06/09 20:10:45 INFO executor.Executor: Running task 3.0 in stage 0.0 (TID 3)
17/06/09 20:10:45 INFO executor.CoarseGrainedExecutorBackend: Got assigned task 4
17/06/09 20:10:45 INFO executor.Executor: Running task 4.0 in stage 0.0 (TID 4)
17/06/09 20:10:45 INFO broadcast.TorrentBroadcast: Started reading broadcast variable 9
17/06/09 20:10:45 INFO storage.MemoryStore: Block broadcast_9_piece0 stored as bytes in memory (estimated size 5.2 KB, free 5.2 KB)
17/06/09 20:10:45 INFO broadcast.TorrentBroadcast: Reading broadcast variable 9 took 160 ms
17/06/09 20:10:46 INFO storage.MemoryStore: Block broadcast_9 stored as values in memory (estimated size 8.8 KB, free 14.0 KB)
17/06/09 20:10:46 INFO spark.CacheManager: Partition rdd_2_1 not found, computing it
17/06/09 20:10:46 INFO spark.CacheManager: Partition rdd_2_3 not found, computing it
17/06/09 20:10:46 INFO spark.CacheManager: Partition rdd_2_0 not found, computing it
17/06/09 20:10:46 INFO spark.CacheManager: Partition rdd_2_2 not found, computing it
17/06/09 20:10:46 INFO spark.CacheManager: Partition rdd_2_4 not found, computing it
17/06/09 20:10:46 INFO rdd.HadoopRDD: Input split: hdfs://<IP>:9000/pjhe/logs/2kSOSP.log:21876+7292

```

## Parsed

```text
[line 1 · INFO]
17/06/09 20:10:40 INFO executor.CoarseGrainedExecutorBackend: Registered signal handlers for [TERM, HUP, INT]

[line 2 · INFO · repeated 2 times]
17/06/09 20:10:40 INFO spark.SecurityManager: Changing view acls to: yarn,curi

[line 3 · INFO · repeated 2 times]
17/06/09 20:10:40 INFO spark.SecurityManager: Changing modify acls to: yarn,curi

[line 4 · INFO · repeated 2 times]
17/06/09 20:10:40 INFO spark.SecurityManager: SecurityManager: authentication disabled; ui acls disabled; users with view permissions: Set(yarn, curi); users with modify permissions: Set(yarn, curi)

[line 8 · INFO]
17/06/09 20:10:41 INFO slf4j.Slf4jLogger: Slf4jLogger started

[line 9 · INFO]
17/06/09 20:10:41 INFO Remoting: Starting remoting

[line 10 · INFO]
17/06/09 20:10:41 INFO Remoting: Remoting started; listening on addresses :[akka.tcp://sparkExecutorActorSystem@mesos-slave-07:55904]

[line 11 · INFO]
17/06/09 20:10:41 INFO util.Utils: Successfully started service 'sparkExecutorActorSystem' on port 55904.

[line 12 · INFO]
17/06/09 20:10:41 INFO storage.DiskBlockManager: Created local directory at /opt/hdfs/nodemanager/usercache/curi/appcache/application_1485248649253_0147/blockmgr-<UUID>

[line 13 · INFO]
17/06/09 20:10:41 INFO storage.MemoryStore: MemoryStore started with capacity 17.7 GB

[line 14 · INFO]
17/06/09 20:10:42 INFO executor.CoarseGrainedExecutorBackend: Connecting to driver: spark://CoarseGrainedScheduler@<IP>:48069

[line 15 · INFO]
17/06/09 20:10:42 INFO executor.CoarseGrainedExecutorBackend: Successfully registered with driver

[line 16 · INFO]
17/06/09 20:10:42 INFO executor.Executor: Starting executor ID 5 on host mesos-slave-07

[line 17 · INFO]
17/06/09 20:10:42 INFO util.Utils: Successfully started service 'org.apache.spark.network.netty.NettyBlockTransferService' on port 40984.

[line 18 · INFO]
17/06/09 20:10:42 INFO netty.NettyBlockTransferService: Server created on 40984

[line 19 · INFO]
17/06/09 20:10:42 INFO storage.BlockManagerMaster: Trying to register BlockManager

[line 20 · INFO]
17/06/09 20:10:42 INFO storage.BlockManagerMaster: Registered BlockManager

[line 21 · INFO · repeated 5 times]
17/06/09 20:10:45 INFO executor.CoarseGrainedExecutorBackend: Got assigned task 0

[line 25 · INFO · repeated 5 times]
17/06/09 20:10:45 INFO executor.Executor: Running task 0.0 in stage 0.0 (TID 0)

[line 31 · INFO]
17/06/09 20:10:45 INFO broadcast.TorrentBroadcast: Started reading broadcast variable 9

[line 32 · INFO]
17/06/09 20:10:45 INFO storage.MemoryStore: Block broadcast_9_piece0 stored as bytes in memory (estimated size 5.2 KB, free 5.2 KB)

[line 33 · INFO]
17/06/09 20:10:45 INFO broadcast.TorrentBroadcast: Reading broadcast variable 9 took 160 ms

[line 34 · INFO]
17/06/09 20:10:46 INFO storage.MemoryStore: Block broadcast_9 stored as values in memory (estimated size 8.8 KB, free 14.0 KB)

[line 35 · INFO · repeated 5 times]
17/06/09 20:10:46 INFO spark.CacheManager: Partition rdd_2_1 not found, computing it

[line 40 · INFO]
17/06/09 20:10:46 INFO rdd.HadoopRDD: Input split: hdfs://<IP>:9000/pjhe/logs/2kSOSP.log:21876+7292
```

## Sent batches

### Batch 1

```text
[line 1 · INFO]
17/06/09 20:10:40 INFO executor.CoarseGrainedExecutorBackend: Registered signal handlers for [TERM, HUP, INT]

[line 2 · INFO · repeated 2 times]
17/06/09 20:10:40 INFO spark.SecurityManager: Changing view acls to: yarn,curi

[line 3 · INFO · repeated 2 times]
17/06/09 20:10:40 INFO spark.SecurityManager: Changing modify acls to: yarn,curi

[line 4 · INFO · repeated 2 times]
17/06/09 20:10:40 INFO spark.SecurityManager: SecurityManager: authentication disabled; ui acls disabled; users with view permissions: Set(yarn, curi); users with modify permissions: Set(yarn, curi)

[line 8 · INFO]
17/06/09 20:10:41 INFO slf4j.Slf4jLogger: Slf4jLogger started

[line 9 · INFO]
17/06/09 20:10:41 INFO Remoting: Starting remoting

[line 10 · INFO]
17/06/09 20:10:41 INFO Remoting: Remoting started; listening on addresses :[akka.tcp://sparkExecutorActorSystem@mesos-slave-07:55904]

[line 11 · INFO]
17/06/09 20:10:41 INFO util.Utils: Successfully started service 'sparkExecutorActorSystem' on port 55904.

[line 12 · INFO]
17/06/09 20:10:41 INFO storage.DiskBlockManager: Created local directory at /opt/hdfs/nodemanager/usercache/curi/appcache/application_1485248649253_0147/blockmgr-<UUID>

[line 13 · INFO]
17/06/09 20:10:41 INFO storage.MemoryStore: MemoryStore started with capacity 17.7 GB

[line 14 · INFO]
17/06/09 20:10:42 INFO executor.CoarseGrainedExecutorBackend: Connecting to driver: spark://CoarseGrainedScheduler@<IP>:48069

[line 15 · INFO]
17/06/09 20:10:42 INFO executor.CoarseGrainedExecutorBackend: Successfully registered with driver

[line 16 · INFO]
17/06/09 20:10:42 INFO executor.Executor: Starting executor ID 5 on host mesos-slave-07

[line 17 · INFO]
17/06/09 20:10:42 INFO util.Utils: Successfully started service 'org.apache.spark.network.netty.NettyBlockTransferService' on port 40984.

[line 18 · INFO]
17/06/09 20:10:42 INFO netty.NettyBlockTransferService: Server created on 40984

[line 19 · INFO]
17/06/09 20:10:42 INFO storage.BlockManagerMaster: Trying to register BlockManager

[line 20 · INFO]
17/06/09 20:10:42 INFO storage.BlockManagerMaster: Registered BlockManager

[line 21 · INFO · repeated 5 times]
17/06/09 20:10:45 INFO executor.CoarseGrainedExecutorBackend: Got assigned task 0

[line 25 · INFO · repeated 5 times]
17/06/09 20:10:45 INFO executor.Executor: Running task 0.0 in stage 0.0 (TID 0)

[line 31 · INFO]
17/06/09 20:10:45 INFO broadcast.TorrentBroadcast: Started reading broadcast variable 9

[line 32 · INFO]
17/06/09 20:10:45 INFO storage.MemoryStore: Block broadcast_9_piece0 stored as bytes in memory (estimated size 5.2 KB, free 5.2 KB)

[line 33 · INFO]
17/06/09 20:10:45 INFO broadcast.TorrentBroadcast: Reading broadcast variable 9 took 160 ms

[line 34 · INFO]
17/06/09 20:10:46 INFO storage.MemoryStore: Block broadcast_9 stored as values in memory (estimated size 8.8 KB, free 14.0 KB)

[line 35 · INFO · repeated 5 times]
17/06/09 20:10:46 INFO spark.CacheManager: Partition rdd_2_1 not found, computing it

[line 40 · INFO]
17/06/09 20:10:46 INFO rdd.HadoopRDD: Input split: hdfs://<IP>:9000/pjhe/logs/2kSOSP.log:21876+7292
```
