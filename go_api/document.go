package main

import "fmt"

type UserRegister struct {
	Name  string `form:"name"`
	Phone string `form:"phone"`
}

type UserLogin struct {
	Phone    string `form:"phone"`
	Password string `form:"password"`
}

type User struct {
	Phone    string `form:"phone" bson:"phone" json:"phone"`
	Name     string `form:"name" bson:"name" json:"name"`
	Password string `form:"password" bson:"password" json:"password"`
}

type Mail struct {
	From    string
	To      []string
	Subject string
	Body    string
}

func (m Mail) Bytes() []byte {
	mail := fmt.Sprintf("To: %s\r\n"+
		"Subject: %s\r\n"+
		"\r\n"+
		"%s\r\n", m.To, m.Subject, m.Body)
	return []byte(mail)
}
