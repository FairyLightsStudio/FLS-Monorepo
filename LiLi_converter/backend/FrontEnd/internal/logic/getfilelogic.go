package logic

import (
	"context"

	"FrontEnd/internal/svc"
	"FrontEnd/internal/types"

	"github.com/zeromicro/go-zero/core/logx"
)

type GetFileLogic struct {
	logx.Logger
	ctx    context.Context
	svcCtx *svc.ServiceContext
}

func NewGetFileLogic(ctx context.Context, svcCtx *svc.ServiceContext) *GetFileLogic {
	return &GetFileLogic{
		Logger: logx.WithContext(ctx),
		ctx:    ctx,
		svcCtx: svcCtx,
	}
}

func (l *GetFileLogic) GetFile() (resp *types.File, err error) {
	// todo: add your logic here and delete this line

	return
}
