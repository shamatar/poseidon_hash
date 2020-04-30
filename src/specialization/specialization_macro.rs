#[macro_export]
macro_rules! construct_sponge {
	( $(#[$attr:meta])* $visibility:vis struct $name:ident ( $n_rate:tt, $n_capacity: tt ); ) => {
		/// Little-endian large integer type
		$(#[$attr])*
        $visibility struct $name<'a, E: PoseidonEngine> 
        {
            params: &'a E::Params,
            internal_state: [E::Fr; $n_rate + $n_capacity],
            mode: OpMode<E>
        }

        #[derive(Clone)]
        enum OpMode<E: PoseidonEngine> {
            AccumulatingToAbsorb(usize, [E::Fr; $n_rate]),
            SqueezedInto(usize, [E::Fr; $n_rate])
        }

        impl<E: PoseidonEngine> Copy for OpMode<E> {}

        impl<'a, E: PoseidonEngine> Clone for $name<'a, E> {
            fn clone(&self) -> Self {
                Self {
                    params: self.params,
                    internal_state: self.internal_state,
                    mode: self.mode
                }
            }
        }

        impl<'a, E: PoseidonEngine> $name<'a, E> {
            pub fn new(
                params: &'a E::Params
            ) -> Self {
                assert!(params.rate() == $n_rate, "rate is invalid for specialization");
                assert!(params.capacity() == $n_capacity, "capacity is invalid for specialization");
                
                let op = OpMode::AccumulatingToAbsorb(0, [E::Fr::zero(); $n_rate]);

                Self {
                    params,
                    internal_state: [E::Fr::zero(); $n_rate + $n_capacity],
                    mode: op
                }
            }

            fn mimc(
                params: &E::Params,
                internal_state: [E::Fr; $n_rate + $n_capacity],
            ) -> [E::Fr; $n_rate + $n_capacity] {
                let mut state = internal_state;
                debug_assert!(params.num_full_rounds() % 2 == 0);
                let half_of_full_rounds = params.num_full_rounds() / 2;
                let mut mds_application_scratch = [E::Fr::zero(); $n_rate + $n_capacity];
                debug_assert_eq!(state.len(), params.state_width() as usize);

                const LAST_ELEM_IDX: usize = $n_rate + $n_capacity - 1;

                // full rounds
                for round in 0..half_of_full_rounds {
                    let round_constants = params.round_constants(round);
                
                    // add round constatnts
                    for (s, c)  in state.iter_mut()
                                .zip(round_constants.iter()) {
                        s.add_assign(c);
                    }

                    params.sbox().apply(&mut state[..]);

                    // mul state by MDS
                    for (row, place_into) in mds_application_scratch.iter_mut()
                                                    .enumerate() {
                        let tmp = scalar_product::<E>(& state[..], params.mds_matrix_row(row as u32));                           
                        *place_into = tmp;
                    }

                    state = mds_application_scratch;
                }

                // partial rounds

                for round in half_of_full_rounds..(params.num_partial_rounds() + half_of_full_rounds){
                    let round_constants = params.round_constants(round);
                
                    // add round constatnts
                    for (s, c)  in state.iter_mut()
                                .zip(round_constants.iter()) {
                        s.add_assign(c);
                    }

                    params.sbox().apply(&mut state[LAST_ELEM_IDX..]);

                    // mul state by MDS
                    for (row, place_into) in mds_application_scratch.iter_mut()
                                                    .enumerate() {
                        let tmp = scalar_product::<E>(& state[..], params.mds_matrix_row(row as u32));
                        *place_into = tmp;                               
                    }

                    state = mds_application_scratch;
                }

                // full rounds
                for round in (params.num_partial_rounds() + half_of_full_rounds)..(params.num_partial_rounds() + params.num_full_rounds()) {
                    let round_constants = params.round_constants(round);
                
                    // add round constatnts
                    for (s, c)  in state.iter_mut()
                                .zip(round_constants.iter()) {
                        s.add_assign(c);
                    }

                    params.sbox().apply(&mut state[..]);

                    // mul state by MDS
                    for (row, place_into) in mds_application_scratch.iter_mut()
                                                    .enumerate() {
                        let tmp = scalar_product::<E>(& state[..], params.mds_matrix_row(row as u32));                           
                        *place_into = tmp;
                    }

                    state = mds_application_scratch;
                }

                state
            }

            pub fn absorb_single_value(
                &mut self,
                value: E::Fr
            ) {
                match self.mode {
                    OpMode::AccumulatingToAbsorb(ref mut len, ref mut into) => {
                        // two cases
                        // either we have accumulated enough already and should to 
                        // a mimc round before accumulating more, or just accumulate more
                        if *len < $n_rate {
                            into[*len] = value;
                            *len += 1;
                        } else {
                            for i in 0..$n_rate {
                                self.internal_state[i].add_assign(&into[i]);
                            }

                            *len = 0;

                            self.internal_state = Self::mimc(&*self.params, self.internal_state);
                        }
                    },
                    OpMode::SqueezedInto(_, _) => {
                        // we don't need anything from the output, so it's dropped

                        let mut s = [E::Fr::zero(); $n_rate];
                        s[0] = value;

                        let op = OpMode::AccumulatingToAbsorb(1, s);
                        self.mode = op;
                    }
                }
            }

            // pub fn absorb(
            //     &mut self,
            //     input: &[E::Fr]
            // ) {
            //     let rate = self.params.rate() as usize;
            //     let mut absorbtion_cycles = input.len() / rate;
            //     if input.len() % rate != 0 {
            //         absorbtion_cycles += 1;
            //     }
            //     let padding_len = absorbtion_cycles * rate - input.len();
            //     let padding = vec![E::Fr::one(); padding_len];

            //     let it = input.iter().chain(&padding);

            //     for &val in it {
            //         self.absorb_single_value(val);
            //     }
            // }

            pub fn squeeze_out_single(
                &mut self,
            ) -> E::Fr {
                match self.mode {
                    OpMode::AccumulatingToAbsorb(len, ref mut into) => {
                        if len < $n_rate {
                            for i in len..$n_rate {
                                debug_assert!(into[i].is_zero());
                            }
                        }

                        // two cases
                        // either we have accumulated enough already and should to 
                        // a mimc round before accumulating more, or just accumulate more
                        for i in 0..len {
                            self.internal_state[i].add_assign(&into[i]);
                        }

                        self.internal_state = Self::mimc(&*self.params, self.internal_state);

                        // we don't take full internal state, but only the rate
                        let mut sponge_output = [E::Fr::zero(); $n_rate];
                        sponge_output.copy_from_slice(&self.internal_state[0..$n_rate]);

                        let output = sponge_output[0];

                        let op = OpMode::SqueezedInto(1, sponge_output);
                        self.mode = op;

                        return output;
                    },
                    OpMode::SqueezedInto(ref mut len, ref mut into) => {
                        if *len == $n_rate {
                            self.internal_state = Self::mimc(&*self.params, self.internal_state);

                            let mut sponge_output = [E::Fr::zero(); $n_rate];
                            sponge_output.copy_from_slice(&self.internal_state[0..$n_rate]);

                            let output = sponge_output[0];

                            let op = OpMode::SqueezedInto(1, sponge_output);
                            self.mode = op;

                            return output;
                        }

                        let output = into[*len];
                        *len += 1;

                        return output;
                    }
                }
            }
        }
    }
}

use crate::pairing::ff::{Field};
use crate::{scalar_product, PoseidonEngine, PoseidonHashParams, SBox};

construct_sponge! {
	pub struct PosendonR2C1(2, 1);
}