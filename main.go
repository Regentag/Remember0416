package main

import (
	"0416/systray"
	"0416/toast"
	"fmt"
	"log"
	"os"
	"os/exec"
	"path"
	"path/filepath"
	"runtime"
	"time"
)

const (
	// 윈도 Toast의 App ID
	APP_ID = "Remember 0416"

	// Toast 메시지 제목
	TITLE = "2014.4.16.  SEWOL."

	// 출력될 본문 메시지.
	MSG = "우리가 기억하는 한\n진실은 절대 침몰하지 않습니다.\n\n사고일로부터 %d일."

	// 나무위키 사고정보 페이지
	URL = "https://namu.wiki/w/%EC%B2%AD%ED%95%B4%EC%A7%84%ED%95%B4%EC%9A%B4%20%EC%84%B8%EC%9B%94%ED%98%B8%20%EC%B9%A8%EB%AA%B0%20%EC%82%AC%EA%B3%A0"

	// 프로그램 정보 페이지
	PROGRAM_INFO = "https://github.com/Regentag/Remember0416"
)

func main() {
	systray.Run(onReady, onExit)
}

func onReady() {
	systray.SetIcon(Icon)
	systray.SetTooltip(APP_ID)

	click := systray.AddLeftClickMenuItem()

	mInfo := systray.AddMenuItem("세월호 침몰 사고 정보(나무위키)", "")
	mAbout := systray.AddMenuItem("프로그램 정보", "")
	systray.AddSeparator()
	mQuit := systray.AddMenuItem("종료", "")

	go func() {
		for {
			select {
			case <-click.ClickedCh:
				showToast()
			case <-mInfo.ClickedCh:
				openBrowser(URL)
			case <-mAbout.ClickedCh:
				openBrowser(PROGRAM_INFO)
			case <-mQuit.ClickedCh:
				systray.Quit()
			}
		}
	}()
}

func onExit() {
	// clean up here
}

// 메시지 출력
func showToast() {
	msg := fmt.Sprintf(MSG, dayAfter())
	iconPath := path.Join(getExecPath(), "ribbon.ico")

	notification := toast.Notification{
		AppID:   APP_ID,
		Title:   TITLE,
		Message: msg,
		Icon:    iconPath,
		Actions: []toast.Action{
			//{Type: "protocol", Label: "사고 정보 보기", Arguments: URL},
			{Type: "system", Label: "닫기", Arguments: "dismiss"},
		},
		Duration: toast.Long,
	}
	err := notification.Push()
	if err != nil {
		log.Fatalln(err)
	}
}

// URL을 브라우저에서 열기.
// code from https://gist.github.com/nanmu42/4fbaf26c771da58095fa7a9f14f23d27#file-openinbrowser-go
func openBrowser(url string) {
	var err error

	switch runtime.GOOS {
	case "linux":
		err = exec.Command("xdg-open", url).Start()
	case "windows":
		err = exec.Command("rundll32", "url.dll,FileProtocolHandler", url).Start()
	case "darwin":
		err = exec.Command("open", url).Start()
	default:
		err = fmt.Errorf("unsupported platform")
	}
	if err != nil {
		log.Fatal(err)
	}
}

// 사고일로부터의 경과일수를 계산하여 반환.
// 2014. 4. 16. 00:00을 기준으로 합니다. (나무위키의 일자 계산과 일치)
func dayAfter() int {
	now := time.Now()
	format := "2006-01-02 15:04:05"
	then, _ := time.Parse(format, "2014-04-16 00:00:00")

	diff := now.Sub(then)

	return int(diff.Hours() / 24)
}

// 실행파일이 설치된 위치를 반환.
func getExecPath() string {
	dir, err := filepath.Abs(filepath.Dir(os.Args[0]))
	if err != nil {
		log.Fatal(err)
	}
	return dir
}
