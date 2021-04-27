package server

import (
	"encoding/json"
	"github.com/sirupsen/logrus"
	"io/ioutil"
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

type LoginRequest struct {
	Login    string `json:"login"`
	Password string `json:"password"`
}

func loginHandler(w http.ResponseWriter, r *http.Request) {
	setCorsAllowAll(w)

	b, err := ioutil.ReadAll(r.Body)
	if err != nil {
		err := json.NewEncoder(w).Encode(ErrorResult{
			Error: Description{
				Code:        http.StatusBadRequest,
				Description: err.Error(),
			},
		})
		if err != nil {
			logrus.Warn("ErrorResult encode error: ", err.Error())
		}
		return
	}
	defer r.Body.Close()

	var payload LoginRequest
	if err = json.Unmarshal(b, &payload); err != nil {
		err := json.NewEncoder(w).Encode(ErrorResult{
			Error: Description{
				Code:        http.StatusBadRequest,
				Description: err.Error(),
			},
		})
		if err != nil {
			logrus.Warn("ErrorResult encode error: ", err.Error())
		}
		return
	}

	if payload.Login != "human" || payload.Password != "iamnotbot" {
		err := json.NewEncoder(w).Encode(ErrorResult{
			Error: Description{
				Code:        http.StatusUnauthorized,
				Description: "Wrong login or password",
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
