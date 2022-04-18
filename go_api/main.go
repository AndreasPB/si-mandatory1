package main

import (
	"context"
	"log"
	"os"

	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

var MongodbClient, err = mongo.NewClient(options.Client().ApplyURI("mongodb://root:example@mongo:27017"))
var Ctx = context.Background()

var JWT_SECRET = os.Getenv("JWT_SECRET")
var EMAIL_PASSWORD = os.Getenv("EMAIL_PASSWORD")

func main() {
	err = MongodbClient.Connect(Ctx)
	if err != nil {
		log.Fatal(err)
	}
	defer MongodbClient.Disconnect(Ctx)

	router := gin.Default()
	router.Use(cors.Default())
	router.POST("/login", PostLogin)
	router.POST("/user", PostUser)
	router.GET("/", Root)
	router.GET("/user", GetUsers)
	router.GET("/user/:name", GetUserByName)
	router.Run(":9901")
}
