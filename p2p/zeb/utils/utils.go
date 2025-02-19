package utils

import (
	"encoding/json"
	"io"
	"net/http"
	"strings"
	"time"
)

func GetExtIp(remote string) (string, error) {
	resp, err := Call(remote, "getExtIp", 0, map[string]interface{}{})
	if err != nil {
		return "", err
	}

	var ret map[string]interface{}
	if err := json.Unmarshal(resp, &ret); err != nil {
		return "", err
	}

	return ret["extIp"].(string), nil
}

func Call(address string, method string, id uint, params map[string]interface{}) ([]byte, error) {
	params["method"] = method
	data, err := json.Marshal(params)
	if err != nil {
		return nil, err
	}

	client := &http.Client{
		Timeout: 5 * time.Second,
	}
	resp, err := client.Post(address, "application/json", strings.NewReader(string(data)))
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, err
	}
	return body, nil
}

func GetWsAddr(remote, addr string) (string, error) {
	resp, err := Call(remote, "getWsAddr", 0, map[string]interface{}{
		"address": addr,
	})
	if err != nil {
		return "", err
	}

	var ret map[string]interface{}
	if err := json.Unmarshal(resp, &ret); err != nil {
		return "", err
	}

	return ret["wsAddr"].(string), nil
}
