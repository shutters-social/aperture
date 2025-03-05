use aws_config::SdkConfig;

use crate::aws::AwsWrapper;

#[derive(Clone)]
pub struct AppState {
    aws_config: SdkConfig,
}

impl AppState {
    pub fn new(aws_config: SdkConfig) -> AppState {
        AppState { aws_config }
    }

    pub fn aws(self) -> AwsWrapper {
        AwsWrapper::new(self.aws_config())
    }

    pub fn aws_config(self) -> SdkConfig {
        self.aws_config.clone()
    }
}
