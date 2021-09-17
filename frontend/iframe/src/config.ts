import {NetworkInfo} from "@terra-money/wallet-provider";

// Defaults for wallet when it's not provided
// not sure if these are actually used, ultimately, but the API seems to require it
export const mainnet = {
  name: 'mainnet',
  chainID: 'columbus-4',
  lcd: 'https://lcd.terra.dev',
};

export const testnet = {
  name: 'testnet',
  chainID: 'tequila-0004',
  lcd: 'https://tequila-lcd.terra.dev',
};

export const walletConnectChainIds: Record<number, NetworkInfo> = {
  0: testnet,
  1: mainnet,
};

// default amount of time to wait before giving up on transaction finishing
export const TRANSACTION_INFO_TIMEOUT:number = 1000 * 60 * 5; // 5 mins

// default amount of time to wait before polling a transaction hash
export const TRANSACTION_INFO_POLL_WAIT:number = 500; //500ms

// to early-exit messages not meant for us
export const TAG = "WALLET_BRIDGE";