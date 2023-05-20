use scrypto::prelude::*;

#[blueprint]
mod treasury {
    struct Treasury {
        // Define what resources and data will be managed by Treasury components
        vault_account: HashMap<String, Vault>,
        deposit_whitelist: HashMap<String, Vec<ComponentAddress>>,
    }

    impl Treasury {
        // Implement the functions and methods which will manage those resources and data

        // This is a function, and can be called directly on the blueprint once deployed
        pub fn instantiate_treasury() -> ComponentAddress {
            Self {
                vault_account: HashMap::new(),
                deposit_whitelist: HashMap::new(),
            }
            .instantiate()
            .globalize()
        }

        // Query token balance
        pub fn balances(&self, token_name: String) {
            info!(
                "My token balance is {:?}",
                self.vault_account.get(&token_name as &str)
            );
        }

        // Create a vault for a new token
        pub fn add_token(&mut self, token_name: String, token_resource_id: ResourceAddress) {
            self.vault_account
                .insert(token_name, Vault::new(token_resource_id));
        }

        // Add an entity and its component address to the deposit whitelist
        pub fn add_to_deposit_whitelist(&mut self, entity_name: String, address: ComponentAddress) {
            self.deposit_whitelist
                .entry(entity_name)
                .or_default()
                .push(address);
        }

        // Remove an entity and its component address from the deposit whitelist
        pub fn remove_from_whitelist(&mut self, entity_name: String, address: ComponentAddress) {
            self.deposit_whitelist
                .entry(entity_name)
                .or_default()
                .retain(|x| x != &address);
        }

        // Method that prints the ComponentAddress of the Treasury Component
        pub fn request_component_address(&self) {
            let address = Runtime::actor();
            info!("Treasury Component Address: {:?}", address);
        }
    }
}