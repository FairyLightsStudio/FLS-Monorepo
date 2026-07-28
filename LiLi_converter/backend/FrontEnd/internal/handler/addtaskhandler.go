package handler

import (
	"net/http"

	"FrontEnd/internal/logic"
	"FrontEnd/internal/svc"
	"github.com/zeromicro/go-zero/rest/httpx"
)

func addTaskHandler(svcCtx *svc.ServiceContext) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		l := logic.NewAddTaskLogic(r.Context(), svcCtx)
		resp, err := l.AddTask()
		if err != nil {
			httpx.ErrorCtx(r.Context(), w, err)
		} else {
			httpx.OkJsonCtx(r.Context(), w, resp)
		}
	}
}
