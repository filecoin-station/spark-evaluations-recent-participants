pub mod recent_participants {

    use contract_bindings::recent_participants::RecentParticipants;
    use ethers::{
        providers::{Http, Middleware, Provider},
        types::H160,
    };
    use fevm_utils::{send_tx, set_tx_gas};
    use std::sync::Arc;

    #[derive(PartialEq)]
    pub enum DeployMethod {
        Mnemonic,
        Ledger,
    }

    pub enum Test<S: Middleware + 'static> {
        Test(S),
    }

    pub async fn deploy_contract<S: Middleware + 'static>(
        client: Arc<S>,
        retries: usize,
        provider: Provider<Http>,
        address: H160,
    ) -> Result<H160, Box<dyn std::error::Error>> {
        let gas_price = provider.get_gas_price().await?;
        println!("current gas price: {:#?}", gas_price);
        println!("using {} retries", retries);

        let mut contract = RecentParticipants::deploy(client.clone(), address)?;
        let tx = contract.deployer.tx.clone();
        set_tx_gas(
            &mut contract.deployer.tx,
            client.estimate_gas(&tx, None).await?,
            gas_price * 2,
        );

        println!(
            "estimated deployment gas cost: {:#?}",
            contract.deployer.tx.gas().unwrap()
        );

        let receipt = send_tx(&contract.deployer.tx, client, retries).await?;

        let address = receipt
            .contract_address
            .ok_or(format!("Error deploying Contract"))?;

        println!("contract deployment address: {:#?}", address);
        Ok(address)
    }
}
