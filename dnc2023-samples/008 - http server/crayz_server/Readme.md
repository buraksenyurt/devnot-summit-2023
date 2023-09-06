# Crayz Server _(Sıfırdan Bir HTTP Server Yazmak)_

```shell
# TcpListener ile dinelemeye başladıktan sonra test etmek istersek
# Önce uygulamayı aşağıdaki gibi başlatıp,
RUST_LOG=info cargo run
# ardından ikinci bir terminal açıp aşağıdaki komutlar ile mesaj göndermeyi deneyebiliriz.
echo "ping" | netcat localhost 5555
echo "ping pong" | netcat localhost 5555
curl localhost:5555
curl -X POST http://localhost:5555/movies/ -H 'Content-Type: application/json' -d '{"message":"only one ping Vaseley."}'
curl http://localhost:5555/query?word=red
```