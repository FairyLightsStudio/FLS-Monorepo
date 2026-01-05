## 📂 client-panel

放的是 terapanel 和 用户客户端 通信时用的契约。

对于流式消息传输场景（例如 Terminal）会走契约之外的 websocket。

## 📂 domain

放的是 面板用到的 领域模型

## 📂 panel-daemon

放的是 terad 和 terapanel 通信时使用的契约。

service 定义在代码内的 NATS service 部分，这里仅仅存放 Message 结构体的序列化 / 反序列化契约。

## buf.yaml

buf 在本项目仅用作 LSP Provider
  