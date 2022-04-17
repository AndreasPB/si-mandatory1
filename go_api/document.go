package main

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
