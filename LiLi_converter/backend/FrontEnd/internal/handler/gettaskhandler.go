package handler

import (
	"net/http"

	"FrontEnd/internal/logic"
	"FrontEnd/internal/svc"
	"github.com/zeromicro/go-zero/rest/httpx"
)

func getTaskHandler(svcCtx *svc.ServiceContext) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		l := logic.NewGetTaskLogic(r.Context(), svcCtx)
		resp, err := l.GetTask()
		if err != nil {
			httpx.ErrorCtx(r.Context(), w, err)
		} else {
			httpx.OkJsonCtx(r.Context(), w, resp)
		}
	}
}
