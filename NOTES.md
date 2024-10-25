

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

> 使用的是哪个版本的PCL文件

1990年的[PCL5](https://developers.hp.com/hp-printer-command-languages-pcl/doc/pcl5)。

可以使用HP的[PCL5通用驱动](https://superuser.com/questions/1797510/where-can-i-find-a-windows-10-pcl5-driver-for-an-unsupported-laserjet-printer)(HP Universal Print Driver for Windows - PCL 5，版本号：upd-pcl5-x64-6.1.0.20062，目前官网已不提供下载)打印到端口。

Should support PCL6 as well.

> Windows是如何控制打印机的？如何删除打印任务？

[打印后台处理程序 API 函数](https://learn.microsoft.com/zh-cn/windows/win32/printdocs/printing-and-print-spooler-functions)

若使用C/C++编写，记得引入`windows.h`

```C
SetJob(hPrinter, `job_id`, 0, NULL, JOB_CONTROL_DELETE);
```

> 如何使用Rust调用Win32 API？如何使用Unsafe Rust？

使用`windows-rs` crate

> 文件是怎么压缩的？

~~调用了`Compress.dll`，内部实现尚未逆向。~~

Using `gzip`

> 如何使用Rust加载dll？

好像也有`crate`, `libloading`

> How to get `.lib` from `.dll` files?



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

Choose PPD Files & Use Socket Backend

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
+ `/var/log/cups`: Debug Log

ipp backend?

CUPS Filter on Debian Wiki

## Get a PPD File
`lpinfo -m`
`/usr/sbin/lpinfo -m | less`
## ~~CUPS Filter~~

Checkout Discription on [Debian Site](https://wiki.debian.org/CUPSFilter).

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
`pdftopdf`。实现了`number-up`变换的PDF。

The latter filter performs the very important task of page management; the application of N-up is obvious in a PDF viewer.

`-e` option. Use Filter in PPD File.

`/usr/sbin/cupsfilter -p laserjet2200.ppd -m printer/foo -e -o number-up=2 --list-filters test.ps`

`/usr/sbin/cupsfilter -p laserjet2200.ppd -m printer/foo -o number-up=2 test.ps -e > out.pcl 2> log`

*cupsFilter or *cupsFilter2 line. This will be the last filter applied in the filter chain; laserjet2200.ppd has
 -m "printer/foo" is nessesary...

*cupsFilter "application/vnd.cups-raster 100 rastertogutenprint 5.2"

`/usr/share/cups/mime`

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

## Register as a Socket Printer

```
lpadmin -p RupmPrinter -E -v socket://127.0.0.1:12345 -m drv:///sample.drv/laserjet.ppd
```

Send PCL Instruction through Socket

### Build GhostPCL
```
./autogen.sh --with-drivers=PNG --without-libtiff --without-libidn --without-libpaper  --without-tesseract --without-ijs  --without-urf  --without-so   --without-cal --without-pdftoraster --with-pcl=gpcl6 --with-pdf=no -with-gpdl=no --with-gs=no --with-xps=no --without-jbig2dec --disable-gtk --disable-cups --disable-openjpeg

make gpcl6
make libgpcl6
```
