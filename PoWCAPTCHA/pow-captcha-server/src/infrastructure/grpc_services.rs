use std::sync::Arc;

use connectrpc::{RequestContext, Response, ServiceRequest, ServiceResult};

use crate::application::captcha_service::CaptchaService;
use crate::domain::{ExamSessionRepository, WorkProofTokenRepository};
use crate::error::AppError;
use crate::proto::pow_captcha::v1::*;

pub struct GrpcService<ESR, WPR>
where
    ESR: ExamSessionRepository + Send + Sync + 'static,
    WPR: WorkProofTokenRepository + Send + Sync + 'static,
{
    captcha_service: Arc<CaptchaService<ESR, WPR>>,
}

impl<ESR, WPR> GrpcService<ESR, WPR>
where
    ESR: ExamSessionRepository + Send + Sync + 'static,
    WPR: WorkProofTokenRepository + Send + Sync + 'static,
{
    pub fn new(captcha_service: Arc<CaptchaService<ESR, WPR>>) -> Self {
        Self { captcha_service }
    }
}

impl<ESR, WPR> Clone for GrpcService<ESR, WPR>
where
    ESR: ExamSessionRepository + Send + Sync + 'static,
    WPR: WorkProofTokenRepository + Send + Sync + 'static,
{
    fn clone(&self) -> Self {
        Self {
            captcha_service: self.captcha_service.clone(),
        }
    }
}

#[allow(refining_impl_trait)]
impl<ESR, WPR> PoWcaptchaFrontendService for GrpcService<ESR, WPR>
where
    ESR: ExamSessionRepository + Send + Sync + 'static,
    WPR: WorkProofTokenRepository + Send + Sync + 'static,
{
    async fn get_challenge(
        &self,
        _ctx: RequestContext,
        _req: ServiceRequest<'_, GetChallengeRequest>,
    ) -> ServiceResult<GetChallengeResponse> {
        let response = self
            .captcha_service
            .get_challenge()
            .await
            .map_err(connectrpc::ConnectError::from)?;
        Response::ok(response)
    }

    async fn submit_solution(
        &self,
        _ctx: RequestContext,
        req: ServiceRequest<'_, SubmitSolutionRequest>,
    ) -> ServiceResult<SubmitSolutionResponse> {
        let exam_session = req
            .exam_session
            .as_option()
            .ok_or_else(|| AppError::InvalidArgument("Missing exam_session".into()))?;
        let solution = req
            .solution
            .as_option()
            .ok_or_else(|| AppError::InvalidArgument("Missing solution".into()))?;

        let response = self
            .captcha_service
            .submit_solution(
                exam_session.access_key_id,
                exam_session.access_key_secret,
                solution.nonce,
            )
            .await
            .map_err(connectrpc::ConnectError::from)?;
        Response::ok(response)
    }
}

#[allow(refining_impl_trait)]
impl<ESR, WPR> PoWcaptchaBackendService for GrpcService<ESR, WPR>
where
    ESR: ExamSessionRepository + Send + Sync + 'static,
    WPR: WorkProofTokenRepository + Send + Sync + 'static,
{
    async fn validate_token(
        &self,
        _ctx: RequestContext,
        req: ServiceRequest<'_, ValidateTokenRequest>,
    ) -> ServiceResult<ValidateTokenResponse> {
        let response = self
            .captcha_service
            .validate_token(req.token)
            .await
            .map_err(connectrpc::ConnectError::from)?;
        Response::ok(response)
    }
}
