# Tasks
+ [ ] 判断客户端上传是否一定需要IP地址作为目标？
+ [x] 首先读取Cookie, 使用`Auth/Check`判断或者获取新Cookie
+ [x] 使用`GetAuthToken`获取`szToken`，用于构建二维码。
+ [x] 手机扫码访问`UniAuth.aspx`界面进行授权
+ [x] 可以使用阻塞的HTTP访问判断是否授权完成
+ 此时Cookie生效，使用`Auth/Check`可以判断
+ 持久化Cookie
+ 后续可以随意调用API。
+ 从打印任务中获取必要信息（？）
+ 发送HTTP请求，创建Job
+ 使用`Compress.dll`压缩PJL文件
+ 发送HTTP请求，上传文件
+ 发送结束请求
+ 清除打印队列内容和临时文件。
+ 查看当前在线队列

# API
## Common API

## Web API

## UPMClient API
+ `GET /api/client/PrintJob/Create`
    ```
    {
    "dwProperty": 0,
    "szJobName": "屏幕截图 2024-10-02 233633",
    "dwCopies": 1,
    "szAttribe": "single,collate,NUP1,",
    "szPaperDetail": "[{\"dwPaperID\":9,\"dwBWPages\":1,\"dwColorPages\":0,\"dwPaperNum\":1}]",
    "szColorMap": "0"
    }
    ```
+ `POST /api/client/PrintJob/Upload?dwJobId={} HTTP/1.1 `
```
multipart/form-data; boundary=---------------------------Boundaryd1vksiw0aMcjdDd46cs3c
```
+ `POST /api/client/PrintJob/UploadPreview?dwJobId={} HTTP/1.1`
+ `POST /api/client/PrintJob/Set HTTP/1.1`
    ```
    Object
        Member: dwJobId
            [Path with value: /dwJobId:167523]
            [Member with value: dwJobId:167523]
            Number value: 167523
            Key: dwJobId
            [Path: /dwJobId]
        Member: dwStatus
            [Path with value: /dwStatus:1]
            [Member with value: dwStatus:1]
            Number value: 1
            Key: dwStatus
            [Path: /dwStatus]
        Member: OSESSIONID
            [Path with value: /OSESSIONID:2a50d60c07f44c0aace049be7aff5fcf]
            [Member with value: OSESSIONID:2a50d60c07f44c0aace049be7aff5fcf]
            String value: 2a50d60c07f44c0aace049be7aff5fcf
            Key: OSESSIONID
            [Path: /OSESSIONID]
        ```
## Charge API
### 获取用户余额
+ `GET /uniwx/UserSet.aspx?m=1&id={jAccountId} HTTP/1.1`
    + 无需鉴权（危险）

### 充值

> 注意：该部分流程尚不清晰，胡乱调用可能导致充值给虚空。

+ `GET /uniwx/s.aspx?c=recharge_1_{jAccountId} HTTP/1.1` 
    + 进行一系列OAuth 2鉴权。
+ 跳转到`GET /uniwx/pay.aspx?state=recharge_1_{jAccountId} HTTP/1.1`
    + 该界面下有Token
    + 有一个填写充值金额的框
+ `POST /uniwx/pay.aspx?state=recharge_1_{jAccountId} HTTP/1.1`
    + 提交表单发送POST请求可以充值
    + 包括Token和充值金额
+ 返回一个带微信支付脚本的网页

# Win32 API
## 打印队列相关

[打印后台处理程序 API 函数](https://learn.microsoft.com/zh-cn/windows/win32/printdocs/printing-and-print-spooler-functions)

# “联创打印管理系统”虚拟打印机执行流程

## 安装目录结构
```
C:\upmclient
├─log
├─mondll # 储存驱动文件 `upmlocalmon.dll`
│  ├─Win10X32
│  ├─Win10X64
│  ├─Win7X32
│  ├─Win7X64
│  ├─Win8X32
│  ├─Win8X64
│  ├─Win8_1X32
│  ├─Win8_1X64
│  └─WinXPX32
├─temp # 临时文件。包括PJL文件(.tmp后缀)、PVG文件（预览信息？）、RAW图片文件、压缩的PJL文件(.tmp2)后缀
├─X32
├─X64
│  cconnector.dll # 用于JsonRPC，UPMClient调用
│  Compress.dll # 用于压缩PJL文件生成`.tmp2`后缀文件，UPMClient调用
│  GdiPlus.dll # Gdi+ 是 Windows 的图形API
│  opmgoupm.exe # 似乎是一个浏览器
│  Setup.exe # 安装UPMClient驱动（使用`UPMPortMonitor.reg`更新注册表）
│  UnInstall.cmd # 带参数执行`Setup.exe`
│  update.exe
│  UPMClient.exe # 主程序。不加参数启动可以进入eprint主页，被驱动使用加命令行参数调用会进入打印界面。
│  UPMClient.ini # 配置文件，作为注册表的备选。
│  UPMPortMonitor.reg
└─ zlib1.dll
```

使用Ghidra进行逆向。

## 驱动部分
+ 驱动是Unidrv，由GPF文件定义GDI指令到PCL指令的转换。内嵌在Setup.exe中

## 端口监视器部分
+ 端口监视器会储存驱动发送到该端口的PJL文件于`temp`目录，命名为`[JobId].tmp`
+ 驱动会使用命令行启动`UMPClient.exe`，参数为`/JOB:[JobId] /PRINTER:联创打印管理系统`

## `UPMClient.exe`部分
+ 使用传入的Job参数和打印机名称使用GetPrinter等Win32API检查打印任务队列
+ 向后端发送添加任务的请求，获得新任务的id。
+ 调用`Compress.dll`中的`CompressFile(input, output, "wb")`获得`[JobId].tmp2`
+ 似乎还生成预览文件`.pvg`并上传
+ 若点击预览，生成`.raw`图片文件
+ 向后端上传压缩后的文件
+ 向后端发送结束请求
+ 清除临时文件，删除打印任务

# Dev Notes
> 向打印机传输的文件是什么？`.tmp`是什么文件？

Spooler队列中是EMF文件

>The print spooler supports the following data types:
>
> Enhanced metafile (EMF).
ASCII text.
Raw data, which includes printer data types such as PostScript, PCL, and custom data types.

交给语言处理器（驱动）后生成PJL文件。

PJL文件，惠普有参考资料。内部数据部分是PCL。
+ [HP PCL](https://developers.hp.com/hp-printer-command-languages-pcl)
+ [HP Printer Job Language
Technical Reference Manual](https://developers.hp.com/sites/default/files/PJL_Technical_Reference_Manual.pdf)

> Windows是如何控制打印机的？如何删除打印任务？

[打印后台处理程序 API 函数](https://learn.microsoft.com/zh-cn/windows/win32/printdocs/printing-and-print-spooler-functions)

若使用C/C++编写，记得引入`windows.h`

```C
SetJob(hPrinter, `job_id`, 0, NULL, JOB_CONTROL_DELETE);
```

> 如何使用Rust调用Win32 API？如何使用Unsafe Rust？

使用`windows-rs` crate

> 文件是怎么压缩的？

调用了`Compress.dll`，内部实现尚未逆向。

> 如何使用Rust加载dll？

好像也有`crate`


> 有些东西写在源代码中，但Fiddler没抓到？

使用`WireShark`抓到了。

> 如何持久化Cookie？

`Reqwest`库有`CookieStorage`，稍加修改。
参考`Canvas Helper`。

> Unix-like Systems

Use CUPS.

![CUPS Architecture](https://www.cups.org/images/cups-postscript-chain.png)

+ [Raster Driver (PCL)](https://www.cups.org/doc/raster-driver.html)
+ [PostScript Driver](https://www.cups.org/doc/postscript-driver.html)
    + Can be combined with [gs](https://www.ghostscript.com/) to generate pdf file.
    + Custom backend like [cups-backend](https://www.cups.org/doc/man-backend.html)

Generate PPD Files for CUPS.
```
// Include standard font and media definitions
#include <font.defs>
#include <media.defs>

// Specify this is a PostScript printer driver
DriverType ps
```

> Windows怎么添加打印机

+ 选择端口
驱动处理完的数据最终如何发送给打印机。
有常见的如Socket/USB。
通过安装端口监控程序（使用INF）可以添加新的端口。

+ 选择驱动
决定了发往端口的数据。
