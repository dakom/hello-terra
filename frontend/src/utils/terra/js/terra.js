const {LCDClient, Coin, MnemonicKey, isTxError, MsgStoreCode} = window.Terra;
// these imports break rollup!

//import { LCDClient, Coin, MnemonicKey} from '@terra-money/terra.js';
/*
import {
  ConnectType,
  CreateTxFailed,
  NetworkInfo,
  Timeout,
  TxFailed,
  TxResult,
  TxUnspecifiedError,
  UserDenied,
  WalletController,
  WalletStatus,
} from '@terra-money/wallet-provider';
*/

let terra;
let wallet;

export function _init(url, chain_id) {
  terra = new LCDClient({
    URL: url, 
    chainID: chain_id 
  });


}

export function _login(mnemonic) {
  const key = new MnemonicKey({
    mnemonic
  });

  wallet = terra.wallet(key);

  return(wallet.key.accAddress);
}

export function _upload_contract(bytes) {
  const storeCode = new MsgStoreCode(wallet.key.accAddress, bytes);
  return wallet.createAndSignTx({ msgs: [storeCode], })
    .then(storeCodeTx => terra.tx.broadcast(storeCodeTx))
    .then(storeCodeTxResult => {
        if (isTxError(storeCodeTxResult)) {
          return Promise.reject(`store code failed. code: ${storeCodeTxResult.code}, codespace: ${storeCodeTxResult.codespace}, raw_log: ${storeCodeTxResult.raw_log}`);
        }

        const {
          store_code: { code_id },
        } = storeCodeTxResult.logs[0].eventsByType;

        return Promise.resolve(code_id);
    })
    .then(res => {
      console.log(res);
      return res;
    });
}