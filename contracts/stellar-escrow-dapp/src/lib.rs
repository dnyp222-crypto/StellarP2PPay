#![no_std]
use soroban_sdk::{contract, contractimpl, token, Address, Env};

#[contract]
pub struct StellarP2PPay;

#[contractimpl]
impl StellarP2PPay {
    /// Hàm thực hiện chuyển USDC (hoặc bất kỳ Token nào) từ người gửi sang người nhận
    /// - `env`: Môi trường thực thi của Soroban
    /// - `token_id`: Địa chỉ Contract của Token trên mạng Stellar (ví dụ: Ví/Contract USDC)
    /// - `from`: Địa chỉ ví của người gửi tiền
    /// - `to`: Địa chỉ ví của người nhận tiền
    /// - `amount`: Số lượng token muốn chuyển (định dạng i128 bao gồm cả phần thập phân)
    pub fn transfer_usdc(
        env: Env,
        token_id: Address,
        from: Address,
        to: Address,
        amount: i128,
    ) {
        // 1. Xác thực quyền sở hữu: Đảm bảo giao dịch này được chính ví `from` ký duyệt
        from.require_auth();

        // 2. Khởi tạo client kết nối với Smart Contract của Token (USDC)
        let token_client = token::Client::new(&env, &token_id);

        // 3. Gọi hàm transfer từ Token Interface để chuyển tiền trực tiếp trên On-chain
        token_client.transfer(&from, &to, &amount);
    }
}

mod test;