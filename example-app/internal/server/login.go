package server

import (
	"encoding/json"
	"github.com/sirupsen/logrus"
	"net/http"
)

type Description struct {
	Code        int    `json:"code"`
	Description string `json:"description"`
}

type ErrorResult struct {
	Error Description `json:"error"`
}

type MessageResult struct {
	Message Description `json:"message"`
}

func loginHandler(w http.ResponseWriter, r *http.Request) {
	err := r.ParseForm()
	if err != nil {
		logrus.Warn("Form parse error: ", err.Error())
	}

	requestId := r.Header.Get("fpjs-request-id")
	if requestId != "" {
		logrus.Info("Received request from bot, detailed information:")
		logrus.Info("fpjs-request-id = " + requestId)
		requestStatus := r.Header.Get("fpjs-request-status")
		logrus.Info("fpjs-request-status = " + requestStatus)
		if requestStatus == "ok" {
			logrus.Info("fpjs-bot-status = " + r.Header.Get("fpjs-bot-status"))
			logrus.Info("fpjs-bot-prob = " + r.Header.Get("fpjs-bot-prob"))
			logrus.Info("fpjs-bot-type = " + r.Header.Get("fpjs-bot-type"))

			logrus.Info("fpjs-search-bot-status = " + r.Header.Get("fpjs-search-bot-status"))
			logrus.Info("fpjs-search-bot-prob = " + r.Header.Get("fpjs-search-bot-prob"))
			logrus.Info("fpjs-search-bot-type = " + r.Header.Get("fpjs-search-bot-type"))

			logrus.Info("fpjs-vm-status = " + r.Header.Get("fpjs-vm-status"))
			logrus.Info("fpjs-vm-prob = " + r.Header.Get("fpjs-vm-prob"))
			logrus.Info("fpjs-vm-type = " + r.Header.Get("fpjs-vm-type"))

			logrus.Info("fpjs-browser-spoofing-status = " + r.Header.Get("fpjs-browser-spoofing-status"))
			logrus.Info("fpjs-browser-spoofing-prob = " + r.Header.Get("fpjs-browser-spoofing-prob"))
		}
		return
	}

	login := r.FormValue("login")
	password := r.FormValue("password")

	if login != "human" || password != "iamnotbot" {
		w.WriteHeader(http.StatusUnauthorized)
		err := json.NewEncoder(w).Encode(ErrorResult{
			Error: Description{
				Code:        http.StatusUnauthorized,
				Description: "Incorrect login or password",
			},
		})
		if err != nil {
			logrus.Warn("ErrorResult encode error: ", err.Error())
		}
		return
	}

	success := json.NewEncoder(w).Encode(MessageResult{
		Message: Description{
			Code:        http.StatusOK,
			Description: "You are successfully logged in!",
		},
	})
	if success != nil {
		logrus.Warn("ErrorResult encode error: ", success.Error())
	}
	return
}
