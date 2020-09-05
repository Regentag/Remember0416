@echo off
rem Icon convert: https://icoconvert.com/

2goarray Icon main < ribbon.ico > iconwin.go

rsrc -manifest comctrl6.manifest -ico ribbon.ico -o rsrc.syso

go build -ldflags="-s -w -H windowsgui"
