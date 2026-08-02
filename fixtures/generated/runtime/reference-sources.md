# Reference-based runtime fixtures

These short synthetic fixtures exercise documented formats without copying full production logs.

- `elasticsearch-ecs.log`: Elastic ECS JSON fields and Elasticsearch dataset requirement:
  <https://www.elastic.co/guide/en/elasticsearch/reference/8.19/logging.html>
- `prometheus-logfmt.log`: official Alertmanager/Prometheus logfmt examples:
  <https://prometheus.io/docs/alerting/latest/configuration/>
- `rabbitmq-plaintext.log`: RabbitMQ default `$time [$level] $pid $msg` layout:
  <https://www.rabbitmq.com/docs/4.1/logging#log-message-format>
