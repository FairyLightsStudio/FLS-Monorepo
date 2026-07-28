package svc

import (
	"FrontEnd/internal/config"
	"FrontEnd/internal/middleware"
	"github.com/zeromicro/go-zero/rest"
)

type ServiceContext struct {
	Config          config.Config
	AuthByTaskToken rest.Middleware
}

func NewServiceContext(c config.Config) *ServiceContext {
	return &ServiceContext{
		Config:          c,
		AuthByTaskToken: middleware.NewAuthByTaskTokenMiddleware().Handle,
	}
}
