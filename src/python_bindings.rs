use pyo3::prelude::*;
use std::fs;
use crate::{WalletBuilder, Wallet, TransactionSigner};
use crate::key_derivation::KeyMaterial;


#[pyfunction]
fn generate_wallet() -> PyResult<String> {
    let image_path = "img.png";
    let image_data = fs::read(image_path).expect("Failed to read the image file");
    let emoji:&str = "👻";
    let color = "#ff5733";
    // let key_material = KeyMaterial::generate(&image_raw_data, &emoji, &color)?;
    let key_material = KeyMaterial::generate(&image_data, &emoji, &color)
    .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    
    let blockchain: &'static str = "solana";
    // let wallet = WalletBuilder::generate_wallet(
    //     &key_material.master_seed, 
    //     &key_material.cipher_key.unwrap(), 
    //     blockchain)?;

    let wallet = WalletBuilder::generate_wallet(
        &key_material.master_seed, 
        &key_material.cipher_key.unwrap(), 
        blockchain)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    
    Ok(wallet.address)
}

#[pymodule]
fn mantishash(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_wallet, m)?)?;
    Ok(())
}
    

    // #[pyfn(m, "sign_transaction")]
    // fn sign_transaction_py(_py: Python, data: String, priv_key: String) -> PyResult<String> {
    //     // bd32fe29c04aeece0da3e9beed527b2c5fa580259a7741bf1759f0dcf827c800
    //     let signer = TransactionSigner::new(priv_key);
    //     let signature = signer.sign(data.as_bytes());
    //     Ok(hex::encode(signature))
    // }
