use azure_identity::DefaultAzureCredential;
use azure_mgmt_resources::Client as ResourceManagementClient;


//pub fn connect_to_az(azConfiguration: &AzCredentials) -> Result<ResourceManagementClient, Error> {
    //let credential = DefaultAzureCredential::default();

    //let client = ResourceManagementClient::builder(credentia)
    //    .subscription_id(azConfiguration.subscription_id)
    //    .build()?;

    //Ok(client)
//}


pub struct AzCredentials {
    client_id: String,
    secret: String,
    tenant: String,
    subscription_id: String,
}

impl AzCredentials {
    pub fn new(client_id: String, secret: String, tenant: String, subscription_id: String) -> AzCredentials {
        AzCredentials {
            client_id,
            secret,
            tenant,
            subscription_id,
        }
    }
}
