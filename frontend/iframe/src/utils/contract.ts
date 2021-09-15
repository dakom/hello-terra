import { WalletState } from "types";
import {StdFee, MsgStoreCode, MsgInstantiateContract, MsgExecuteContract} from "@terra-money/terra.js";
import {Wallet} from "@terra-money/wallet-provider";
import {requestTxInfo} from "./transaction";

export function contractUpload(walletState:WalletState, msg:MsgStoreCode):Promise<number> {

    return walletState.wallet.post({ msgs: [msg] })
        .then(res => {
            if(!res.success) {
                return Promise.reject(res);
            } else {
                return requestTxInfo({
                    walletState,
                    hash: res.result.txhash,
                    validator: (res:any) => {
                        if(res.logs?.length > 0) {
                            const code_ids = res.logs[0].eventsByType?.store_code?.code_id;
                            if(code_ids?.length > 0) {
                                return Promise.resolve(parseInt(code_ids[0]));
                            }
                        }
                        return Promise.reject("bad code id!");
                    }
                });
            }
        })
}

export function contractInstantiate(walletState:WalletState, msg:MsgInstantiateContract):Promise<string> {
    return walletState.wallet.post({ msgs: [msg] })
        .then(res => {
            if(!res.success) {
                return Promise.reject(res);
            } else {
                return requestTxInfo({
                    walletState,
                    hash: res.result.txhash,
                    validator: (res:any) => {
                        if(res.logs?.length > 0) {
                            const addrs = res.logs[0].eventsByType?.instantiate_contract?.contract_address;
                            if(addrs?.length > 0) {
                                return Promise.resolve(addrs[0]);
                            }
                        }
                        return Promise.reject("bad contract addr!");
                    }
                });
            }
        })
}
export function contractExecute(walletState:WalletState, msg:MsgExecuteContract):Promise<string> {
    return walletState.wallet.post({ 
        //fee: new StdFee(10000, '20000uusd'),
        msgs: [msg] 
    })
        .then(res => {
            console.log(res);
            if(!res.success) {
                return Promise.reject(res);
            } else {
                return requestTxInfo({
                    walletState,
                    hash: res.result.txhash,
                    validator: (res:any) => {
                        console.log(res);

                        /*
                        if(res.logs?.length > 0) {
                            const addrs = res.logs[0].eventsByType?.instantiate_contract?.contract_address;
                            if(addrs?.length > 0) {
                                return Promise.resolve(addrs[0]);
                            }
                        }
                        */
                        return Promise.reject("bad contract addr!");
                    }
                });
            }
        })
}