package main

import (
	"log"
	"math/rand"
	"net/smtp"
	"time"
)

func GenerateToken() string {
	rand.Seed(time.Now().UnixNano())
	const TOKEN_CHAR_SET = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
	token := [4]byte{}
	for i := 0; i < 4; i++ {
		token[i] = TOKEN_CHAR_SET[rand.Intn(len(TOKEN_CHAR_SET))]
	}

	return string(token[:])
}

func SendEmail(from string, password string, to []string, message []byte) {
	smtpHost := "smtp.gmail.com"
	smtpPort := "587"

	auth := smtp.PlainAuth("", from, password, smtpHost)
	emailErr := smtp.SendMail(smtpHost+":"+smtpPort, auth, from, to, message)
	if emailErr != nil {
		log.Fatal(emailErr)
	}
}
