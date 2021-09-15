import {LCDClient, Wallet as ManualWallet, CreateTxOptions} from "@terra-money/terra.js";
import {Wallet as AutoWallet, TxResult} from "@terra-money/wallet-provider";

export enum WalletKind {
  Auto,
  Manual
}
export class WalletState {
    constructor(
        private wallet: AutoWallet | ManualWallet, 
        public lcd: LCDClient,
        private walletKind: WalletKind, 
    ) {

    }

    public post(tx: CreateTxOptions):Promise<TxResult> {
        if(this.walletKind === WalletKind.Manual) {
            return(this.withManual(wallet => {
                return wallet
                    .createAndSignTx(tx)
                    .then(tx => wallet.lcd.tx.broadcastSync(tx))
                    .then(res => {
                        if(!res) {
                            return {
                                success: false,
                                ...tx
                            } as TxResult
                        } else {
                            return {
                                result: res,
                                success: true,
                                ...tx
                            }
                        }
                    });
            }))
        } else { 
            return(this.withAuto(wallet => wallet.post(tx)));
        } 
    }
    withAuto<A>(f: (wallet:AutoWallet) => A) {
        return(f(this.wallet as AutoWallet));
    }
    withManual<A>(f: (wallet:ManualWallet) => A) {
        return(f(this.wallet as ManualWallet));
    }

    public get addr():string {
        if(this.walletKind === WalletKind.Manual) {
            return(this.withManual(wallet => wallet.key.accAddress));
        } else { 
            return(this.withAuto(wallet => wallet.wallets[0].terraAddress));
        } 
    }
}