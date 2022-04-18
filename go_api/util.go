package main

import (
	"fmt"
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

func SendEmail(from string, to []string, message string) {
	smtpHost := "smtp.gmail.com"
	smtpPort := "587"
	auth := smtp.PlainAuth("", from, EMAIL_PASSWORD, smtpHost)

	emailMessage := fmt.Sprintf("To: %s\r\n"+
		"Subject: Verification token\r\n"+
		"\r\n"+
		"Here is your verification code: %s\r\n", to, message)
	emailErr := smtp.SendMail(smtpHost+":"+smtpPort, auth, from, to, []byte(emailMessage))
	if emailErr != nil {
		log.Fatal(emailErr)
	}
}
