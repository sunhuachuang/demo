use serde_json::Value;
use web3::{api::Eth, contract::Contract, transports::Http};

const ADDRESS: &str = include_str!("../public/mainnet.json");

pub fn lock(eth: Eth<Http>) -> Result<Contract<Http>, ()> {
    let address: Value = serde_json::from_str(ADDRESS).map_err(|_| ())?;
    let abi = serde_json::to_vec(
        &serde_json::from_str::<Value>(include_str!("../artifacts/contracts/Lock.sol/Lock.json"))
            .map_err(|_| ())?["abi"],
    )
    .map_err(|_| ())?;
    Contract::from_json(
        eth,
        address["Lock"]["address"]
            .as_str()
            .ok_or(())?
            .parse()
            .map_err(|_| ())?,
        &abi,
    )
    .map_err(|_| ())
}
