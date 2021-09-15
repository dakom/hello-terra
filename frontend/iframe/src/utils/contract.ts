import { WalletState } from "../wallet";
import {MsgStoreCode, MsgInstantiateContract, MsgExecuteContract} from "@terra-money/terra.js";
import {requestTxInfo} from "./transaction";

export function contractUpload(wallet:WalletState, msg:MsgStoreCode):Promise<number> {

    return wallet.post({ msgs: [msg] })
        .then(res => {
            if(!res.success) {
                return Promise.reject(res);
            } else {
                return requestTxInfo({
                    wallet,
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
                })
            }
        })
}

export function contractInstantiate(wallet:WalletState, msg:MsgInstantiateContract):Promise<string> {
    return wallet.post({ msgs: [msg] })
        .then(res => {
            if(!res.success) {
                return Promise.reject(res);
            } else {
                return requestTxInfo({
                    wallet,
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
export function contractExecute(wallet:WalletState, msg:MsgExecuteContract):Promise<string> {
    console.log("posting", msg.toJSON());
    return wallet.post({ 
        //fee: new StdFee(10000, '20000uusd'),
        msgs: [msg] 
    })
        .then(res => {
            if(!res.success) {
                return Promise.reject(res);
            } else {
                return requestTxInfo({
                    wallet,
                    hash: res.result.txhash,
                    validator: (res:any) => {
                        if(res.logs?.length > 0) {
                            const eventsByType = res.logs[0].eventsByType;
                            if(eventsByType) {
                                console.log(eventsByType);
                                return Promise.reject("TODO: get contract response...");
                            }
                        }
                        return Promise.reject("bad contract addr!");
                    }
                });
            }
        })
}