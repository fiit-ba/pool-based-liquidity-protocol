import {expect, setupContract, fromSigner} from './helpers'
import BN from 'bn.js'

describe('Liquidity_pool_protocol', () => {
  async function setup() {
    // setup all contracts needed
    let stablecoin1 = await setupContract('stablecoin_contract', 'new', `StableCoin1`, `SC1`)
    let stablecoin2 = await setupContract('stablecoin_contract', 'new', `StableCoin2`, `SC2`)
    let btoken = await (await setupContract('btoken_contract', 'new', '', '')).abi
    let loan = await (await setupContract('loan_contract', 'new')).abi
    let lending_pool_manager = await setupContract('liquidity_pool_manager_contract', 'new', loan.source.hash, btoken.source.hash)
    return {lending_pool_manager, stablecoin1, alice: stablecoin1.defaultSigner, stablecoin2, bob: stablecoin2.defaultSigner, charlie: lending_pool_manager.defaultSigner, dave: loan.defaultSigner, eve: btoken.defaultSigner}
  }

  it('Lend - lend asset successfully', async () => {
    const {lending_pool_manager, stablecoin1, alice, stablecoin2, bob, charlie, dave, eve} = await setup()
    // amount Alice wants to lend into the lending pool
    const lend_amount = 10000;
    // Alice approves lend amount for lending pool contract
    await expect(fromSigner(stablecoin1.contract, alice.address).tx.approve(lending_pool_manager.contract.address, lend_amount)).to.eventually.be.fulfilled
    // get Alice initial stablecoin1 balance
    let alice_initial_balance = (await stablecoin1.query.balanceOf(alice.address)).output;
    // Alice balance should be greater equal to lend amount
    expect(alice_initial_balance).to.gte(lend_amount);
    // allow new asset (stablecoin1) for lending into the pool
    await expect(fromSigner(lending_pool_manager.contract, charlie.address).tx.setAssetAllowance(stablecoin1.contract.address)).to.eventually.be.fulfilled
    // get initial stablecoin1 balance of the pool
    let pool_initial_balance = (await stablecoin1.query.balanceOf(lending_pool_manager.contract.address)).output;
    // Alice lends stablecoin1 to the lending pool
    await expect(fromSigner(lending_pool_manager.contract, alice.address).tx.lend(stablecoin1.contract.address, lend_amount)).to.eventually.be.fulfilled
    // check for Alice new stablecoin1 balance
    await expect(stablecoin1.query.balanceOf(alice.address)).to.have.output((alice_initial_balance.sub(new BN(lend_amount))))
    // check for pool new stablecoin1 balance
    await expect(stablecoin1.query.balanceOf(lending_pool_manager.contract.address)).to.have.output((pool_initial_balance.add(new BN(lend_amount))))
  })

  it('Lend - insufficient allowance', async () => {
    const {lending_pool_manager, stablecoin1, alice, stablecoin2, bob, charlie, dave, eve} = await setup()
    // amount Alice wants to lend into the lending pool
    const lend_amount = 10000;
    // get Alice initial stablecoin1 balance
    let alice_initial_balance = (await stablecoin1.query.balanceOf(alice.address)).output;
    // Alice balance should be greater equal to lend amount
    expect(alice_initial_balance).to.gte(lend_amount);
    // allow new asset (stablecoin1) for lending into the pool
    await expect(fromSigner(lending_pool_manager.contract, charlie.address).tx.setAssetAllowance(stablecoin1.contract.address)).to.eventually.be.fulfilled
    // Alice lends stablecoin1 to the lending pool
    await expect(fromSigner(lending_pool_manager.contract, alice.address).tx.lend(stablecoin1.contract.address, lend_amount)).to.eventually.be.rejected
  })

  it('Lend - insufficient balance', async () => {
    const {lending_pool_manager, stablecoin1, alice, stablecoin2, bob, charlie, dave, eve} = await setup()
    // amount Alice wants to lend into the lending pool
    const lend_amount = 10000;
    // Alice approves lend amount for lending pool contract for stablecoin2
    await expect(fromSigner(stablecoin2.contract, alice.address).tx.approve(lending_pool_manager.contract.address, lend_amount)).to.eventually.be.fulfilled
    // allow new asset (stablecoin1) for lending into the pool
    await expect(fromSigner(lending_pool_manager.contract, charlie.address).tx.setAssetAllowance(stablecoin2.contract.address)).to.eventually.be.fulfilled
    // Alice lends stablecoin1 to the lending pool
    await expect(fromSigner(lending_pool_manager.contract, alice.address).tx.lend(stablecoin2.contract.address, lend_amount)).to.eventually.be.rejected
  })

  it('Lend - asset not allowed', async () => {
    const {lending_pool_manager, stablecoin1, alice, stablecoin2, bob, charlie, dave, eve} = await setup()
    // amount Alice wants to lend into the lending pool
    const lend_amount = 10000;
    // Alice approves lend amount for lending pool contract
    await expect(fromSigner(stablecoin1.contract, alice.address).tx.approve(lending_pool_manager.contract.address, lend_amount)).to.eventually.be.fulfilled
    // get Alice initial stablecoin1 balance
    let alice_initial_balance = (await stablecoin1.query.balanceOf(alice.address)).output;
    // Alice balance should be greater equal to lend amount
    expect(alice_initial_balance).to.gte(lend_amount);
    // Alice lends stablecoin1 to the lending pool
    await expect(fromSigner(lending_pool_manager.contract, alice.address).tx.lend(stablecoin1.contract.address, lend_amount)).to.eventually.be.rejected
  })

  it('Borrow - borrow asset successfully', async () => {
    const {lending_pool_manager, stablecoin1, alice, stablecoin2, bob, charlie, dave, eve} = await setup()
    // amount Alice wants to lend into the lending pool
    const lend_amount = 10000;
    // Alice approves lend amount for lending pool contract
    await expect(fromSigner(stablecoin1.contract, alice.address).tx.approve(lending_pool_manager.contract.address, lend_amount)).to.eventually.be.fulfilled
    // allow new asset (stablecoin1) for lending into the pool
    await expect(fromSigner(lending_pool_manager.contract, charlie.address).tx.setAssetAllowance(stablecoin1.contract.address)).to.eventually.be.fulfilled
    // Alice lends stablecoin1 to the lending pool
    await expect(fromSigner(lending_pool_manager.contract, alice.address).tx.lend(stablecoin1.contract.address, lend_amount)).to.eventually.be.fulfilled

    // amount Bob wants to borrow from the lending pool
    const borrow_amount = 5000;
    // get Bob initial stablecoin1 balance
    let bob_initial_asset1 = (await stablecoin1.query.balanceOf(bob.address)).output;
    // get Bob initial stablecoin2 balance
    let bob_initial_asset2 = (await stablecoin2.query.balanceOf(bob.address)).output;
    // allow new asset (stablecoin2) for colltarelization
    await expect(fromSigner(lending_pool_manager.contract, charlie.address).tx.setCollateralAllowance(stablecoin2.contract.address)).to.eventually.be.fulfilled
    // Bob approves colateral amount for lending pool contract
    await expect(fromSigner(stablecoin2.contract, bob.address).tx.approve(lending_pool_manager.contract.address, borrow_amount)).to.eventually.be.fulfilled
    // set up conversion rates
    await expect(lending_pool_manager.tx.setConversionRate(stablecoin1.contract.address, stablecoin2.contract.address, 1)).to.eventually.be.fulfilled
    await expect(lending_pool_manager.tx.setConversionRate(stablecoin2.contract.address, stablecoin1.contract.address, 1)).to.eventually.be.fulfilled
    // get contract initial stablecoin1 balance (borrow asset)
    let pool_initial_asset1 = (await stablecoin1.query.balanceOf(lending_pool_manager.contract.address)).output;
    // get contract initial stablecoin2 balance (collateral)
    let pool_initial_asset2 = (await stablecoin2.query.balanceOf(lending_pool_manager.contract.address)).output;
    // Bob borrows assets with collateral
    await expect(fromSigner(lending_pool_manager.contract, bob.address).tx.borrow(stablecoin1.contract.address, stablecoin2.contract.address, borrow_amount)).to.eventually.be.fulfilled
    // check for Bob new stablecoin1 balance
    await expect(stablecoin1.query.balanceOf(bob.address)).to.have.output((bob_initial_asset1.add(new BN(3500))))
    // check for Bob new stablecoin2 balance
    await expect(stablecoin2.query.balanceOf(bob.address)).to.have.output((bob_initial_asset2.sub(new BN(5000))))
    // check for pool new stablecoin1 balance
    await expect(stablecoin1.query.balanceOf(lending_pool_manager.contract.address)).to.have.output((pool_initial_asset1.sub(new BN(3500))))
    // check for pool new stablecoin2 balance
    await expect(stablecoin2.query.balanceOf(lending_pool_manager.contract.address)).to.have.output((pool_initial_asset2.add(new BN(5000))))
  })

  it('Borrow - asset not supported', async () => {
    const {lending_pool_manager, stablecoin1, alice, stablecoin2, bob, charlie, dave, eve} = await setup()
    // amount to borrow
    let borrow_amount = 5000;
    // allow new asset (stablecoin2) for colltarelization
    await expect(fromSigner(lending_pool_manager.contract, charlie.address).tx.setCollateralAllowance(stablecoin2.contract.address)).to.eventually.be.fulfilled
    // Bob approves colateral amount for lending pool contract
    await expect(fromSigner(stablecoin2.contract, bob.address).tx.approve(lending_pool_manager.contract.address, borrow_amount)).to.eventually.be.fulfilled
    // set up conversion rates
    await expect(lending_pool_manager.tx.setConversionRate(stablecoin1.contract.address, stablecoin2.contract.address, 1)).to.eventually.be.fulfilled
    await expect(lending_pool_manager.tx.setConversionRate(stablecoin2.contract.address, stablecoin1.contract.address, 1)).to.eventually.be.fulfilled
    // Bob borrows assets with collateral
    await expect(fromSigner(lending_pool_manager.contract, bob.address).tx.borrow(stablecoin1.contract.address, stablecoin2.contract.address, borrow_amount)).to.eventually.be.rejected
  })

  it('Borrow - collateral not supported', async () => {
    const {lending_pool_manager, stablecoin1, alice, stablecoin2, bob, charlie, dave, eve} = await setup()
    // amount Alice wants to lend into the lending pool
    const lend_amount = 10000;
    // Alice approves lend amount for lending pool contract
    await expect(fromSigner(stablecoin1.contract, alice.address).tx.approve(lending_pool_manager.contract.address, lend_amount)).to.eventually.be.fulfilled
    // allow new asset (stablecoin1) for lending into the pool
    await expect(fromSigner(lending_pool_manager.contract, charlie.address).tx.setAssetAllowance(stablecoin1.contract.address)).to.eventually.be.fulfilled
    // Alice lends stablecoin1 to the lending pool
    await expect(fromSigner(lending_pool_manager.contract, alice.address).tx.lend(stablecoin1.contract.address, lend_amount)).to.eventually.be.fulfilled

    // amount Bob wants to borrow from the lending pool
    const borrow_amount = 5000;
    // Bob approves colateral amount for lending pool contract
    await expect(fromSigner(stablecoin2.contract, bob.address).tx.approve(lending_pool_manager.contract.address, borrow_amount)).to.eventually.be.fulfilled
    // set up conversion rates
    await expect(lending_pool_manager.tx.setConversionRate(stablecoin1.contract.address, stablecoin2.contract.address, 1)).to.eventually.be.fulfilled
    await expect(lending_pool_manager.tx.setConversionRate(stablecoin2.contract.address, stablecoin1.contract.address, 1)).to.eventually.be.fulfilled
    // Bob borrows assets with collateral
    await expect(fromSigner(lending_pool_manager.contract, bob.address).tx.borrow(stablecoin1.contract.address, stablecoin2.contract.address, borrow_amount)).to.eventually.be.rejected
  })

  it('Borrow - insufficient pool balance', async () => {
    const {lending_pool_manager, stablecoin1, alice, stablecoin2, bob, charlie, dave, eve} = await setup()
    // amount Alice wants to lend into the lending pool
    const lend_amount = 5000;
    // Alice approves lend amount for lending pool contract
    await expect(fromSigner(stablecoin1.contract, alice.address).tx.approve(lending_pool_manager.contract.address, lend_amount)).to.eventually.be.fulfilled
    // allow new asset (stablecoin1) for lending into the pool
    await expect(fromSigner(lending_pool_manager.contract, charlie.address).tx.setAssetAllowance(stablecoin1.contract.address)).to.eventually.be.fulfilled
    // Alice lends stablecoin1 to the lending pool
    await expect(fromSigner(lending_pool_manager.contract, alice.address).tx.lend(stablecoin1.contract.address, lend_amount)).to.eventually.be.fulfilled

    // amount Bob wants to borrow from the lending pool
    const borrow_amount = 10000;
    // allow new asset (stablecoin2) for colltarelization
    await expect(fromSigner(lending_pool_manager.contract, charlie.address).tx.setCollateralAllowance(stablecoin2.contract.address)).to.eventually.be.fulfilled
    // Bob approves colateral amount for lending pool contract
    await expect(fromSigner(stablecoin2.contract, bob.address).tx.approve(lending_pool_manager.contract.address, borrow_amount)).to.eventually.be.fulfilled
    // set up conversion rates
    await expect(lending_pool_manager.tx.setConversionRate(stablecoin1.contract.address, stablecoin2.contract.address, 1)).to.eventually.be.fulfilled
    await expect(lending_pool_manager.tx.setConversionRate(stablecoin2.contract.address, stablecoin1.contract.address, 1)).to.eventually.be.fulfilled
    // Bob borrows assets with collateral
    await expect(fromSigner(lending_pool_manager.contract, bob.address).tx.borrow(stablecoin1.contract.address, stablecoin2.contract.address, borrow_amount)).to.eventually.be.rejected
  })

  it('Borrow - insufficient collateral allowance', async () => {
    const {lending_pool_manager, stablecoin1, alice, stablecoin2, bob, charlie, dave, eve} = await setup()
    // amount Alice wants to lend into the lending pool
    const lend_amount = 10000;
    // Alice approves lend amount for lending pool contract
    await expect(fromSigner(stablecoin1.contract, alice.address).tx.approve(lending_pool_manager.contract.address, lend_amount)).to.eventually.be.fulfilled
    // allow new asset (stablecoin1) for lending into the pool
    await expect(fromSigner(lending_pool_manager.contract, charlie.address).tx.setAssetAllowance(stablecoin1.contract.address)).to.eventually.be.fulfilled
    // Alice lends stablecoin1 to the lending pool
    await expect(fromSigner(lending_pool_manager.contract, alice.address).tx.lend(stablecoin1.contract.address, lend_amount)).to.eventually.be.fulfilled

    // amount Bob wants to borrow from the lending pool
    const borrow_amount = 5000;
    // allow new asset (stablecoin2) for colltarelization
    await expect(fromSigner(lending_pool_manager.contract, charlie.address).tx.setCollateralAllowance(stablecoin2.contract.address)).to.eventually.be.fulfilled
    // set up conversion rates
    await expect(lending_pool_manager.tx.setConversionRate(stablecoin1.contract.address, stablecoin2.contract.address, 1)).to.eventually.be.fulfilled
    await expect(lending_pool_manager.tx.setConversionRate(stablecoin2.contract.address, stablecoin1.contract.address, 1)).to.eventually.be.fulfilled
    // Bob borrows assets with collateral
    await expect(fromSigner(lending_pool_manager.contract, bob.address).tx.borrow(stablecoin1.contract.address, stablecoin2.contract.address, borrow_amount)).to.eventually.be.rejected
  })

  it('Borrow - insufficient collateral balance', async () => {
    const {lending_pool_manager, stablecoin1, alice, stablecoin2, bob, charlie, dave, eve} = await setup()
    // amount Alice wants to lend into the lending pool
    const lend_amount = 10000;
    // Alice approves lend amount for lending pool contract
    await expect(fromSigner(stablecoin1.contract, alice.address).tx.approve(lending_pool_manager.contract.address, lend_amount)).to.eventually.be.fulfilled
    // allow new asset (stablecoin1) for lending into the pool
    await expect(fromSigner(lending_pool_manager.contract, charlie.address).tx.setAssetAllowance(stablecoin1.contract.address)).to.eventually.be.fulfilled
    // Alice lends stablecoin1 to the lending pool
    await expect(fromSigner(lending_pool_manager.contract, alice.address).tx.lend(stablecoin1.contract.address, lend_amount)).to.eventually.be.fulfilled

    // get Bob initial stablecoin2 balance
    let bob_initial_asset2 = (await stablecoin2.query.balanceOf(bob.address)).output;
    // amount Bob wants to borrow from the lending pool
    let borrow_amount = bob_initial_asset2.add(new BN(100));
    // allow new asset (stablecoin2) for colltarelization
    await expect(fromSigner(lending_pool_manager.contract, charlie.address).tx.setCollateralAllowance(stablecoin2.contract.address)).to.eventually.be.fulfilled
    // Bob approves colateral amount for lending pool contract
    await expect(fromSigner(stablecoin2.contract, bob.address).tx.approve(lending_pool_manager.contract.address, borrow_amount)).to.eventually.be.fulfilled
    // set up conversion rates
    await expect(lending_pool_manager.tx.setConversionRate(stablecoin1.contract.address, stablecoin2.contract.address, 1)).to.eventually.be.fulfilled
    await expect(lending_pool_manager.tx.setConversionRate(stablecoin2.contract.address, stablecoin1.contract.address, 1)).to.eventually.be.fulfilled
    // Bob borrows assets with collateral
    await expect(fromSigner(lending_pool_manager.contract, bob.address).tx.borrow(stablecoin1.contract.address, stablecoin2.contract.address, borrow_amount)).to.eventually.be.rejected
  })

  it('Borrow - conversion rates no existent', async () => {
    const {lending_pool_manager, stablecoin1, alice, stablecoin2, bob, charlie, dave, eve} = await setup()
    // amount Alice wants to lend into the lending pool
    const lend_amount = 10000;
    // Alice approves lend amount for lending pool contract
    await expect(fromSigner(stablecoin1.contract, alice.address).tx.approve(lending_pool_manager.contract.address, lend_amount)).to.eventually.be.fulfilled
    // allow new asset (stablecoin1) for lending into the pool
    await expect(fromSigner(lending_pool_manager.contract, charlie.address).tx.setAssetAllowance(stablecoin1.contract.address)).to.eventually.be.fulfilled
    // Alice lends stablecoin1 to the lending pool
    await expect(fromSigner(lending_pool_manager.contract, alice.address).tx.lend(stablecoin1.contract.address, lend_amount)).to.eventually.be.fulfilled

    // amount Bob wants to borrow from the lending pool
    const borrow_amount = 5000;
    // allow new asset (stablecoin2) for colltarelization
    await expect(fromSigner(lending_pool_manager.contract, charlie.address).tx.setCollateralAllowance(stablecoin2.contract.address)).to.eventually.be.fulfilled
    // Bob approves colateral amount for lending pool contract
    await expect(fromSigner(stablecoin2.contract, bob.address).tx.approve(lending_pool_manager.contract.address, borrow_amount)).to.eventually.be.fulfilled
    // Bob borrows assets with collateral
    await expect(fromSigner(lending_pool_manager.contract, bob.address).tx.borrow(stablecoin1.contract.address, stablecoin2.contract.address, borrow_amount)).to.eventually.be.rejected
  })
})
