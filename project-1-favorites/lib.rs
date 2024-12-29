use anchor_lang::prelude::*;

// Program's unique identifier, matches the key in the target/deploy directory
declare_id!("7awdvNfpaAt8JiCSzmge4KZniG8MaPw3cQNf1FZRkJvF");

// Size reserved for the Anchor discriminator (8 bytes)
pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

// Main Solana program module
#[program]
pub mod favorites {
    use super::*;

    /// Instruction handler to set the user's favorite number, color, and hobbies
    ///
    /// # Parameters
    /// - `context`: Context object that provides the accounts being accessed
    /// - `number`: The user's favorite number
    /// - `color`: The user's favorite color
    /// - `hobbies`: A list of the user's hobbies
    ///
    /// # Behavior
    /// Logs the user's details and updates the `Favorites` account with the provided information.
    pub fn set_favorites(
        context: Context<SetFavorites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        let user_public_key = context.accounts.user.key();

        // Log the user's details
        msg!("Program invoked by: {}", context.program_id);
        msg!(
            "User {}'s favorite number is {}, favorite color is {}",
            user_public_key,
            number,
            color
        );
        msg!("User's hobbies are: {:?}", hobbies);

        // Update the Favorites account with the provided data
        context.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies,
        });

        Ok(())
    }

    // TODO: Consider adding a `get_favorites` instruction handler to allow users to retrieve their stored data.
}

// Persistent storage structure for user's favorites
#[account]
#[derive(InitSpace)] // Custom derive macro to initialize the account space
pub struct Favorites {
    /// User's favorite number
    pub number: u64,

    /// User's favorite color (max length: 50 characters)
    #[max_len(50)]
    pub color: String,

    /// List of user's hobbies (max 5 hobbies, each up to 50 characters)
    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

/// Context struct for the `set_favorites` instruction
///
/// Contains the accounts that will be accessed or modified during the instruction.
#[derive(Accounts)]
pub struct SetFavorites<'info> {
    /// User account that signs the transaction
    #[account(mut)]
    pub user: Signer<'info>,

    /// Favorites account to store user data
    /// - Initializes the account if it does not exist
    /// - Paid for by the user
    /// - Space is calculated based on the discriminator and `Favorites` struct size
    #[account(
        init_if_needed, 
        payer = user, 
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE, 
        seeds = [b"favorites", user.key().as_ref()],
        bump
    )]
    pub favorites: Account<'info, Favorites>,

    /// System program required for account initialization
    pub system_program: Program<'info, System>,
}
