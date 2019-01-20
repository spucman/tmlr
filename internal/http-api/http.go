package httpapi

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
)

//SendJSON sends the given data via JSON Post to the server
func SendJSON(url, token string, data interface{}) (string, []byte, error) {
	jsonValue, err := json.Marshal(data)

	req, err := http.NewRequest("POST", url, bytes.NewBuffer(jsonValue))
	if err != nil {
		return "", nil, err
	}

	if token != "" {
		req.Header.Set("Authorization", fmt.Sprintf("Bearer %s", token))
	}
	req.Header.Set("Content-Type", "application/json")

	return doSend(req)
}

func doSend(req *http.Request) (string, []byte, error) {
	resp, err := httpClient.Do(req)
	if err != nil {
		return "", nil, err
	}
	defer resp.Body.Close()

	body, _ := ioutil.ReadAll(resp.Body)

	return resp.Status, body, err
}
