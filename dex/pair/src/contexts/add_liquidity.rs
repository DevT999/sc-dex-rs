elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use super::base::*;
use crate::State;

pub struct AddLiquidityContext<M: ManagedTypeApi> {
    caller: ManagedAddress<M>,
    tx_input: AddLiquidityTxInput<M>,
    contract_state: State,
    lp_token_id: TokenIdentifier<M>,
    first_token_id: TokenIdentifier<M>,
    second_token_id: TokenIdentifier<M>,
    first_token_reserve: BigUint<M>,
    second_token_reserve: BigUint<M>,
    lp_token_supply: BigUint<M>,
    initial_k: BigUint<M>,
    first_token_optimal: BigUint<M>,
    second_token_optimal: BigUint<M>,
    liquidity_added: BigUint<M>,
    output_payments: ManagedVec<M, EsdtTokenPayment<M>>,
}

pub struct AddLiquidityTxInput<M: ManagedTypeApi> {
    args: AddLiquidityArgs<M>,
    payments: AddLiquidityPayments<M>,
}

pub struct AddLiquidityArgs<M: ManagedTypeApi> {
    first_token_amount_min: BigUint<M>,
    second_token_amount_min: BigUint<M>,
    opt_accept_funds_func: OptionalArg<ManagedBuffer<M>>,
}

pub struct AddLiquidityPayments<M: ManagedTypeApi> {
    first_payment: Option<EsdtTokenPayment<M>>,
    second_payment: Option<EsdtTokenPayment<M>>,
}

impl<M: ManagedTypeApi> AddLiquidityTxInput<M> {
    pub fn new(args: AddLiquidityArgs<M>, payments: AddLiquidityPayments<M>) -> Self {
        AddLiquidityTxInput { args, payments }
    }
}

impl<M: ManagedTypeApi> AddLiquidityArgs<M> {
    pub fn new(
        first_token_amount_min: BigUint<M>,
        second_token_amount_min: BigUint<M>,
        opt_accept_funds_func: OptionalArg<ManagedBuffer<M>>,
    ) -> Self {
        AddLiquidityArgs {
            first_token_amount_min,
            second_token_amount_min,
            opt_accept_funds_func,
        }
    }
}

impl<M: ManagedTypeApi> AddLiquidityPayments<M> {
    pub fn new(
        first_payment: Option<EsdtTokenPayment<M>>,
        second_payment: Option<EsdtTokenPayment<M>>,
    ) -> Self {
        AddLiquidityPayments {
            first_payment,
            second_payment,
        }
    }
}

impl<M: ManagedTypeApi> AddLiquidityContext<M> {
    pub fn new(tx_input: AddLiquidityTxInput<M>, caller: ManagedAddress<M>) -> Self {
        AddLiquidityContext {
            caller,
            tx_input,
            contract_state: State::Inactive,
            lp_token_id: TokenIdentifier::egld(),
            first_token_id: TokenIdentifier::egld(),
            second_token_id: TokenIdentifier::egld(),
            first_token_reserve: BigUint::zero(),
            second_token_reserve: BigUint::zero(),
            lp_token_supply: BigUint::zero(),
            initial_k: BigUint::zero(),
            first_token_optimal: BigUint::zero(),
            second_token_optimal: BigUint::zero(),
            liquidity_added: BigUint::zero(),
            output_payments: ManagedVec::new(),
        }
    }
}

impl<M: ManagedTypeApi> Context<M> for AddLiquidityContext<M> {
    fn set_contract_state(&mut self, contract_state: State) {
        self.contract_state = contract_state;
    }

    fn get_contract_state(&self) -> &State {
        &self.contract_state
    }

    fn set_lp_token_id(&mut self, lp_token_id: TokenIdentifier<M>) {
        self.lp_token_id = lp_token_id;
    }

    fn get_lp_token_id(&self) -> &TokenIdentifier<M> {
        &self.lp_token_id
    }

    fn set_first_token_id(&mut self, token_id: TokenIdentifier<M>) {
        self.first_token_id = token_id;
    }

    fn get_first_token_id(&self) -> &TokenIdentifier<M> {
        &self.first_token_id
    }

    fn set_second_token_id(&mut self, token_id: TokenIdentifier<M>) {
        self.second_token_id = token_id;
    }

    fn get_second_token_id(&self) -> &TokenIdentifier<M> {
        &self.second_token_id
    }

    fn set_first_token_reserve(&mut self, amount: BigUint<M>) {
        self.first_token_reserve = amount;
    }

    fn get_first_token_reserve(&self) -> &BigUint<M> {
        &self.first_token_reserve
    }

    fn set_second_token_reserve(&mut self, amount: BigUint<M>) {
        self.second_token_reserve = amount;
    }

    fn get_second_token_reserve(&self) -> &BigUint<M> {
        &self.second_token_reserve
    }

    fn set_lp_token_supply(&mut self, amount: BigUint<M>) {
        self.lp_token_supply = amount;
    }

    fn get_lp_token_supply(&self) -> &BigUint<M> {
        &self.lp_token_supply
    }

    fn set_initial_k(&mut self, k: BigUint<M>) {
        self.initial_k = k;
    }

    fn get_initial_k(&self) -> &BigUint<M> {
        &self.initial_k
    }

    fn get_caller(&self) -> &ManagedAddress<M> {
        &self.caller
    }

    fn get_tx_input(&self) -> &dyn TxInput<M> {
        &self.tx_input
    }
}

impl<M: ManagedTypeApi> AddLiquidityContext<M> {}

impl<M: ManagedTypeApi> TxInputArgs<M> for AddLiquidityArgs<M> {
    fn are_valid(&self) -> bool {
        self.first_token_amount_min != 0 && self.second_token_amount_min != 0
    }
}

impl<M: ManagedTypeApi> TxInputPayments<M> for AddLiquidityPayments<M> {
    fn are_valid(&self) -> bool {
        self.is_valid_payment(&self.first_payment.as_ref())
            && self.is_valid_payment(&self.second_payment.as_ref())
    }
}

impl<M: ManagedTypeApi> AddLiquidityPayments<M> {
    fn is_valid_payment(&self, payment_opt: &Option<&EsdtTokenPayment<M>>) -> bool {
        match payment_opt {
            Some(payment) => payment.amount != 0 && payment.token_nonce == 0,
            None => false,
        }
    }
}

impl<M: ManagedTypeApi> TxInput<M> for AddLiquidityTxInput<M> {
    fn get_args(&self) -> &dyn TxInputArgs<M> {
        &self.args
    }

    fn get_payments(&self) -> &dyn TxInputPayments<M> {
        &self.payments
    }

    fn is_valid(&self) -> bool {
        self.args_match_payments()
    }
}

impl<M: ManagedTypeApi> AddLiquidityTxInput<M> {
    fn args_match_payments(&self) -> bool {
        self.min_leq_payment_amount(
            &self.args.first_token_amount_min,
            &self.payments.first_payment.as_ref(),
        ) && self.min_leq_payment_amount(
            &self.args.second_token_amount_min,
            &self.payments.second_payment.as_ref(),
        )
    }

    fn min_leq_payment_amount(
        &self,
        min: &BigUint<M>,
        payment_opt: &Option<&EsdtTokenPayment<M>>,
    ) -> bool {
        match payment_opt {
            Some(payment) => min <= &payment.amount,
            None => false,
        }
    }
}

impl<M: ManagedTypeApi> AddLiquidityContext<M> {
    pub fn payment_tokens_match_pool_tokens(&self) -> bool {
        self.payment_token_match_pool_token(
            &self.first_token_id,
            &self.tx_input.payments.first_payment.as_ref(),
        ) && self.payment_token_match_pool_token(
            &self.second_token_id,
            &self.tx_input.payments.second_payment.as_ref(),
        )
    }

    fn payment_token_match_pool_token(
        &self,
        token_id: &TokenIdentifier<M>,
        payment_opt: &Option<&EsdtTokenPayment<M>>,
    ) -> bool {
        match payment_opt {
            Some(payment) => token_id == &payment.token_identifier,
            None => false,
        }
    }

    pub fn get_first_payment(&self) -> &EsdtTokenPayment<M> {
        self.tx_input.payments.first_payment.as_ref().unwrap()
    }

    pub fn get_second_payment(&self) -> &EsdtTokenPayment<M> {
        self.tx_input.payments.second_payment.as_ref().unwrap()
    }

    pub fn set_liquidity_added(&mut self, amount: BigUint<M>) {
        self.liquidity_added = amount;
    }

    pub fn get_liquidity_added(&self) -> &BigUint<M> {
        &self.liquidity_added
    }

    pub fn increase_lp_token_supply(&mut self, amount: &BigUint<M>) {
        self.lp_token_supply += amount;
    }

    pub fn increase_reserves(&mut self) {
        self.first_token_reserve += &self.first_token_optimal;
        self.second_token_reserve += &self.second_token_optimal;
    }

    pub fn set_first_amount_optimal(&mut self, amount: BigUint<M>) {
        self.first_token_optimal = amount;
    }

    pub fn get_first_amount_optimal(&self) -> &BigUint<M> {
        &self.first_token_optimal
    }

    pub fn set_second_amount_optimal(&mut self, amount: BigUint<M>) {
        self.second_token_optimal = amount;
    }

    pub fn get_second_amount_optimal(&self) -> &BigUint<M> {
        &self.second_token_optimal
    }

    pub fn get_first_token_amount_min(&self) -> &BigUint<M> {
        &self.tx_input.args.first_token_amount_min
    }

    pub fn get_second_token_amount_min(&self) -> &BigUint<M> {
        &self.tx_input.args.second_token_amount_min
    }

    pub fn set_output_payments(&mut self, payments: ManagedVec<M, EsdtTokenPayment<M>>) {
        self.output_payments = payments
    }

    pub fn get_output_payments(&self) -> &ManagedVec<M, EsdtTokenPayment<M>> {
        &self.output_payments
    }

    pub fn get_opt_accept_funds_func(&self) -> &OptionalArg<ManagedBuffer<M>> {
        &self.tx_input.args.opt_accept_funds_func
    }
}