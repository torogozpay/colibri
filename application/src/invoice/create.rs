use tonic_openssl_lnd::lnrpc::{AddInvoiceResponse, Invoice, PaymentHash};
use tonic_openssl_lnd::{LndClientError, LndLightningClient};

use infrastructure::connect;

pub async fn create_invoice(
    description: &str,
    amount: u32,
) -> Result<AddInvoiceResponse, LndClientError> {
    let mut client = connect().await.unwrap();
    let invoice = Invoice {
        memo: description.to_string(),
        value: amount as i64,
        ..Default::default()
    };
    let invoice = client.add_invoice(invoice).await?.into_inner();

    Ok(invoice)
}