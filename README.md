## 掘金 接口相关

- 签到
- 签到天数
- 当前积分
- ...todo!

### 环境变量

- JUEJIN_COOKIE
#### 企业微信应用通知相关

[基本概念](https://developer.work.weixin.qq.com/document/path/90665#secret)

- QY_WEIXIN_CORPID            企业id
- QY_WEIXIN_CORPSECRET        应用 secret
- QY_WEIXIN_AGEBTID           应用id
- QY_WEIXIN_TOUSER            指定接收消息的成员，成员ID列表（多个接收者用‘|’分隔，最多支持1000个） 默认为 @all


- 运行

```zsh
JUEJIN_COOKIE=xxxx cargo run
```

- 运行测试

```sh
$ RUST_TEST_NOCAPTURE=1 cargo test
```

![process.png](/images/xxxxx.png)

### Docker 支持

- 运行 `docker build -t juejing:v1 ./` 构建镜像
- 修改 `docker-compose.yml` 环境变量，运行 `docker-compose up -d`