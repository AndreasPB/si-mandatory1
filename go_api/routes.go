package main

import (
	"log"
	"net/http"
	"time"

	"github.com/gin-gonic/gin"
	"github.com/golang-jwt/jwt"
	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
)

func Root(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{"message": "Hello World"})
}

func PostLogin(c *gin.Context) {
	userLogin := UserLogin{}
	c.Bind(&userLogin)

	userCollection := MongodbClient.Database("db").Collection("User")

	user := User{}
	mongoErr := userCollection.FindOne(Ctx, bson.M{"phone": userLogin.Phone}).Decode(&user)
	if mongoErr != nil {
		switch mongoErr {
		case mongo.ErrNoDocuments:
			c.JSON(http.StatusNotFound, gin.H{"info": "User not found"})
			return
		default:
			c.JSON(http.StatusInternalServerError, gin.H{"info": err.Error()})
			return
		}
	} else {
		if userLogin.Password == user.Password {
			jwt := jwt.NewWithClaims(
				jwt.SigningMethodHS256,
				jwt.MapClaims{
					"exp":  time.Now().Add(10 * time.Minute).Unix(),
					"iat":  time.Now().Unix(),
					"user": user,
				})

			jwtToken, err := jwt.SignedString([]byte(JWT_SECRET))
			if err != nil {
				log.Fatal(err)
			}
			c.JSON(http.StatusOK, jwtToken)
		} else {
			c.JSON(http.StatusForbidden, gin.H{"info": "Login failed"})
		}
	}
}

func PostUser(c *gin.Context) {
	userCollection := MongodbClient.Database("db").Collection("User")

	password := GenerateToken()

	user := User{}
	c.Bind(&user)
	user.Password = password

	userToFind := User{}
	userCollection.FindOne(Ctx, bson.M{"phone": user.Phone}).Decode(&userToFind)
	if userToFind.Phone == user.Phone {
		c.JSON(http.StatusConflict, gin.H{"info": "User already exists"})
		return
	}

	SendEmail("<FROM>", []string{"<TO>"}, password)
	userCollection.InsertOne(Ctx, user)
	c.JSON(http.StatusCreated, user)

}

func GetUsers(c *gin.Context) {
	userCollection := MongodbClient.Database("db").Collection("User")

	foundUsers, _ := userCollection.Find(Ctx, bson.D{})

	users := []User{}
	if err = foundUsers.All(Ctx, &users); err != nil {
		c.JSON(http.StatusConflict, gin.H{"info": err.Error()})
		return
	}

	c.JSON(http.StatusOK, users)
}

func GetUserByName(c *gin.Context) {
	userCollection := MongodbClient.Database("db").Collection("User")

	user := User{}
	err := userCollection.FindOne(Ctx, bson.M{"name": c.Param("name")}).Decode(&user)
	if err != nil {
		switch err {
		case mongo.ErrNoDocuments:
			c.JSON(http.StatusNotFound, gin.H{"info": "User not found"})
			return
		}
	}
	c.JSON(http.StatusOK, user)
}
