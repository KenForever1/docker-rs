## 简介
使用rust实现简单的docker，熟悉linux cgroups、pivotRoot、overlayfs等。

## 测试环境
为了避免对开发环境造成影响，可以启动一个docker容器执行docker-rs。
```
docker run --privileged  --network=host  -v /home/ken/Codes/:/workspace  --name mydemo -it  ubuntu:22.04 /bin/bash
```

## 阶段

### 阶段1

docker启动子进程执行 /bin/ls命令：
```bash
$ ./docker-rs run -- /bin/ls -al
```

```bash
call Init subcommand, args : /bin/ls -al
command: /bin/ls, args: -al
Mount proc filesystem...
arg cstr: -al
Exec command...
total 15076
drwxr-xr-x  7 1000 1000     4096 Sep  8 15:49 .
drwxr-xr-x  3 1000 1000     4096 Sep  8 02:50 ..
-rw-r--r--  1 1000 1000        0 Sep  8 02:50 .cargo-lock
drwxr-xr-x 52 1000 1000     4096 Sep  8 15:48 .fingerprint
drwxr-xr-x  8 1000 1000     4096 Sep  8 03:08 build
drwxr-xr-x  2 1000 1000    20480 Sep  8 15:49 deps
-rwxr-xr-x  2 1000 1000 15385280 Sep  8 15:49 docker-rs
-rw-r--r--  1 1000 1000       88 Sep  8 03:08 docker-rs.d
drwxr-xr-x  2 1000 1000     4096 Sep  8 02:50 examples
drwxr-xr-x  9 1000 1000     4096 Sep  8 15:48 incremental
```