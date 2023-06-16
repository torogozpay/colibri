use tonic_openssl_lnd::lnrpc::{AddInvoiceResponse, Invoice, PaymentHash};
use tonic_openssl_lnd::{LndClientError, LndLightningClient};

use infrastructure::connect;

pub async fn get_invoice(hash: &[u8]) -> Result<Invoice, LndClientError> {
    let mut client = connect().await.unwrap();
    let invoice = client
        .lookup_invoice(PaymentHash {
            r_hash: hash.to_vec(),
            ..Default::default()
        })
        .await?
        .into_inner();

    Ok(invoice)
}