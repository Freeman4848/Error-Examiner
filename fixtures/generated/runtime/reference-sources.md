# Reference-based runtime fixtures

These short synthetic fixtures exercise documented formats without copying full production logs.

- `elasticsearch-ecs.log`: Elastic ECS JSON fields and Elasticsearch dataset requirement:
  <https://www.elastic.co/guide/en/elasticsearch/reference/8.19/logging.html>
- `prometheus-logfmt.log`: official Alertmanager/Prometheus logfmt examples:
  <https://prometheus.io/docs/alerting/latest/configuration/>
- `rabbitmq-plaintext.log`: RabbitMQ default `$time [$level] $pid $msg` layout:
  <https://www.rabbitmq.com/docs/4.1/logging#log-message-format>
- `kafka-broker-log4j.log`: Kafka's official `[%d] %p %m (%c)%n` Log4j2 pattern:
  <https://github.com/apache/kafka/blob/trunk/config/log4j2.yaml>
- `opensearch-application.log`: OpenSearch application log layout and examples:
  <https://docs.opensearch.org/latest/install-and-configure/configuring-opensearch/logs/>
- `aws-vpc-flow-default.log`: AWS default VPC Flow Log fields and examples:
  <https://docs.aws.amazon.com/vpc/latest/userguide/flow-logs-records-examples.html>
