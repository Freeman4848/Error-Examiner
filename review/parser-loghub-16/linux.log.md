# linux.log

Format: `linux-syslog` · chars 4258→951 · events 40 · important 20 · duplicates 32 · batches 1

## Raw

```text
Jun 14 15:16:01 combo sshd(pam_unix)[19939]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=<IP>
Jun 14 15:16:02 combo sshd(pam_unix)[19937]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=<IP>
Jun 15 02:04:59 combo sshd(pam_unix)[20882]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 02:04:59 combo sshd(pam_unix)[20884]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 02:04:59 combo sshd(pam_unix)[20883]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 02:04:59 combo sshd(pam_unix)[20885]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 02:04:59 combo sshd(pam_unix)[20886]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 02:04:59 combo sshd(pam_unix)[20892]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 02:04:59 combo sshd(pam_unix)[20893]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 02:04:59 combo sshd(pam_unix)[20896]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 02:04:59 combo sshd(pam_unix)[20897]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 02:04:59 combo sshd(pam_unix)[20898]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root
Jun 15 12:12:34 combo sshd(pam_unix)[23397]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=<IP>
Jun 15 12:12:34 combo sshd(pam_unix)[23395]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=<IP>
Jun 15 12:12:34 combo sshd(pam_unix)[23404]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=<IP>
Jun 15 12:12:34 combo sshd(pam_unix)[23399]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=<IP>
Jun 15 12:12:34 combo sshd(pam_unix)[23406]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=<IP>
Jun 15 12:12:34 combo sshd(pam_unix)[23394]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=<IP>
Jun 15 12:12:34 combo sshd(pam_unix)[23396]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=<IP>
Jun 15 12:12:34 combo sshd(pam_unix)[23407]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=<IP>
Jun 14 15:16:02 combo sshd(pam_unix)[19937]: check pass; user unknown
Jun 15 04:06:18 combo su(pam_unix)[21416]: session opened for user cyrus by (uid=0)
Jun 15 04:06:19 combo su(pam_unix)[21416]: session closed for user cyrus
Jun 15 04:06:20 combo logrotate: ALERT exited abnormally with [1]
Jun 15 04:12:42 combo su(pam_unix)[22644]: session opened for user news by (uid=0)
Jun 15 04:12:43 combo su(pam_unix)[22644]: session closed for user news
Jun 15 12:12:34 combo sshd(pam_unix)[23397]: check pass; user unknown
Jun 15 12:12:34 combo sshd(pam_unix)[23395]: check pass; user unknown
Jun 15 12:12:34 combo sshd(pam_unix)[23404]: check pass; user unknown
Jun 15 12:12:34 combo sshd(pam_unix)[23399]: check pass; user unknown
Jun 15 12:12:34 combo sshd(pam_unix)[23406]: check pass; user unknown
Jun 15 12:12:34 combo sshd(pam_unix)[23396]: check pass; user unknown
Jun 15 12:12:34 combo sshd(pam_unix)[23394]: check pass; user unknown
Jun 15 12:12:34 combo sshd(pam_unix)[23407]: check pass; user unknown
Jun 15 12:12:34 combo sshd(pam_unix)[23403]: check pass; user unknown
Jun 15 12:12:34 combo sshd(pam_unix)[23412]: check pass; user unknown
Jun 15 12:13:19 combo sshd(pam_unix)[23414]: check pass; user unknown
Jun 15 12:13:20 combo sshd(pam_unix)[23416]: check pass; user unknown
Jun 15 14:53:32 combo sshd(pam_unix)[23661]: check pass; user unknown
Jun 15 14:53:32 combo sshd(pam_unix)[23663]: check pass; user unknown

```

## Parsed

```text
[line 1 · ERROR · repeated 10 times]
Jun 14 15:16:01 combo sshd(pam_unix)[19939]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=<IP>

[line 3 · ERROR · repeated 10 times]
Jun 15 02:04:59 combo sshd(pam_unix)[20882]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root

[line 21 · UNKNOWN · repeated 15 times]
Jun 14 15:16:02 combo sshd(pam_unix)[19937]: check pass; user unknown

[line 22 · UNKNOWN]
Jun 15 04:06:18 combo su(pam_unix)[21416]: session opened for user cyrus by (uid=0)

[line 23 · UNKNOWN]
Jun 15 04:06:19 combo su(pam_unix)[21416]: session closed for user cyrus

[line 24 · UNKNOWN]
Jun 15 04:06:20 combo logrotate: ALERT exited abnormally with [1]

[line 25 · UNKNOWN]
Jun 15 04:12:42 combo su(pam_unix)[22644]: session opened for user news by (uid=0)

[line 26 · UNKNOWN]
Jun 15 04:12:43 combo su(pam_unix)[22644]: session closed for user news
```

## Sent batches

### Batch 1

```text
[line 1 · ERROR · repeated 10 times]
Jun 14 15:16:01 combo sshd(pam_unix)[19939]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=<IP>

[line 3 · ERROR · repeated 10 times]
Jun 15 02:04:59 combo sshd(pam_unix)[20882]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost=220-135-151-1.hinet-ip.hinet.net  user=root

[line 21 · UNKNOWN · repeated 15 times]
Jun 14 15:16:02 combo sshd(pam_unix)[19937]: check pass; user unknown

[line 22 · UNKNOWN]
Jun 15 04:06:18 combo su(pam_unix)[21416]: session opened for user cyrus by (uid=0)

[line 23 · UNKNOWN]
Jun 15 04:06:19 combo su(pam_unix)[21416]: session closed for user cyrus

[line 24 · UNKNOWN]
Jun 15 04:06:20 combo logrotate: ALERT exited abnormally with [1]

[line 25 · UNKNOWN]
Jun 15 04:12:42 combo su(pam_unix)[22644]: session opened for user news by (uid=0)

[line 26 · UNKNOWN]
Jun 15 04:12:43 combo su(pam_unix)[22644]: session closed for user news
```
