@echo off
echo Build...

2goarray Icon main < ribbon.ico > iconwin.go

rsrc -manifest comctrl6.manifest -ico ribbon.ico -o rsrc.syso

setlocal

echo 64 bit windows...
set GOOS=windows
set GOARCH=amd64

go build -ldflags="-s -w -H windowsgui" -o 0416.exe

echo done.

endlocal
