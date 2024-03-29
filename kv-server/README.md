## KV server 示例

一个类似 Redis 缓存服务器 demo。

### 需求分析
- 根据不同的命令完成数据存储、读取、监听等操作。
- 客户端可以通过网络访问 KV server。
- 数据可根据需要存储在内存或持久化到磁盘。

### 组件设计
![kv-server-design.png](docs%2Fkv-server-design.png)