import { WalletState } from "types";
import {TRANSACTION_INFO_POLL_WAIT, TRANSACTION_INFO_TIMEOUT} from "../config";

interface TxInfoRequest {
    timeout?: number,
    pollWait?: number,
    //first arg is if valid, second arg is the data to pass back
    validator?: (res:any) => Promise<any>, 
    hash: string,
    walletState: WalletState
}


//https://github.com/terra-money/wallet-provider/issues/23#issuecomment-918725271
export function requestTxInfo({timeout, pollWait, hash, walletState, validator}:TxInfoRequest):Promise<any> {
    const DEFAULT_VALIDATOR = (res:any) => {
        if(res == null) {
            return Promise.reject("no result!");
        } else {
            return Promise.resolve(res); 
        }
    }

    const TIMEOUT = timeout == null ? TRANSACTION_INFO_TIMEOUT : timeout;
    const POLL_WAIT = pollWait == null ? TRANSACTION_INFO_POLL_WAIT : pollWait;
    const VALIDATOR = validator == null ? DEFAULT_VALIDATOR : validator;

    const badPromise:Promise<string> = new Promise((resolve, reject) => {
        setTimeout(() => reject("timeout!"), TIMEOUT);
    });

    const goodPromise:Promise<any> = new Promise(resolve => {
        const poll = () => {
            setTimeout(() => {
                walletState.lcd.tx.txInfo(hash)
                    .then(VALIDATOR)
                    .then(resolve)
                    .catch(_res => {
                        poll();
                    });
            }, POLL_WAIT);
        }

        poll();
    });

    return Promise.race([badPromise, goodPromise]);
}