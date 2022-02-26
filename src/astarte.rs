use astarte_sdk::builder::AstarteOptions;
use astarte_sdk::{AstarteError, AstarteSdk};
use futures::executor::block_on;

use crate::config::AstarteDeviceParams;
use crate::store::{store_get_string, store_set_string};
use crate::{Publisher, Topic};

static DEVICE_CREDENTIAL_KEY: &str = "dev_cred_k";

#[derive(Clone)]
pub struct Astarte {
    pub device_sdk: AstarteSdk,
}

pub async fn init_astarte(
    astarte_device_params: AstarteDeviceParams,
) -> Result<AstarteSdk, AstarteError> {
    let mut credentials_secret = store_get_string(DEVICE_CREDENTIAL_KEY);
    if credentials_secret.is_none() {
        credentials_secret = Some(get_credentials(&astarte_device_params).await?);
    }

    let sdk_options = AstarteOptions::new(
        &astarte_device_params.realm,
        &astarte_device_params.device_id,
        &credentials_secret.unwrap(),
        &astarte_device_params.pairing_url,
    )
    .ignore_ssl_errors()
    .interface_directory("./interfaces")?
    .build();

    let device = astarte_sdk::AstarteSdk::new(&sdk_options).await?;
    Ok(device)
}

pub async fn get_credentials(
    astarte_device_params: &AstarteDeviceParams,
) -> Result<String, AstarteError> {
    let credentials_secret = astarte_sdk::registration::register_device(
        &astarte_device_params.token,
        &astarte_device_params.pairing_url,
        &astarte_device_params.realm,
        &astarte_device_params.device_id,
    )
    .await
    .unwrap();
    store_set_string(DEVICE_CREDENTIAL_KEY, credentials_secret.as_str());
    Ok(credentials_secret)
}

impl Publisher<String> for Astarte {
    fn publish(&self, topic: &Topic, value: String) {
        match block_on(self.device_sdk.send(&topic.name, &topic.path, &value)) {
            Ok(_) => {
                println!("Send to Astarte {}", value);
            }
            Err(err) => {
                println!("Unable to send value to Astarte {:?}", err)
            }
        }
    }
}
