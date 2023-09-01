import { RedspotUserConfig } from 'redspot/types'
import '@redspot/patract'
import '@redspot/chai'
import '@redspot/gas-reporter'
const types = {
  ContractsPsp34Id: {
    _enum: {
      U8: 'u8',
      U16: 'u16',
      U32: 'u32',
      U64: 'u64',
      U128: 'u128',
      Bytes: 'Vec<u8>'
    }
  },
  ContractsDiamondFacetCut: {
    hash: '[u8; 32]',
    selectors: 'Vec<[u8; 4]>'
  }
}

export default {
  defaultNetwork: 'development',
  contract: {
    ink: {
      toolchain: 'nightly',
      sources: ['project/contracts/**']
    }
  },
  networks: {
    development: {
      endpoint: 'ws://127.0.0.1:9944',
      types,
      gasLimit: '400000000000',
      explorerUrl: 'https://polkadot.js.org/apps/#/explorer/query/?rpc=ws://127.0.0.1:9944/'
    },
    substrate: {
      endpoint: 'ws://127.0.0.1:9944',
      gasLimit: '400000000000',
      accounts: ['//Alice'],
      types
    }
  },
  mocha: {
    timeout: 60000
  }
} as RedspotUserConfig
