# tech
# Built with Seahorse v0.2.7

from seahorse.prelude import *

declare_id('B9kViFpeL3ShhoQkfCX8LKZsisZuPhCiabcpEG3nXPvV')

class HopDong(Account):
    owner: Pubkey # 32 bytes
    student: Pubkey # 32 bytes
    is_done: bool # 8 bytes
    price: u64 # 8 bytes
    message_u16_128_array: Array[u16, 128]

@instruction
def init_hopdong(
    # Account
    payer: Signer,
    owner: Signer,
    student: Signer,
    hopdong: Empty[HopDong],
    # Data
    seed_sha256: u128,
    price: u64,
    message_u16_128_array: Array[u16, 128]
):
    hopdong = hopdong.init(payer = payer, seeds = [owner, "hopdong", seed_sha256])
    hopdong.student = student.key()
    hopdong.price = price
    hopdong.message_u16_128_array = message_u16_128_array

    # Check if user have enough money
    # Transfer money to owner
    student.transfer_lamports(owner, hopdong.price)

@instruction
def xacnhan_done(
    # Account
    payer: Signer,
    owner: Signer,
    student: Signer,
    hopdong: HopDong
):
    assert owner.key() == hopdong.owner, "Hopdong owner not same!"
    assert student.key() == hopdong.student, "Hopdong student not same!"
    assert hopdong.is_done == False, "Hopdong already done!"

    hopdong.is_done = True

    # Check if user have enough money
    # Transfer money to owner
    owner.transfer_lamports(student, hopdong.price)