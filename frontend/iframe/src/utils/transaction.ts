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

//we need to poll at a regular interval, but also timeout at some point
//see discussion at https://github.com/terra-money/wallet-provider/issues/23#issuecomment-918725271
//The validator allows for async verification (failure there will reject properly, not case a re-poll)
export function requestTxInfo({timeout, pollWait, hash, wallet, validator}:TxInfoRequest):Promise<any> {
    return new Promise((resolve, reject) => {
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

        let stopProcessing = false;
        let pollId:NodeJS.Timeout;

        function clearTimeouts() {
            stopProcessing = true;
            clearTimeout(pollId);
            clearTimeout(timeoutId);
        }

        let timeoutId = setTimeout(() => {
            clearTimeouts();
            reject("timeout");
        }, TIMEOUT);


        function poll() {
            if(!stopProcessing) {
                wallet.lcd.tx.txInfo(hash)
                    .then(res => {
                        return VALIDATOR(res)
                            .then(data => ({valid: true, data}))
                            .catch(reason => Promise.resolve({valid: false, data: reason}))
                    })
                    .then(
                        ({valid, data}) => {
                            clearTimeouts();
                            if(!valid) {
                                // validation failed - don't poll again, fully reject
                                reject(data);
                            } else {
                                // validation succeeded - resolve
                                resolve(data);
                            }
                        },
                        // error in tx itself, i.e. 404, try again!
                        (_err) => {
                            pollId = setTimeout(() => {
                                poll();
                            }, POLL_WAIT);
                        }
                    )
            }
        } 

        poll();
    })
}