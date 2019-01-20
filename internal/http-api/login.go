package httpapi

import (
	"encoding/json"
	"fmt"
	"strings"
)

// Login sign in a user with api key and secret
func Login(key, secret string) {

	data := map[string]string{"apiKey": key, "apiSecret": secret}

	status, body, err := SendJSON(loginURL, "", data)

	if err != nil {
		fmt.Printf("Unable to retrieve token %v", err)
	}

	if strings.HasPrefix(status, statusOK) {
		var resp LoginResponse
		err := json.Unmarshal(body, &resp)
		if err != nil {
			fmt.Println("error:", err)
		}

		fmt.Printf("JWT Token: %s", resp.Token)
	} else {
		fmt.Printf("Status %s - %v", status, body)
	}
}

// LoginResponse represents the response of the login
type LoginResponse struct {
	Token string `json:"token"`
}
