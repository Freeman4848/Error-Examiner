# apache.log

Format: `apache-error-log` · chars 3265→364 · events 40 · important 20 · duplicates 37 · batches 1

## Raw

```text
[Sun Dec 04 04:47:44 2005] [error] mod_jk child workerEnv in error state 6
[Sun Dec 04 04:51:18 2005] [error] mod_jk child workerEnv in error state 6
[Sun Dec 04 04:51:18 2005] [error] mod_jk child workerEnv in error state 6
[Sun Dec 04 04:51:18 2005] [error] mod_jk child workerEnv in error state 6
[Sun Dec 04 04:51:55 2005] [error] mod_jk child workerEnv in error state 6
[Sun Dec 04 04:52:15 2005] [error] mod_jk child workerEnv in error state 6
[Sun Dec 04 04:52:15 2005] [error] mod_jk child workerEnv in error state 7
[Sun Dec 04 04:52:15 2005] [error] mod_jk child workerEnv in error state 7
[Sun Dec 04 04:52:52 2005] [error] mod_jk child workerEnv in error state 7
[Sun Dec 04 04:52:52 2005] [error] mod_jk child workerEnv in error state 6
[Sun Dec 04 04:53:16 2005] [error] mod_jk child workerEnv in error state 7
[Sun Dec 04 04:53:16 2005] [error] mod_jk child workerEnv in error state 6
[Sun Dec 04 04:53:54 2005] [error] mod_jk child workerEnv in error state 7
[Sun Dec 04 04:54:18 2005] [error] mod_jk child workerEnv in error state 6
[Sun Dec 04 04:54:18 2005] [error] mod_jk child workerEnv in error state 6
[Sun Dec 04 04:54:18 2005] [error] mod_jk child workerEnv in error state 7
[Sun Dec 04 04:54:18 2005] [error] mod_jk child workerEnv in error state 7
[Sun Dec 04 04:54:20 2005] [error] mod_jk child workerEnv in error state 6
[Sun Dec 04 04:56:59 2005] [error] mod_jk child workerEnv in error state 6
[Sun Dec 04 04:57:00 2005] [error] mod_jk child workerEnv in error state 6
[Sun Dec 04 04:47:44 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties
[Sun Dec 04 04:51:08 2005] [notice] jk2_init() Found child 6725 in scoreboard slot 10
[Sun Dec 04 04:51:09 2005] [notice] jk2_init() Found child 6726 in scoreboard slot 8
[Sun Dec 04 04:51:09 2005] [notice] jk2_init() Found child 6728 in scoreboard slot 6
[Sun Dec 04 04:51:14 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties
[Sun Dec 04 04:51:14 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties
[Sun Dec 04 04:51:14 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties
[Sun Dec 04 04:51:37 2005] [notice] jk2_init() Found child 6736 in scoreboard slot 10
[Sun Dec 04 04:51:38 2005] [notice] jk2_init() Found child 6733 in scoreboard slot 7
[Sun Dec 04 04:51:38 2005] [notice] jk2_init() Found child 6734 in scoreboard slot 9
[Sun Dec 04 04:51:52 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties
[Sun Dec 04 04:51:52 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties
[Sun Dec 04 04:52:04 2005] [notice] jk2_init() Found child 6738 in scoreboard slot 6
[Sun Dec 04 04:52:04 2005] [notice] jk2_init() Found child 6741 in scoreboard slot 9
[Sun Dec 04 04:52:05 2005] [notice] jk2_init() Found child 6740 in scoreboard slot 7
[Sun Dec 04 04:52:05 2005] [notice] jk2_init() Found child 6737 in scoreboard slot 8
[Sun Dec 04 04:52:12 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties
[Sun Dec 04 04:52:12 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties
[Sun Dec 04 04:52:12 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties
[Sun Dec 04 04:52:36 2005] [notice] jk2_init() Found child 6748 in scoreboard slot 6

```

## Parsed

```text
[line 1 · ERROR · repeated 20 times]
[Sun Dec 04 04:47:44 2005] [error] mod_jk child workerEnv in error state 6

[line 21 · INFO · repeated 9 times]
[Sun Dec 04 04:47:44 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties

[line 22 · INFO · repeated 11 times]
[Sun Dec 04 04:51:08 2005] [notice] jk2_init() Found child 6725 in scoreboard slot 10
```

## Sent batches

### Batch 1

```text
[line 1 · ERROR · repeated 20 times]
[Sun Dec 04 04:47:44 2005] [error] mod_jk child workerEnv in error state 6

[line 21 · INFO · repeated 9 times]
[Sun Dec 04 04:47:44 2005] [notice] workerEnv.init() ok /etc/httpd/conf/workers2.properties

[line 22 · INFO · repeated 11 times]
[Sun Dec 04 04:51:08 2005] [notice] jk2_init() Found child 6725 in scoreboard slot 10
```
