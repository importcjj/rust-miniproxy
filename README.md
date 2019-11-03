# miniproxy

使用Rust实现的简易代理，同时支持HTTP，HTTPS和SOCKS5协议，支持仅用于学习交流。

## 如何编译

首先安装Rust，如何安装请移步[官网](https://www.rust-lang.org/learn/get-started)

```sh
cargo build --release
```

二进制文件会在项目目录的target/release文件夹下，找到两个名为`minilocal`和`miniserver`的二进制文件即可。关于如何交叉编译，请自行搜索。不过我自己在macbook上交叉编译就没成功过。

## 如何使用

本代理分为两部分：`minilocal`和`miniserver`。`miniserver`运行于网络服务器上，`minilocal`运行于本地。

a. 先在服务器上部署`miniserver`

```
miniserver -h 0.0.0.0 -p 59999 -d
```

b. 然后在本地启动`minilocal`

```
minilocal -s "xxx.xx.xx.xx:59999" -p 9998
```

c. 进行系统代理设定，代理地址为`127.0.0.0:9998`，本代理同时支持HTTP，HTTPS和SOCKS5协议

## 原理及教程

尚在编写中，文档可见[docs](./docs)

