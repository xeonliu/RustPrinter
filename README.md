# **R**upmPrinter
**R**upmPrinter is designed to be a _multi-platform_ __printer server__ & __frontend application__ for SJTU Library Printing System.

It is a reversed version of the official `UPMClient` Windows driver from `Unifound`.

**R** as in `Rust`/`Reverse`/`Rewrite`.

It works as a Sockets Server, a PCL Interpreter and a Frontend Client on `Windows`/`Linux`/`MacOS`.

On `Windows` it can also work as a **patch** of the original client.

# Usage
## Windows

### As a Patch
1. Rename `RustPrinter.exe` to `UPMPrinter.exe`
2. Replace `C:\upmclient\UPMPrinter.exe` with the new one.
### As a Server

## Unix-like Systems
1. Install CUPS (Usually preinstalled)
2. Register the printer (Please make sure port 6981 is available)
```shell
lpadmin -p RupmPrinter -E -v socket://127.0.0.1:6981 -m drv:///sample.drv/laserjet.ppd
```
> Note: You can change the PPD file to other PCL5 compatible files.
3. Run the client
4. Choose `RupmPrinter` as target when printing

## MacOS
### Same as Unix-like Systems
### Manually Set up in Settings



# Tasks
+ [x] 判断客户端上传是否一定需要IP地址作为目标：不用
+ [x] 首先读取Cookie, 使用`Auth/Check`判断或者获取新Cookie
+ [x] 使用`GetAuthToken`获取`szToken`，用于构建二维码。
+ [x] 手机扫码访问`UniAuth.aspx`界面进行授权
+ [x] 可以使用阻塞的HTTP访问判断是否授权完成
+ [x] 此时Cookie生效，使用`Auth/Check`可以判断
+ [x] 持久化Cookie
+ [x] 后续可以随意调用API。
+ [x] 从打印任务中获取必要信息
+ [x] 发送HTTP请求，创建Job
+ [x] ~~使用`Compress.dll`压缩PJL文件~~Use `gzip`.
+ [x] 发送HTTP请求，上传文件
+ [x] 发送结束请求
+ [ ] 清除打印队列内容和临时文件。
+ [x] 类Unix系统支持 
  + [ ] ~~编写CUPS Backend~~
  + [ ] ~~编写PPD文件~~
  + [x] Use Socket Backend
  + [x] PCL Intepretation (GhostPDL?)
+ [ ] 查看当前在线队列
+ [ ] 查询历史打印信息
+ [ ] ~~使用IPP提供服务~~
+ [ ] 接入网页端PDF上传接口

# Dependencies

+ GhostPCL(PDL): AGPL 3.0

# License

AGPL 3.0

# Build

Checkout GitHub Action Files