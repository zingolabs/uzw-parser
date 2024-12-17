use zwl_parser::{Wallet, zwl::ZecWalletLite};

fn main() {
    // let wallet = Wallet::parse::<ZecWalletLite>("zecwallet-light-wallet.dat").map_err(|_|"Cannot open file").unwrap();
    match Wallet::parse::<ZecWalletLite>("zecwallet-light-wallet.dat") {
        Ok(w) => println!("{:?}", w),
        Err(e) => println!("{}", e.to_string())
    }
    // println!("{:?}", wallet);
}