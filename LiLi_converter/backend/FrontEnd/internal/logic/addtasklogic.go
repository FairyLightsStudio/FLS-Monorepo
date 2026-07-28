package logic

import (
	"context"

	"FrontEnd/internal/svc"
	"FrontEnd/internal/types"

	"github.com/zeromicro/go-zero/core/logx"
)

type AddTaskLogic struct {
	logx.Logger
	ctx    context.Context
	svcCtx *svc.ServiceContext
}

func NewAddTaskLogic(ctx context.Context, svcCtx *svc.ServiceContext) *AddTaskLogic {
	return &AddTaskLogic{
		Logger: logx.WithContext(ctx),
		ctx:    ctx,
		svcCtx: svcCtx,
	}
}

func (l *AddTaskLogic) AddTask() (resp *types.AddTaskResp, err error) {
	// todo: add your logic here and delete this line

	return
}
