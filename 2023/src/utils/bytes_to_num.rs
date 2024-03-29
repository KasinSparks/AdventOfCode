#[derive(Debug)]
pub struct BytesToNumErr {
    msg: String,
}

pub fn bytes_to_num(nums: &[u8]) -> Result<usize, BytesToNumErr> {
    let l = nums.len();
    let mut result = 0;

    for i in (0..l).rev() {
        if !nums[i].is_ascii_digit() {
            return Err(BytesToNumErr { msg: String::from("Non-ascii digit.") });
        }

        result += usize::pow(10, i as u32) * (nums[(l - 1) - i] - '0' as u8) as usize;
    }

    return Ok(result);
}