use crate::prompt_user;

pub fn prompt_for_repeat() -> bool {
    let response = prompt_user("Would you like to generate another QR code? (y/n):");
    response.trim().to_lowercase() == "y"
}