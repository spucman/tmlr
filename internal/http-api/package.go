package httpapi

import (
	"fmt"
	"net/http"
	"time"
)

const (
	serverName = "api.timeular.com"
	apiVersion = "v2"

	statusOK = "200"
)

var (
	baseURL    = fmt.Sprintf("https://%s/api/%s", serverName, apiVersion)
	loginURL   = fmt.Sprintf("%s/developer/sign-in", baseURL)
	httpClient = &http.Client{
		Timeout: time.Second * 60,
	}
)
