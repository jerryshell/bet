# BET

[English](README.md) | 简体中文

BET 是一个开源免费的邮件群发工具。使用 Rust 编写。

## 使用方法

1. 将 `email_config.example.json` 重命名为 `email_config.json`
2. 修改 `email_config.json`
3. 将需要发送的附件放到 `data` 目录下，文件名为对应的邮箱地址，注意不能省略文件本身的后缀名
4. 运行

## email_config.json 配置项说明

```text
smtp_domain -> smtp 域名
from_email_address -> 发件人邮箱地址
from_email_password -> 发件人邮箱密码
subject -> 邮件标题
html_body -> 邮件正文
attachment_file_name -> 附件文件名
```

## 开源许可证

[GNU Affero General Public License v3.0](https://choosealicense.com/licenses/agpl-3.0/)
