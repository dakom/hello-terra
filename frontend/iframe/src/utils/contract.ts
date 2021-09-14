import { WalletState } from "types";
import {MsgStoreCode} from "@terra-money/terra.js";
import {requestTxInfo} from "./transaction";

export function contractUpload(walletState:WalletState, bytes:string):Promise<string> {
    const {lcd, addr, wallet} = walletState;
    const storeCode = new MsgStoreCode(addr, bytes);

    return wallet.post({ msgs: [storeCode] })
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
                                return Promise.resolve(code_ids[0]);
                            }
                        }
                        return Promise.reject("bad code id!");
                    }
                });
            }
        })
}