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
                        //TODO - get binary! https://github.com/terra-money/terra.js/issues/133
                        if(res.logs?.length > 0) {
                            const data = res.logs[0].eventsByType.from_contract?.data;
                            if(data) {
                                if(Array.isArray(data)) {
                                     if(data.length > 0) {
                                         return Promise.resolve(data[0]);
                                     } else {
                                         return Promise.reject("empty data array!");
                                     }
                                }
                                return Promise.resolve(data);
                            }
                        }
                        return Promise.reject("bad contract addr!");
                    }
                });
            }
        })
}

export function contractQuery(wallet:WalletState, addr:string, query:any):Promise<any> {
    console.log(query);

    return wallet.lcd.wasm.contractQuery(addr, query)
}