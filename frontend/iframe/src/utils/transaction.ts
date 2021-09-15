import { WalletState } from "../wallet";
import {TRANSACTION_INFO_POLL_WAIT, TRANSACTION_INFO_TIMEOUT} from "../config";

interface TxInfoRequest {
    timeout?: number,
    pollWait?: number,
    //first arg is if valid, second arg is the data to pass back
    validator?: (res:any) => Promise<any>, 
    hash: string,
    wallet: WalletState
}


//https://github.com/terra-money/wallet-provider/issues/23#issuecomment-918725271
//The first promise to txInfo() will fail on 404 and cause a re-poll
//The validator allows for async verification, but rejection there will *not* cause a re-polling
export function requestTxInfo({timeout, pollWait, hash, wallet, validator}:TxInfoRequest):Promise<any> {
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

    const goodPromise:Promise<any> = new Promise<{valid: boolean, data: any}>(resolve => {
        const poll = () => {
            setTimeout(() => {
                wallet.lcd.tx.txInfo(hash)
                    .then(res => {
                        return VALIDATOR(res)
                            .then(data => ({valid: true, data}))
                            .catch(reason => Promise.resolve({valid: false, data: reason}))
                    })
                    .then(resolve)
                    .catch(_res => {
                        poll();
                    });
            }, POLL_WAIT);
        }

        poll();
    })
    .then(({valid, data}) => {
        if(valid) {
            return Promise.resolve(data);
        } else {
            return Promise.reject(data);
        }
    });

    return Promise.race([badPromise, goodPromise]);
}