# **R**upmPrinter
**R**upmPrinter is designed to be a _multi-platform_ __printer driver__ & __frontend application__ for SJTU Library Printing System.

It is a reversed version of the official `UPMClient` Windows driver from `Unifound`.

**R** as in `Rust`/`Reverse`/`Rewrite`.

On `Windows` it works as an **extension** of the original driver & port monitor (for now)

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
+ [ ] 类Unix系统支持 
  + [ ] 编写CUPS Backend
  + [ ] 编写PPD文件
+ [ ] 查看当前在线队列
+ [ ] 查询历史打印信息
+ [ ] 使用IPP提供服务
+ [ ] 接入网页端PDF上传接口
# API
## Common API

## Web API

## UPMClient API
+ `POST /api/client/PrintJob/Create`
    ```json
    {
    "dwProperty": 0,
    "szJobName": "屏幕截图 2024-10-02 233633",
    "dwCopies": 1,
    "szAttribe": "single,collate,NUP1,", // "color,vdup,collate,NUP1," // vdup means Vertical Duplex. // "color,hdup,collate,NUP1,"
    "szPaperDetail": "[{\"dwPaperID\":9,\"dwBWPages\":1,\"dwColorPages\":0,\"dwPaperNum\":1}]", // dwPaperID 9 means A4 dwBWPages 5 dwColorPages 0, dwPaperNum 3. // Notice dwPaper Num is calculated by total paper number.
    "szColorMap": "0" // "11101" Color
    }
    ```
    ```json
    {
        "code": 0,
        "message": "",
        "result": {
            "szLogonName": "",
            "dwStatus": 17,
            "dwJobId": 167523,
            "dwOldJobId": 579850,
            "dwProperty": 33554432,
            "dwCreateDate": 20241005,
            "dwCreateTime": 213611,
            "dwPrintDate": 0,
            "dwPrintTime": 0,
            "dwCopies": 1,
            "dwDevSN": 0,
            "dwFee": 0,
            "dwType": 1,
            "dwSpecPrinter": 0,
            "dwFSID": 0,
            "szAttribe": "single,collate,NUP1,",
            "szPaperDetail": "[{\"dwPaperID\":9,\"dwBWPages\":1,\"dwColorPages\":0,\"dwPaperNum\":1}]",
            "szColorMap": "0",
            "szCardNO": "",
            "szTrueName": "",
            "szJobName": " 2024-10-02 233633",
            "szFileName": "20241005/f4a5169164ba403a87fcbb572f7aa6ee",
            "szMemo": null
        }
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

# “联创打印管理系统”虚拟打印机执行流程

## 安装目录结构
```
C:\upmclient
├─log
├─mondll # 储存端口监视器文件 `upmlocalmon.dll`
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
│  Compress.dll # 用于`gzip`压缩。用于压缩PJL文件生成`.tmp2`后缀文件，UPMClient调用
│  GdiPlus.dll # Gdi+ 是 Windows 的图形API。
│  opmgoupm.exe # 似乎是一个浏览器。打开会进入eprint主页。
│  Setup.exe # 安装UPMClient驱动（使用`UPMPortMonitor.reg`更新注册表）
│  UnInstall.cmd # 带参数执行`Setup.exe`
│  update.exe
│  UPMClient.exe # 主程序。不加参数启动可以进入eprint主页，被端口监视器使用命令行参数调用会进入打印界面。
│  UPMClient.ini # 配置文件，作为注册表的备选。
│  UPMPortMonitor.reg # 配置端口监视器的注册表脚本
└─ zlib1.dll # 用于`zlib`压缩
```

使用Ghidra进行逆向。

## 驱动部分
+ 驱动是Unidrv，由GPF文件定义GDI指令到PCL指令的转换。内嵌在Setup.exe中，安装时添加到系统驱动目录。

## 端口监视器部分
+ 端口监视器会储存驱动发送到该端口的PJL文件于`temp`目录，命名为`[JobId].tmp`
+ 驱动会使用命令行启动`UMPClient.exe`，参数为`/JOB:[JobId] /PRINTER:联创打印管理系统`

## `UPMClient.exe`部分
+ 使用传入的Job参数和打印机名称使用GetPrinter等Win32API检查打印任务队列
+ 生成预览文件`.pvg`（实现未知, Should be Using GDI+ to render EMF spooler file）
    + `.pvg` is a self-defined format containing base64 encoded png.
+ 向后端发送添加任务的请求，获得新任务的id。
+ 调用`Compress.dll`中的`CompressFile(input, output, "wb")`获得`[JobId].tmp2`（实为`gzip`压缩）
+ 若点击预览，生成`.raw` PNG 图片文件
+ 向后端上传压缩后的文件
+ 向后端上传预览文件？（实现未知）
+ 向后端发送结束请求
+ 清除临时文件，删除打印任务
+ 清除临时文件，删除打印任务

# Dev Notes
> 向打印机传输的文件是什么？`.tmp`是什么文件？

Spooler队列中是EMF文件

Spooler队列中是EMF文件

>The print spooler supports the following data types:
>
> Enhanced metafile (EMF).
ASCII text.
Raw data, which includes printer data types such as PostScript, PCL, and custom data types.

交给语言处理器（驱动）后生成PJL文件。

交给语言处理器（驱动）后生成PJL文件。

PJL文件，惠普有参考资料。内部数据部分是PCL。
+ [HP PCL](https://developers.hp.com/hp-printer-command-languages-pcl)
+ [HP Printer Job Language
Technical Reference Manual](https://developers.hp.com/sites/default/files/PJL_Technical_Reference_Manual.pdf)

> 使用的是哪个版本的PCL文件

1990年的[PCL5](https://developers.hp.com/hp-printer-command-languages-pcl/doc/pcl5)。

可以使用HP的[PCL5通用驱动](https://superuser.com/questions/1797510/where-can-i-find-a-windows-10-pcl5-driver-for-an-unsupported-laserjet-printer)(HP Universal Print Driver for Windows - PCL 5，版本号：upd-pcl5-x64-6.1.0.20062，目前官网已不提供下载)打印到端口。

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

~~> 如何使用Rust加载dll？~~

~~好像也有`crate`~~


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
    + Can be combine with custom filters like `foomatic-rip` or [`gs`](https://www.ghostscript.com/)
    + Custom backend like [cups-backend](https://www.cups.org/doc/man-backend.html)

Generate PPD Files for CUPS.

Backend Example: https://www.cups-pdf.de/download.shtml

https://www.cups.org/doc/man-backend.html

PPD Example: Can be downloaded from OpenPrinting. Using Generic PCL5?

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

> Windows怎么添加打印机

+ 选择端口
驱动处理完的数据最终如何发送给打印机。
有常见的如Socket/USB。
通过安装端口监控程序（使用INF）可以添加新的端口。

+ 选择驱动
决定了发往端口的数据。

> PVG File Format?

+ 12 Byte Header ("PVG")
+ Entry
  + 4 Byte Size (Last Entry: 12 Bytes)
  + Base-64 Encoded PNG File (Res: 585x838) (Last Entry: Magic Bytes)

# CUPS
+ `/etc/cups/ppd`: Stores PPD File?
+ `/usr/lib/cups`: Stores Backend
+ `/usr/share/cups`: ??
+ `/usr/share/ppd/cupsfilters/`: Default PPD Files.

ipp backend?

CUPS Filter on Debian Wiki

# Get a PPD File

`/usr/sbin/lpinfo -m | less`

`-m`参数：决定过滤器输出。
+ 这些MIME类型定义在`/usr/share/cups/mime`中。
+ MIME间如何互转也定义在其中。
+ 默认：`application/pdf`，作为Filter间一般的传递文件。
+ application/postscript
+ application/vnd.cups-pdf：真的PDF
+ application/vnd.cups-postscript
+ application/vnd.cups-raster：CUPS Raster File
+ `printer/foo`表示使用`-p`指定的PPD文件中的Filters流。

查看过程中使用的过滤器

/usr/sbin/cupsfilter -p laserjet2200.ppd -m printer/foo -e --list-filters


`/usr/sbin/cupsfilter -p laserjet2200.ppd -m application/vnd.cups-pdf -o number-up=2 test.ps > out.pdf 2> log`
`pdftopdf`。实现了nUP变换的PDF。

The latter filter performs the very important task of page management; the application of N-up is obvious in a PDF viewer.

`-e` option. Use Filter in PPD File.

`/usr/sbin/cupsfilter -p laserjet2200.ppd -m printer/foo -e -o number-up=2 --list-filters test.ps`

`/usr/sbin/cupsfilter -p laserjet2200.ppd -m printer/foo -o number-up=2 test.ps -e > out.pcl 2> log`

*cupsFilter or *cupsFilter2 line. This will be the last filter applied in the filter chain; laserjet2200.ppd has
 -m "printer/foo" is nessesary...

*cupsFilter "application/vnd.cups-raster 100 rastertogutenprint 5.2"

`:/usr/share/cups/mime`

-m mime/type
Specifies the destination file type. The default file type is application/pdf. Use **printer/foo** to convert to the printer format defined by the filters in the PPD file.

/usr/sbin/cupsfilter -e  -p /usr/share/ppd/cupsfilters/pxlmono.ppd test.ps -m printer/foo > test2.pcl

+ A PPD For Printer.
+ A PPD For Preview.
gstopdf

/usr/lib/cups/filter/gstopxl


cupsFilter
*cupsFilter: "source/type cost program"

This string keyword provides a conversion rule from the given source type to the printer's native format using the filter "program". If a printer supports the source type directly, the special filter program "-" may be specified.

Examples:

*% Standard raster printer driver filter
*cupsFilter: "application/vnd.cups-raster 100 rastertofoo"

*% Plain text filter
*cupsFilter: "text/plain 10 texttofoo"

*% Pass-through filter for PostScript printers
*cupsFilter: "application/vnd.cups-postscript 0 -"
CUPS 1.5cupsFilter2
*cupsFilter2: "source/type destination/type cost program"

This string keyword provides a conversion rule from the given source type to the printer's native format using the filter "program". If a printer supports the source type directly, the special filter program "-" may be specified. The destination type is automatically created as needed and is passed to the filters and backend as the FINAL_CONTENT_TYPE value.

Note:
The presence of a single cupsFilter2 keyword in the PPD file will hide any cupsFilter keywords from the CUPS scheduler. When using cupsFilter2 to provide filters specific for CUPS 1.5 and later, provide a cupsFilter2 line for every filter and a cupsFilter line for each filter that is compatible with older versions of CUPS.

Examples:

*% Standard raster printer driver filter
*cupsFilter2: "application/vnd.cups-raster application/vnd.foo 100 rastertofoo"

*% Plain text filter
*cupsFilter2: "text/plain application/vnd.foo 10 texttofoo"

*% Pass-through filter for PostScript printers
*cupsFilter2: "application/vnd.cups-postscript application/postscript 0 -"

PCL 5 Parser?

```
string* "\x1B&l0O"    // Orientation
string* "\x1B&l26A"    // Page Size
string* "\x1B&l7H"    // Media Source
string* "\x1B&l0S"    // Simplex/Duplex Mode

// Color Checking

string* "\x1B*v6W"    // Configure Image Data (CID)
hex_raw* [ 00 03 08 08 08 08 ]
string* "\x1B*v0a"    // Color Component 1
      string* "0b"    // Color Component 2
      string* "0c"    // Color Component 3
      string* "7i"    // Assign Color Index
      string* "255a"    // Color Component 1
      string* "255b"    // Color Component 2
      string* "255c"    // Color Component 3
      string* "0I"    // Assign Color Index

// End of Page

string* "\x0C" // Form Feed
```

.\gpcl6win64.exe -sDEVICE=png16 -o %03d.png  -r100 C:\upmclient\temp\17.tmp