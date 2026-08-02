# EE coverage: 100 high-volume application log sources

There is no official global size ranking for application logs. Ordering uses practical
volume tiers and engineering priority; positions inside a tier are approximate.

Coverage: **44 covered / 12 partial / 44 Raw**.

| # | Volume | Application / source | EE | Adapter or gap |
|---:|---|---|---|---|
| 1 | Extreme | Kubernetes pod/container logs | partial | kubernetes-prefixed-pod-log; arbitrary payload Raw |
| 2 | Extreme | Apache Kafka broker | covered | kafka-broker-log4j |
| 3 | Extreme | AWS VPC Flow Logs | covered | aws-vpc-flow-default |
| 4 | Extreme | Cloudflare HTTP/WAF | raw | adapter needed |
| 5 | Extreme | GCP Cloud Logging | covered | gcp-http/proto/text/json payload |
| 6 | Extreme | Azure Monitor and Activity Logs | raw | adapter needed |
| 7 | Extreme | AWS CloudTrail | raw | adapter needed |
| 8 | Extreme | Elasticsearch | covered | elastic-ecs-server-json |
| 9 | Extreme | OpenSearch | covered | opensearch-application-log |
| 10 | Extreme | Hadoop MapReduce | covered | hadoop |
| 11 | Extreme | HDFS | covered | hdfs |
| 12 | Extreme | Apache Spark | covered | spark |
| 13 | Extreme | Apache Flink | raw | adapter needed |
| 14 | Extreme | Apache Cassandra | raw | adapter needed |
| 15 | Extreme | ClickHouse | raw | adapter needed |
| 16 | Extreme | MongoDB | covered | mongodb-json-log |
| 17 | Extreme | PostgreSQL server | partial | client connection error only |
| 18 | Extreme | MySQL server | covered | mysql-error-log |
| 19 | Extreme | Oracle Database | raw | adapter needed |
| 20 | Extreme | Microsoft SQL Server | raw | adapter needed |
| 21 | High | Nginx | covered | nginx-access and nginx-error |
| 22 | High | Apache HTTP Server | covered | apache-error-log |
| 23 | High | Envoy proxy | raw | adapter needed |
| 24 | High | HAProxy | raw | adapter needed |
| 25 | High | Istio service mesh | raw | adapter needed |
| 26 | High | Docker and BuildKit | covered | docker-cli and docker-buildkit-error |
| 27 | High | containerd and CRI | raw | adapter needed |
| 28 | High | systemd-journald | covered | systemd journal short and JSON |
| 29 | High | Linux syslog | covered | linux-syslog |
| 30 | High | Windows Event Log and ETW | partial | Windows CBS text variant |
| 31 | High | Android Logcat | covered | android-logcat |
| 32 | High | macOS Unified Log | covered | mac-unified-syslog |
| 33 | High | OpenStack | covered | openstack |
| 34 | High | Apache ZooKeeper | covered | zookeeper |
| 35 | High | RabbitMQ | covered | rabbitmq-plaintext |
| 36 | High | Apache ActiveMQ | raw | adapter needed |
| 37 | High | Apache Pulsar | raw | adapter needed |
| 38 | High | NATS | raw | adapter needed |
| 39 | High | Redis | covered | redis-log |
| 40 | High | Prometheus server | covered | prometheus-logfmt |
| 41 | High | Grafana Loki | raw | adapter needed |
| 42 | High | OpenTelemetry Collector | raw | OTLP JSON adapter needed |
| 43 | High | Fluent Bit | raw | adapter needed |
| 44 | High | Logstash | raw | adapter needed |
| 45 | High | AWS ALB and ELB access logs | raw | adapter needed |
| 46 | High | AWS API Gateway | raw | adapter needed |
| 47 | High | AWS Lambda | raw | adapter needed |
| 48 | High | GCP Cloud Run | partial | GCP envelope covered; inner app varies |
| 49 | High | Azure Functions | raw | adapter needed |
| 50 | High | Kubernetes Events | covered | kubernetes-events |
| 51 | High | Suricata IDS | raw | EVE JSON adapter needed |
| 52 | High | Zeek network logs | raw | TSV and JSON adapters needed |
| 53 | High | Cisco ASA | raw | adapter needed |
| 54 | High | Palo Alto PAN-OS | raw | adapter needed |
| 55 | High | FortiGate | raw | adapter needed |
| 56 | High | pfSense | raw | adapter needed |
| 57 | High | HashiCorp Vault audit | raw | adapter needed |
| 58 | High | Keycloak | raw | adapter needed |
| 59 | High | Active Directory | raw | adapter needed |
| 60 | High | OpenSSH | covered | openssh-auth |
| 61 | Medium | Jenkins | raw | adapter needed |
| 62 | Medium | GitHub Actions | raw | workflow annotation adapter needed |
| 63 | Medium | GitLab CI | raw | adapter needed |
| 64 | Medium | Argo CD | raw | adapter needed |
| 65 | Medium | Terraform | raw | adapter needed |
| 66 | Medium | Ansible | raw | adapter needed |
| 67 | Medium | Docker Compose | partial | Docker errors covered; Compose envelope varies |
| 68 | Medium | npm | covered | npm-cli-error |
| 69 | Medium | Node.js and V8 | covered | node-exception |
| 70 | Medium | Python runtime | covered | python-traceback |
| 71 | Medium | Java and JVM | covered | javac and java-exception |
| 72 | Medium | .NET runtime | covered | dotnet-exception |
| 73 | Medium | Go runtime and tests | covered | go-panic and go-test-failure |
| 74 | Medium | Rust and Cargo | covered | cargo-rustc-error |
| 75 | Medium | GCC and G++ | covered | gcc-compiler and gpp-compiler |
| 76 | Medium | MSBuild | covered | msbuild-diagnostic |
| 77 | Medium | Apache Maven | raw | adapter needed |
| 78 | Medium | Gradle | raw | adapter needed |
| 79 | Medium | pytest | partial | Python traceback covered; pytest summary not parsed |
| 80 | Medium | JUnit | partial | Java exception covered; test report not parsed |
| 81 | Medium | React browser errors | covered | react-browser-error |
| 82 | Medium | Vue browser errors | covered | vue-browser-error |
| 83 | Medium | Angular browser errors | covered | angular-browser-error |
| 84 | Medium | Chrome and Opera DevTools | covered | browser Chromium V8 and security |
| 85 | Medium | Firefox DevTools | covered | browser-firefox-stack and security |
| 86 | Medium | Safari DevTools | covered | browser-safari-webkit-stack |
| 87 | Medium | Spring Boot | partial | Java stack covered; framework envelope varies |
| 88 | Medium | Apache Tomcat | partial | Java stack covered; Catalina format missing |
| 89 | Medium | Microsoft IIS | raw | adapter needed |
| 90 | Medium | WordPress and PHP | raw | PHP adapter needed |
| 91 | Medium | Laravel | raw | adapter needed |
| 92 | Medium | Django | partial | Python traceback covered; request envelope varies |
| 93 | Medium | Flask | partial | Python traceback covered; request envelope varies |
| 94 | Medium | FastAPI and Uvicorn | partial | Python traceback covered; server envelope varies |
| 95 | Medium | Ruby on Rails | raw | Ruby adapter needed |
| 96 | Special | Google Firestore audit logs | covered | gcp-proto-payload-audit |
| 97 | Special | GCP HTTP request logs | covered | gcp-http-request |
| 98 | Special | AddressSanitizer and LeakSanitizer | covered | native-asan |
| 99 | Special | GDB native backtrace | covered | native-gdb-backtrace and native-crash |
| 100 | Special | Bash and shell command errors | covered | shell-command-not-found |

Definitions: `covered` = executable adapter plus fixture; `partial` = only common variants;
`raw` = safe Raw fallback, therefore a candidate for a future adapter.

References: [LogHub benchmark](https://github.com/logpai/loghub), [OpenTelemetry Logs Data Model](https://opentelemetry.io/docs/specs/otel/logs/data-model/) and [mapping appendix](https://opentelemetry.io/docs/specs/otel/logs/data-model-appendix/).
