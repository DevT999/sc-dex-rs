elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use common_structs::FarmTokenAttributes;
use farm_token::FarmToken;

use super::base::*;
use config::State;

pub struct ClaimRewardsContext<M: ManagedTypeApi> {
    caller: ManagedAddress<M>,
    tx_input: ClaimRewardsTxInput<M>,
    block_nonce: u64,
    block_epoch: u64,
    position_reward: BigUint<M>,
    storage_cache: StorageCache<M>,
    initial_farming_amount: BigUint<M>,
    final_reward: Option<EsdtTokenPayment<M>>,
    output_attributes: Option<FarmTokenAttributes<M>>,
    output_created_with_merge: bool,
    output_payments: ManagedVec<M, EsdtTokenPayment<M>>,
}

pub struct ClaimRewardsTxInput<M: ManagedTypeApi> {
    args: ClaimRewardsArgs<M>,
    payments: ClaimRewardsPayments<M>,
    attributes: Option<FarmTokenAttributes<M>>,
}

pub struct ClaimRewardsArgs<M: ManagedTypeApi> {
    opt_accept_funds_func: OptionalArg<ManagedBuffer<M>>,
}

pub struct ClaimRewardsPayments<M: ManagedTypeApi> {
    first_payment: EsdtTokenPayment<M>,
    additional_payments: ManagedVec<M, EsdtTokenPayment<M>>,
}

impl<M: ManagedTypeApi> ClaimRewardsTxInput<M> {
    pub fn new(args: ClaimRewardsArgs<M>, payments: ClaimRewardsPayments<M>) -> Self {
        ClaimRewardsTxInput {
            args,
            payments,
            attributes: None,
        }
    }
}

impl<M: ManagedTypeApi> ClaimRewardsArgs<M> {
    pub fn new(opt_accept_funds_func: OptionalArg<ManagedBuffer<M>>) -> Self {
        ClaimRewardsArgs {
            opt_accept_funds_func,
        }
    }
}

impl<M: ManagedTypeApi> ClaimRewardsPayments<M> {
    pub fn new(
        first_payment: EsdtTokenPayment<M>,
        additional_payments: ManagedVec<M, EsdtTokenPayment<M>>,
    ) -> Self {
        ClaimRewardsPayments {
            first_payment,
            additional_payments,
        }
    }
}

impl<M: ManagedTypeApi> ClaimRewardsContext<M> {
    pub fn new(tx_input: ClaimRewardsTxInput<M>, caller: ManagedAddress<M>) -> Self {
        ClaimRewardsContext {
            caller,
            tx_input,
            block_nonce: 0,
            block_epoch: 0,
            position_reward: BigUint::zero(),
            storage_cache: StorageCache::default(),
            initial_farming_amount: BigUint::zero(),
            final_reward: None,
            output_attributes: None,
            output_created_with_merge: true,
            output_payments: ManagedVec::new(),
        }
    }
}

impl<M: ManagedTypeApi> Context<M> for ClaimRewardsContext<M> {
    #[inline]
    fn set_contract_state(&mut self, contract_state: State) {
        self.storage_cache.contract_state = contract_state;
    }

    #[inline]
    fn get_contract_state(&self) -> &State {
        &self.storage_cache.contract_state
    }

    #[inline]
    fn get_caller(&self) -> &ManagedAddress<M> {
        &self.caller
    }

    #[inline]
    fn set_output_payments(&mut self, payments: ManagedVec<M, EsdtTokenPayment<M>>) {
        self.output_payments = payments
    }

    #[inline]
    fn get_output_payments(&self) -> &ManagedVec<M, EsdtTokenPayment<M>> {
        &self.output_payments
    }

    #[inline]
    fn get_opt_accept_funds_func(&self) -> &OptionalArg<ManagedBuffer<M>> {
        &self.tx_input.args.opt_accept_funds_func
    }

    #[inline]
    fn get_tx_input(&self) -> &dyn TxInput<M> {
        &self.tx_input
    }

    #[inline]
    fn set_farm_token_id(&mut self, farm_token_id: TokenIdentifier<M>) {
        self.storage_cache.farm_token_id = farm_token_id
    }

    #[inline]
    fn get_farm_token_id(&self) -> &TokenIdentifier<M> {
        &self.storage_cache.farm_token_id
    }

    #[inline]
    fn set_farming_token_id(&mut self, farming_token_id: TokenIdentifier<M>) {
        self.storage_cache.farming_token_id = farming_token_id
    }

    #[inline]
    fn get_farming_token_id(&self) -> &TokenIdentifier<M> {
        &self.storage_cache.farming_token_id
    }

    #[inline]
    fn set_reward_token_id(&mut self, reward_token_id: TokenIdentifier<M>) {
        self.storage_cache.reward_token_id = reward_token_id;
    }

    #[inline]
    fn get_reward_token_id(&self) -> &TokenIdentifier<M> {
        &self.storage_cache.reward_token_id
    }

    #[inline]
    fn set_block_nonce(&mut self, nonce: u64) {
        self.block_nonce = nonce;
    }

    #[inline]
    fn get_block_nonce(&self) -> u64 {
        self.block_nonce
    }

    #[inline]
    fn set_block_epoch(&mut self, epoch: u64) {
        self.block_epoch = epoch;
    }

    #[inline]
    fn get_block_epoch(&self) -> u64 {
        self.block_epoch
    }

    #[inline]
    fn set_reward_per_share(&mut self, rps: BigUint<M>) {
        self.storage_cache.reward_per_share = rps;
    }

    #[inline]
    fn get_reward_per_share(&self) -> &BigUint<M> {
        &self.storage_cache.reward_per_share
    }

    #[inline]
    fn set_farm_token_supply(&mut self, supply: BigUint<M>) {
        self.storage_cache.farm_token_supply = supply;
    }

    #[inline]
    fn get_farm_token_supply(&self) -> &BigUint<M> {
        &self.storage_cache.farm_token_supply
    }

    #[inline]
    fn set_division_safety_constant(&mut self, dsc: BigUint<M>) {
        self.storage_cache.division_safety_constant = dsc;
    }

    #[inline]
    fn get_division_safety_constant(&self) -> &BigUint<M> {
        &self.storage_cache.division_safety_constant
    }

    #[inline]
    fn set_reward_reserve(&mut self, rr: BigUint<M>) {
        self.storage_cache.reward_reserve = rr;
    }

    #[inline]
    fn get_reward_reserve(&self) -> &BigUint<M> {
        &self.storage_cache.reward_reserve
    }

    #[inline]
    fn increase_reward_reserve(&mut self, amount: &BigUint<M>) {
        self.storage_cache.reward_reserve += amount;
    }

    #[inline]
    fn decrease_reward_reserve(&mut self) {
        self.storage_cache.reward_reserve -= &self.position_reward;
    }

    #[inline]
    fn update_reward_per_share(&mut self, reward_added: &BigUint<M>) {
        if self.storage_cache.farm_token_supply != 0u64 {
            self.storage_cache.reward_per_share += reward_added
                * &self.storage_cache.division_safety_constant
                / &self.storage_cache.farm_token_supply;
        }
    }

    #[inline]
    fn get_storage_cache(&self) -> &StorageCache<M> {
        &self.storage_cache
    }

    #[inline]
    fn set_input_attributes(&mut self, _attr: FarmTokenAttributes<M>) {}

    #[inline]
    fn get_input_attributes(&self) -> Option<&FarmTokenAttributes<M>> {
        self.tx_input.attributes.as_ref()
    }

    #[inline]
    fn set_position_reward(&mut self, amount: BigUint<M>) {
        self.position_reward = amount;
    }

    #[inline]
    fn get_position_reward(&self) -> Option<&BigUint<M>> {
        Some(&self.position_reward)
    }

    #[inline]
    fn set_initial_farming_amount(&mut self, amount: BigUint<M>) {
        self.initial_farming_amount = amount;
    }

    #[inline]
    fn get_initial_farming_amount(&self) -> Option<&BigUint<M>> {
        Some(&self.initial_farming_amount)
    }

    #[inline]
    fn set_final_reward(&mut self, payment: EsdtTokenPayment<M>) {
        self.final_reward = Some(payment);
    }

    #[inline]
    fn get_final_reward(&self) -> Option<&EsdtTokenPayment<M>> {
        self.final_reward.as_ref()
    }

    #[inline]
    fn was_output_created_with_merge(&self) -> bool {
        self.output_created_with_merge
    }

    #[inline]
    fn get_output_attributes(&self) -> Option<&FarmTokenAttributes<M>> {
        self.output_attributes.as_ref()
    }

    #[inline]
    fn set_output_position(&mut self, position: FarmToken<M>, created_with_merge: bool) {
        self.output_payments.push(position.token_amount);
        self.output_created_with_merge = created_with_merge;
        self.output_attributes = Some(position.attributes);
    }
}

impl<M: ManagedTypeApi> TxInputArgs<M> for ClaimRewardsArgs<M> {
    #[inline]
    fn are_valid(&self) -> bool {
        true
    }
}

impl<M: ManagedTypeApi> TxInputPayments<M> for ClaimRewardsPayments<M> {
    #[inline]
    fn are_valid(&self) -> bool {
        true
    }

    #[inline]
    fn get_first(&self) -> &EsdtTokenPayment<M> {
        &self.first_payment
    }

    #[inline]
    fn get_additional(&self) -> Option<&ManagedVec<M, EsdtTokenPayment<M>>> {
        Some(&self.additional_payments)
    }
}

impl<M: ManagedTypeApi> ClaimRewardsPayments<M> {}

impl<M: ManagedTypeApi> TxInput<M> for ClaimRewardsTxInput<M> {
    #[inline]
    fn get_args(&self) -> &dyn TxInputArgs<M> {
        &self.args
    }

    #[inline]
    fn get_payments(&self) -> &dyn TxInputPayments<M> {
        &self.payments
    }

    #[inline]
    fn is_valid(&self) -> bool {
        true
    }
}

impl<M: ManagedTypeApi> ClaimRewardsTxInput<M> {}

impl<M: ManagedTypeApi> ClaimRewardsContext<M> {
    pub fn is_accepted_payment(&self) -> bool {
        let first_payment_pass = self.tx_input.payments.first_payment.token_identifier
            == self.storage_cache.farm_token_id
            && self.tx_input.payments.first_payment.token_nonce != 0
            && self.tx_input.payments.first_payment.amount != 0u64;

        if !first_payment_pass {
            return false;
        }

        for payment in self.tx_input.payments.additional_payments.iter() {
            let payment_pass = payment.token_identifier == self.storage_cache.farm_token_id
                && payment.token_nonce != 0
                && payment.amount != 0;

            if !payment_pass {
                return false;
            }
        }

        true
    }
}