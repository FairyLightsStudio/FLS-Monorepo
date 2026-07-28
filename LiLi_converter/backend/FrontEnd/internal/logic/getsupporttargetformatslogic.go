package logic

import (
	"context"

	"FrontEnd/internal/svc"
	"FrontEnd/internal/types"

	"github.com/zeromicro/go-zero/core/logx"
)

type GetSupportTargetFormatsLogic struct {
	logx.Logger
	ctx    context.Context
	svcCtx *svc.ServiceContext
}

func NewGetSupportTargetFormatsLogic(ctx context.Context, svcCtx *svc.ServiceContext) *GetSupportTargetFormatsLogic {
	return &GetSupportTargetFormatsLogic{
		Logger: logx.WithContext(ctx),
		ctx:    ctx,
		svcCtx: svcCtx,
	}
}

func (l *GetSupportTargetFormatsLogic) GetSupportTargetFormats() (resp []types.Conversion, err error) {
	// todo: add your logic here and delete this line

	return
}
