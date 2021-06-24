package server

import (
	"fmt"
	log "github.com/sirupsen/logrus"
	"html/template"
	"net/http"
	"strconv"
)

type ResultTemplate struct {
	Result string
}

const LOGIN = "human"
const PASSWORD = "iamnotbot"

const HeaderRequestId = "botd-request-id"
const HeaderRequestStatus = "botd-request-status"

const HeaderBotStatus = "botd-automation-tool-status"
const HeaderBotProb = "botd-automation-tool-prob"
const HeaderBotType = "botd-automation-tool-type"

const HeaderSearchBotStatus = "botd-search-bot-status"
const HeaderSearchBotProb = "botd-search-bot-prob"
const HeaderSearchBotType = "botd-search-bot-type"

const HeaderVMStatus = "botd-vm-status"
const HeaderVMProb = "botd-vm-prob"
const HeaderVMType = "botd-vm-type"

const HeaderBrowserSpoofingStatus = "botd-browser-spoofing-status"
const HeaderBrowserSpoofingProb = "botd-browser-spoofing-prob"
const HeaderBrowserSpoofingType = "botd-browser-spoofing-type"

func logError(err error, msg string) {
	if err != nil {
		log.Warn(msg, err.Error())
	}
}

func loginHandler(w http.ResponseWriter, r *http.Request) {
	err := r.ParseForm()
	logError(err, "Form parse error: ")

	requestId := r.Header.Get(HeaderRequestId)

	if requestId == "" {
		log.Error("Empty " + HeaderRequestId + " header!")
	} else {
		log.Info(HeaderRequestId + " = " + requestId)

		requestStatus := r.Header.Get(HeaderRequestStatus)
		log.Info(HeaderRequestStatus + " = " + requestStatus)

		if requestStatus == "processed" {
			botStatus := r.Header.Get(HeaderBotStatus)
			botProb := r.Header.Get(HeaderBotProb)
			botType := r.Header.Get(HeaderBotType)

			botProbFloat, err := strconv.ParseFloat(botProb, 32)
			logError(err, "Can`t cast botProb to float: ")

			log.Info(HeaderBotStatus + " = " + botStatus)
			log.Info(HeaderBotProb + " = " + botProb)
			log.Info(HeaderBotType + " = " + botType)

			searchBotStatus := r.Header.Get(HeaderSearchBotStatus)
			searchBotProb := r.Header.Get(HeaderSearchBotProb)
			searchBotType := r.Header.Get(HeaderSearchBotType)

			searchBotProbFloat, err := strconv.ParseFloat(searchBotProb, 32)
			logError(err, "Can`t cast searchBotProb to float: ")

			log.Info(HeaderSearchBotStatus + " = " + searchBotStatus)
			log.Info(HeaderSearchBotProb + " = " + searchBotProb)
			log.Info(HeaderSearchBotType + " = " + searchBotType)

			vmStatus := r.Header.Get(HeaderVMStatus)
			vmProb := r.Header.Get(HeaderVMProb)
			vmType := r.Header.Get(HeaderVMType)

			vmProbFloat, err := strconv.ParseFloat(vmProb, 32)
			logError(err, "Can`t cast vmProb to float: ")

			log.Info(HeaderVMStatus + " = " + vmStatus)
			log.Info(HeaderVMProb + " = " + vmProb)
			log.Info(HeaderVMType + " = " + vmType)

			browserSpoofingStatus := r.Header.Get(HeaderBrowserSpoofingStatus)
			browserSpoofingProb := r.Header.Get(HeaderBrowserSpoofingProb)
			browserSpoofingType := r.Header.Get(HeaderBrowserSpoofingType)

			browserSpoofingProbFloat, err := strconv.ParseFloat(browserSpoofingProb, 32)
			logError(err, "Can`t cast browserSpoofingProb to float: ")

			log.Info(HeaderBrowserSpoofingStatus + " = " + browserSpoofingStatus)
			log.Info(HeaderBrowserSpoofingProb + " = " + browserSpoofingProb)
			log.Info(HeaderBrowserSpoofingType + " = " + browserSpoofingType)

			if botProbFloat+searchBotProbFloat+vmProbFloat+browserSpoofingProbFloat > 0 {
				resultString := fmt.Sprintf("{\"status\":\"%s\",\"bot\":{\"automationTool\":{\"status\":\"%s\",\"probability\":%s,\"type\":\"%s\"},\"browserSpoofing\":{\"status\":\"%s\",\"probability\":%s,\"type\":\"%s\"},\"searchEngine\": {\"status\":\"%s\",\"probability\":%s,\"type\":\"%s\"}},\"vm\":{\"status\":\"%s\",\"probability\":%s,\"type\":\"%s\"}}",
					requestStatus, botStatus, botProb, botType, browserSpoofingStatus, browserSpoofingProb, browserSpoofingType, searchBotStatus, searchBotProb, searchBotType, vmStatus, vmProb, vmType)

				log.Info(resultString)

				p := ResultTemplate{
					Result: resultString,
				}

				t, err := template.ParseFiles("static/templates/is_bot.html")
				logError(err, "Server error: ")

				err = t.Execute(w, p)
				logError(err, "Can`t execute template: ")

				return
			}
		}
	}

	formLogin := r.FormValue("login")
	formPassword := r.FormValue("password")

	p := ResultTemplate{Result: "Wrong login or password"}

	if formLogin == LOGIN && formPassword == PASSWORD {
		p.Result = "Welcome back, " + LOGIN
	}

	t, err := template.ParseFiles("static/templates/not_bot.html")
	logError(err, "Server error: ")

	err = t.Execute(w, p)
	logError(err, "Can`t execute template: ")

	return
}
