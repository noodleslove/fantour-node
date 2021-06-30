#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{debug, decl_module, dispatch::DispatchResult};
use frame_system::ensure_signed;
use sp_runtime::print;

pub trait Config: frame_system::Config {}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {

        /// A function that says hello to the user by printing messages to the node log
        #[weight = 10_000]
        pub fn say_hello(origin) -> DispatchResult {
            // Ensure that the caller is a regular keypair account
            let caller = ensure_signed(origin)?;

            // Print a message
            print("Hello World");
            // Inspecting variables
            debug::info!("Request sent by: {:?}", caller);

            // Indicate that this call succeeded
            Ok(())
        }

        // More dispatchable calls could go here
    }
}
