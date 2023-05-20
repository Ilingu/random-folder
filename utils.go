package main

import (
	"encoding/json"
	"strings"
)

type Response[T any] struct {
	Succeed bool   `json:"succeed"`
	Data    T      `json:"data,omitempty"`
	Reason  string `json:"reason,omitempty"`
}

func (r Response[T]) marshal() string {
	rawJson, err := json.Marshal(r)
	if err != nil {
		return "{succeed:false}"
	}
	return string(rawJson)
}

func isImage(filename string) bool {
	return strings.Contains(filename, ".png") || strings.Contains(filename, ".jpg") || strings.Contains(filename, ".jpeg")
}
